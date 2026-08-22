//! Gate 5: zero runtime dependencies, no build script, no unsafe code.

use std::path::Path;
use std::process::Command;

/// `cargo metadata` shows no normal or build dependency for this package,
/// under every feature.
#[test]
fn no_runtime_dependencies() {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let output = Command::new(cargo)
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--all-features",
        ])
        .arg("--manifest-path")
        .arg(&manifest)
        .output()
        .expect("cargo metadata runs");
    assert!(output.status.success());
    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("metadata parses");
    let packages = metadata["packages"].as_array().expect("packages");
    let package = packages
        .iter()
        .find(|p| p["name"] == "digit-grouping")
        .expect("this package");
    for dependency in package["dependencies"].as_array().expect("dependencies") {
        assert_eq!(
            dependency["kind"].as_str(),
            Some("dev"),
            "non-dev dependency: {}",
            dependency["name"]
        );
    }
}

/// No build script exists.
#[test]
fn no_build_script() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!root.join("build.rs").exists());
}

/// The only `unsafe` in src/ is the forbid attribute.
#[test]
fn no_unsafe_code() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut stack = vec![src];
    let mut hits = Vec::new();
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = std::fs::read_to_string(&path).unwrap();
                for (number, line) in text.lines().enumerate() {
                    if line.contains("unsafe") {
                        hits.push((path.clone(), number + 1, line.to_owned()));
                    }
                }
            }
        }
    }
    assert_eq!(hits.len(), 1, "unexpected unsafe mentions: {hits:?}");
    assert!(hits[0].2.contains("forbid(unsafe_code)"));
}
