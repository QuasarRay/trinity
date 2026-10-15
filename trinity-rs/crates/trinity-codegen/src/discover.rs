use std::{collections::BTreeSet, fs, path::Path};

use regex::Regex;
use roxmltree::Document;
use walkdir::WalkDir;

use crate::model::{DoxygenSymbol, LegacyTest, LegacyTestKind, RustTest, VerificationClass};

pub fn discover_legacy_tests(
    root: &Path,
) -> Result<Vec<LegacyTest>, Box<dyn std::error::Error>> {
    let mut out = BTreeSet::new();
    for (path, kind) in [
        (root.join("trinityal/tests"), 0),
        (root.join("trinity/tests/Python/tests"), 1),
    ] {
        if !path.exists() {
            continue;
        }
        for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }
            let extension = entry.path().extension().and_then(|value| value.to_str());
            if (kind == 0 && !matches!(extension, Some("cpp" | "cc" | "cxx")))
                || (kind == 1 && extension != Some("py"))
            {
                continue;
            }
            let source = relative(root, entry.path());
            let text = read_lossy(entry.path())?;
            if kind == 0 {
                out.extend(discover_gtest_text(&source, &text));
            } else {
                out.extend(discover_python_text(&source, &text));
            }
        }
    }
    Ok(out.into_iter().collect())
}

#[must_use]
pub fn discover_gtest_text(source: &str, text: &str) -> Vec<LegacyTest> {
    let regex =
        Regex::new(r"\bTEST(?:_F|_P)?\s*\(\s*([^,]+)\s*,\s*([^)]+)\)").unwrap();
    let mut out = BTreeSet::new();
    for captures in regex.captures_iter(text) {
        let suite = captures[1].trim().to_owned();
        let name = captures[2].trim().to_owned();
        let position = captures.get(0).map_or(0, |matched| matched.start());
        let line = u32::try_from(
            text[..position]
                .bytes()
                .filter(|byte| *byte == b'\n')
                .count()
                + 1,
        )
        .ok();
        let id = format!("{source}::{suite}::{name}");
        let lower = id.to_ascii_lowercase();
        let verification = if source.ends_with("ALResultTest.cpp")
            || source.ends_with("BitmapDimensions.cpp")
            || source.ends_with("VertexDefinition.cpp")
            || source.ends_with("TextureSubresource.cpp")
        {
            VerificationClass::Kani
        } else if lower.contains("screenshot") {
            VerificationClass::Screenshot
        } else if [
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
        .any(|needle| lower.contains(needle))
        {
            VerificationClass::RenderDevice
        } else {
            VerificationClass::Integration
        };
        out.insert(LegacyTest {
            id,
            source: source.into(),
            suite,
            name,
            line,
            kind: LegacyTestKind::CppGTest,
            suggested_verification: verification,
        });
    }
    out.into_iter().collect()
}

#[must_use]
pub fn discover_python_text(source: &str, text: &str) -> Vec<LegacyTest> {
    let mut out = BTreeSet::new();
    let mut class = "module".to_owned();
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("class ") {
            let next_class = rest
                .split(['(', ':'])
                .next()
                .unwrap_or("module")
                .trim();
            class.clear();
            class.push_str(next_class);
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("def ") {
            let name = rest.split('(').next().unwrap_or("").trim();
            if name.starts_with("test_") {
                out.insert(LegacyTest {
                    id: format!("{source}::{class}::{name}"),
                    source: source.into(),
                    suite: class.clone(),
                    name: name.into(),
                    line: u32::try_from(index + 1).ok(),
                    kind: LegacyTestKind::Python,
                    suggested_verification: VerificationClass::Kani,
                });
            }
        }
    }
    out.into_iter().collect()
}

pub fn discover_rust_tests(root: &Path) -> Result<Vec<RustTest>, Box<dyn std::error::Error>> {
    let mut out = BTreeSet::new();
    for entry in WalkDir::new(root.join("crates"))
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file()
            || entry.path().extension().and_then(|value| value.to_str()) != Some("rs")
        {
            continue;
        }
        let relative_path = relative(root, entry.path());
        if !relative_path.contains("/tests/") || relative_path.ends_with("_kani_generated.rs") {
            continue;
        }
        let crate_name = relative_path.split('/').nth(1).unwrap_or("unknown").to_owned();
        let text = fs::read_to_string(entry.path())?;
        let file = syn::parse_file(&text)?;
        for item in file.items {
            let syn::Item::Fn(function) = item else {
                continue;
            };
            let Some(attr) = function
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("trinity_test"))
            else {
                continue;
            };
            let mut legacy = String::new();
            let mut verification = VerificationClass::Kani;
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("legacy") {
                    legacy = meta.value()?.parse::<syn::LitStr>()?.value();
                } else if meta.path.is_ident("verification") {
                    verification = match meta.value()?.parse::<syn::LitStr>()?.value().as_str() {
                        "kani" => VerificationClass::Kani,
                        "native" => VerificationClass::Native,
                        "render-device" => VerificationClass::RenderDevice,
                        "screenshot" => VerificationClass::Screenshot,
                        "integration" => VerificationClass::Integration,
                        _ => VerificationClass::Unmapped,
                    };
                }
                Ok(())
            })?;
            let function_name = function.sig.ident.to_string();
            let source = relative_path
                .strip_prefix(&format!("crates/{crate_name}/tests/"))
                .unwrap_or(&relative_path)
                .to_owned();
            out.insert(RustTest {
                crate_name: crate_name.clone(),
                source,
                function: function_name.clone(),
                body_name: format!("__trinity_test_body_{function_name}"),
                legacy,
                verification,
            });
        }
    }
    Ok(out.into_iter().collect())
}

pub fn discover_doxygen_symbols(
    xml: &Path,
) -> Result<Vec<DoxygenSymbol>, Box<dyn std::error::Error>> {
    let mut out = BTreeSet::new();
    if !xml.exists() {
        return Ok(vec![]);
    }
    for entry in WalkDir::new(xml).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file()
            || entry.path().extension().and_then(|value| value.to_str()) != Some("xml")
        {
            continue;
        }
        let text = read_lossy(entry.path())?;
        let Ok(document) = Document::parse(&text) else {
            continue;
        };
        for compound in document
            .descendants()
            .filter(|node| node.has_tag_name("compounddef"))
        {
            let name = child(compound, "compoundname").unwrap_or_default();
            let source = compound
                .descendants()
                .find(|node| node.has_tag_name("location"))
                .and_then(|node| node.attribute("file"))
                .unwrap_or("")
                .replace('\\', "/");
            if !name.is_empty() {
                out.insert(DoxygenSymbol {
                    source: source.clone(),
                    qualified_name: name.clone(),
                    kind: compound.attribute("kind").unwrap_or("compound").into(),
                });
            }
            for member in compound
                .descendants()
                .filter(|node| node.has_tag_name("memberdef"))
            {
                if let Some(member_name) = child(member, "name") {
                    out.insert(DoxygenSymbol {
                        source: source.clone(),
                        qualified_name: if name.is_empty() {
                            member_name
                        } else {
                            format!("{name}::{member_name}")
                        },
                        kind: member.attribute("kind").unwrap_or("member").into(),
                    });
                }
            }
        }
    }
    Ok(out.into_iter().collect())
}

fn read_lossy(path: &Path) -> Result<String, std::io::Error> {
    Ok(String::from_utf8_lossy(&fs::read(path)?).into_owned())
}

fn child(node: roxmltree::Node<'_, '_>, tag: &str) -> Option<String> {
    node.children()
        .find(|child| child.has_tag_name(tag))
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
