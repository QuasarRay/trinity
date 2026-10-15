use std::{collections::BTreeSet, fs, path::Path};

use regex::Regex;
use roxmltree::Document;
use walkdir::WalkDir;

use crate::model::{DoxygenSymbol, LegacyTest, LegacyTestKind, RustTest, VerificationClass};

pub fn discover_legacy_tests(
    repo_root: &Path,
) -> Result<Vec<LegacyTest>, Box<dyn std::error::Error>> {
    let mut tests = BTreeSet::new();
    let cpp_root = repo_root.join("trinityal/tests");
    if cpp_root.exists() {
        for entry in WalkDir::new(cpp_root).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }
            if !matches!(
                entry.path().extension().and_then(|x| x.to_str()),
                Some("cpp" | "cc" | "cxx")
            ) {
                continue;
            }
            let relative = relative(repo_root, entry.path());
            let bytes = fs::read(entry.path())?;
            let text = String::from_utf8_lossy(&bytes);
            tests.extend(discover_gtest_text(&relative, &text));
        }
    }

    let python_root = repo_root.join("trinity/tests/Python/tests");
    if python_root.exists() {
        for entry in WalkDir::new(python_root).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file()
                || entry.path().extension().and_then(|x| x.to_str()) != Some("py")
            {
                continue;
            }
            let relative = relative(repo_root, entry.path());
            let bytes = fs::read(entry.path())?;
            let text = String::from_utf8_lossy(&bytes);
            tests.extend(discover_python_text(&relative, &text));
        }
    }
    Ok(tests.into_iter().collect())
}

#[must_use]
pub fn discover_gtest_text(source: &str, text: &str) -> Vec<LegacyTest> {
    let regex =
        Regex::new(r"\bTEST(?:_F|_P)?\s*\(\s*([^,]+)\s*,\s*([^)]+)\)").expect("static regex");
    let mut out = BTreeSet::new();
    for captures in regex.captures_iter(text) {
        let suite = captures[1].trim().to_owned();
        let name = captures[2].trim().to_owned();
        let offset = captures.get(0).map_or(0, |m| m.start());
        let line =
            u32::try_from(text[..offset].bytes().filter(|byte| *byte == b'\n').count() + 1).ok();
        let suggested_verification = classify_gtest(source, &suite, &name);
        out.insert(LegacyTest {
            id: format!("{source}::{suite}::{name}"),
            source: source.to_owned(),
            suite,
            name,
            line,
            kind: LegacyTestKind::CppGTest,
            suggested_verification,
        });
    }
    out.into_iter().collect()
}

#[must_use]
pub fn discover_python_text(source: &str, text: &str) -> Vec<LegacyTest> {
    let mut out = BTreeSet::new();
    let mut classes: Vec<(usize, String)> = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let indent = line.len().saturating_sub(trimmed.len());
        while classes
            .last()
            .is_some_and(|(class_indent, _)| indent <= *class_indent)
        {
            classes.pop();
        }
        if let Some(rest) = trimmed.strip_prefix("class ") {
            if let Some(name) = rest
                .split(['(', ':'])
                .next()
                .map(str::trim)
                .filter(|name| !name.is_empty())
            {
                classes.push((indent, name.to_owned()));
            }
            continue;
        }
        let Some(rest) = trimmed.strip_prefix("def ") else {
            continue;
        };
        let Some(name) = rest.split('(').next().map(str::trim) else {
            continue;
        };
        if !name.starts_with("test_") {
            continue;
        }
        let suite = classes
            .last()
            .map_or_else(|| "module".to_owned(), |(_, name)| name.clone());
        out.insert(LegacyTest {
            id: format!("{source}::{suite}::{name}"),
            source: source.to_owned(),
            suite,
            name: name.to_owned(),
            line: u32::try_from(index + 1).ok(),
            kind: LegacyTestKind::Python,
            suggested_verification: VerificationClass::Kani,
        });
    }
    out.into_iter().collect()
}

fn classify_gtest(source: &str, suite: &str, name: &str) -> VerificationClass {
    let identity = format!("{source}::{suite}::{name}").to_ascii_lowercase();
    if source.ends_with("ALResultTest.cpp")
        || source.ends_with("BitmapDimensions.cpp")
        || source.ends_with("VertexDefinition.cpp")
        || source.ends_with("TextureSubresource.cpp")
    {
        return VerificationClass::Kani;
    }
    if identity.contains("screenshot") {
        return VerificationClass::Screenshot;
    }
    if [
        "render",
        "swapchain",
        "raytracing",
        "compute",
        "gpu",
        "occlusion",
        "fence",
        "texture",
        "buffer",
        "shader",
    ]
    .iter()
    .any(|needle| identity.contains(needle))
    {
        return VerificationClass::RenderDevice;
    }
    VerificationClass::Integration
}

pub fn discover_rust_tests(
    workspace_root: &Path,
) -> Result<Vec<RustTest>, Box<dyn std::error::Error>> {
    let mut tests = BTreeSet::new();
    for entry in WalkDir::new(workspace_root.join("crates"))
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file()
            || entry.path().extension().and_then(|x| x.to_str()) != Some("rs")
        {
            continue;
        }
        let normalized = entry.path().to_string_lossy().replace('\\', "/");
        if !normalized.contains("/tests/") || normalized.ends_with("_kani_generated.rs") {
            continue;
        }
        let relative = relative(workspace_root, entry.path());
        let crate_name = relative.split('/').nth(1).unwrap_or("unknown").to_owned();
        let text = fs::read_to_string(entry.path())?;
        let file = syn::parse_file(&text)?;
        collect_rust_tests(&crate_name, &relative, &file.items, &mut tests)?;
    }
    Ok(tests.into_iter().collect())
}

fn collect_rust_tests(
    crate_name: &str,
    source: &str,
    items: &[syn::Item],
    out: &mut BTreeSet<RustTest>,
) -> Result<(), Box<dyn std::error::Error>> {
    for item in items {
        if let syn::Item::Fn(function) = item {
            let Some(attr) = function
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("trinity_test"))
            else {
                continue;
            };
            let mut legacy = None;
            let mut verification = VerificationClass::Kani;
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("legacy") {
                    legacy = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                } else if meta.path.is_ident("verification") {
                    let value = meta.value()?.parse::<syn::LitStr>()?.value();
                    verification = match value.as_str() {
                        "kani" => VerificationClass::Kani,
                        "native" => VerificationClass::Native,
                        "render-device" => VerificationClass::RenderDevice,
                        "screenshot" => VerificationClass::Screenshot,
                        "integration" => VerificationClass::Integration,
                        "unmapped" => VerificationClass::Unmapped,
                        _ => VerificationClass::Unmapped,
                    };
                }
                Ok(())
            })?;
            let function_name = function.sig.ident.to_string();
            out.insert(RustTest {
                crate_name: crate_name.to_owned(),
                source: source
                    .strip_prefix(&format!("crates/{crate_name}/tests/"))
                    .unwrap_or(source)
                    .to_owned(),
                body_name: format!("__trinity_test_body_{function_name}"),
                function: function_name,
                legacy: legacy.unwrap_or_default(),
                verification,
            });
        }
    }
    Ok(())
}

pub fn discover_doxygen_symbols(
    xml_dir: &Path,
) -> Result<Vec<DoxygenSymbol>, Box<dyn std::error::Error>> {
    let mut symbols = BTreeSet::new();
    if !xml_dir.exists() {
        return Ok(Vec::new());
    }
    for entry in WalkDir::new(xml_dir).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file()
            || entry.path().extension().and_then(|x| x.to_str()) != Some("xml")
        {
            continue;
        }
        let bytes = fs::read(entry.path())?;
        let text = String::from_utf8_lossy(&bytes);
        let Ok(document) = Document::parse(&text) else {
            continue;
        };
        for compound in document
            .descendants()
            .filter(|node| node.has_tag_name("compounddef"))
        {
            let compound_name = child_text(compound, "compoundname").unwrap_or_default();
            let source = compound
                .descendants()
                .find(|node| node.has_tag_name("location"))
                .and_then(|node| node.attribute("file"))
                .unwrap_or_default()
                .replace('\\', "/");
            if !compound_name.is_empty() {
                symbols.insert(DoxygenSymbol {
                    source: source.clone(),
                    qualified_name: compound_name.clone(),
                    kind: compound.attribute("kind").unwrap_or("compound").to_owned(),
                });
            }
            for member in compound
                .descendants()
                .filter(|node| node.has_tag_name("memberdef"))
            {
                let Some(name) = child_text(member, "name") else {
                    continue;
                };
                symbols.insert(DoxygenSymbol {
                    source: member
                        .descendants()
                        .find(|node| node.has_tag_name("location"))
                        .and_then(|node| node.attribute("file"))
                        .unwrap_or(&source)
                        .replace('\\', "/"),
                    qualified_name: if compound_name.is_empty() {
                        name
                    } else {
                        format!("{compound_name}::{name}")
                    },
                    kind: member.attribute("kind").unwrap_or("member").to_owned(),
                });
            }
        }
    }
    Ok(symbols.into_iter().collect())
}

fn child_text(node: roxmltree::Node<'_, '_>, tag: &str) -> Option<String> {
    node.children()
        .find(|child| child.has_tag_name(tag))
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(ToOwned::to_owned)
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
