use std::{env, path::PathBuf};

use trinity_codegen::{discover_doxygen_symbols, discover_legacy_tests, discover_rust_tests, write_inventory, write_kani_harnesses, Inventory};

const BASELINE_COMMIT: &str = "a9343c8904310374f284c38a71998af0672b3184";
const DOCS_SHA256: &str = "9f77e297e1c0bcb83f0cac2cdb390ac43de9e341db9d11e6e0efce9ad59ec255";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut workspace_root = PathBuf::from(".");
    let mut repo_root = PathBuf::from("..");
    let mut xml_dir = PathBuf::from("generated/doxygen-xml/xml");
    let mut out_dir = PathBuf::from("generated");
    let mut deny_unmapped_kani = false;

    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "generate".to_owned());
    if command != "generate" {
        return Err(format!("unsupported command `{command}`; expected `generate`").into());
    }
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--workspace-root" => workspace_root = PathBuf::from(args.next().ok_or("missing --workspace-root value")?),
            "--repo-root" => repo_root = PathBuf::from(args.next().ok_or("missing --repo-root value")?),
            "--xml" => xml_dir = PathBuf::from(args.next().ok_or("missing --xml value")?),
            "--out" => out_dir = PathBuf::from(args.next().ok_or("missing --out value")?),
            "--deny-unmapped-kani" => deny_unmapped_kani = true,
            other => return Err(format!("unknown argument `{other}`").into()),
        }
    }

    let inventory = Inventory {
        baseline_commit: BASELINE_COMMIT.to_owned(),
        docs_sha256: DOCS_SHA256.to_owned(),
        doxygen_symbols: discover_doxygen_symbols(&workspace_root.join(&xml_dir))?,
        legacy_tests: discover_legacy_tests(&workspace_root.join(&repo_root))?,
        rust_tests: discover_rust_tests(&workspace_root)?,
    };
    write_inventory(&workspace_root.join(&out_dir), &inventory)?;
    write_kani_harnesses(&workspace_root, &workspace_root.join(&out_dir), &inventory.rust_tests)?;

    if deny_unmapped_kani {
        let mapped = inventory.rust_tests.iter().map(|test| test.legacy.as_str()).collect::<std::collections::BTreeSet<_>>();
        let unmapped = inventory.legacy_tests.iter().filter(|test| test.suggested_verification == trinity_codegen::VerificationClass::Kani && !mapped.contains(test.id.as_str())).count();
        if unmapped != 0 {
            return Err(format!("{unmapped} Kani-suitable legacy contracts remain unmapped").into());
        }
    }

    println!(
        "trinity-codegen: {} Doxygen symbols, {} legacy tests, {} Rust semantic tests, {} Kani harnesses",
        inventory.doxygen_symbols.len(),
        inventory.legacy_tests.len(),
        inventory.rust_tests.len(),
        inventory.rust_tests.iter().filter(|test| test.verification == trinity_codegen::VerificationClass::Kani).count(),
    );
    Ok(())
}
