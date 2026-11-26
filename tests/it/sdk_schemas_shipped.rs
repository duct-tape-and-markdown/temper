//! The SDK package ships the built-in floor's JSON Schema per frontmatter kind under
//! `sdk/schemas/<kind>.json` — the file the `yaml-language-server` modeline's
//! version-pinned CDN URL resolves to. Each is committed and byte-compared against a
//! fresh `temper schema --kind <kind>` run with no lock in reach, so a contract change
//! that moves the floor fails here until the files are regenerated:
//! `BLESS_SDK_SCHEMAS=1 cargo test --test it sdk_schemas_shipped`.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::common;

const BIN: &str = env!("CARGO_BIN_EXE_temper");

fn schemas_dir() -> PathBuf {
    common::sdk_root().join("schemas")
}

/// Runs `temper schema` with `args` in a fresh lockless directory, so the output is the
/// pure built-in floor.
fn schema_stdout(label: &str, args: &[&str]) -> String {
    let cwd = common::tmpdir(label);
    let output = Command::new(BIN)
        .current_dir(&cwd)
        .arg("schema")
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "temper schema {args:?} must exit zero: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

/// The floor's schema per frontmatter kind, keyed by file name — the kind list is the
/// engine's own domain (`temper schema`'s map keys), never a list kept here.
fn fresh_schemas() -> BTreeMap<String, String> {
    let all: serde_json::Value =
        serde_json::from_str(&schema_stdout("sdk-schemas-domain", &[])).unwrap();
    let kinds: Vec<String> = all.as_object().unwrap().keys().cloned().collect();
    assert!(
        kinds.iter().any(|kind| kind == "skill") && kinds.len() > 1,
        "the floor's domain is every frontmatter built-in kind, got: {kinds:?}"
    );
    kinds
        .iter()
        .map(|kind| {
            (
                format!("{kind}.json"),
                schema_stdout("sdk-schemas-kind", &["--kind", kind]),
            )
        })
        .collect()
}

fn committed_schemas() -> BTreeMap<String, String> {
    let Ok(entries) = fs::read_dir(schemas_dir()) else {
        return BTreeMap::new();
    };
    entries
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".json"))
        .map(|entry| {
            (
                entry.file_name().to_string_lossy().into_owned(),
                fs::read_to_string(entry.path()).unwrap(),
            )
        })
        .collect()
}

#[test]
fn the_sdk_package_ships_the_builtin_schema_of_every_frontmatter_kind_byte_for_byte() {
    let fresh = fresh_schemas();

    if std::env::var_os("BLESS_SDK_SCHEMAS").is_some() {
        let dir = schemas_dir();
        fs::create_dir_all(&dir).unwrap();
        // A retired kind leaves no orphan schema behind.
        for name in committed_schemas().keys() {
            if !fresh.contains_key(name) {
                fs::remove_file(dir.join(name)).unwrap();
            }
        }
        for (name, content) in &fresh {
            fs::write(dir.join(name), content).unwrap();
        }
        return;
    }

    let committed = committed_schemas();
    assert_eq!(
        committed.keys().collect::<Vec<_>>(),
        fresh.keys().collect::<Vec<_>>(),
        "sdk/schemas holds exactly one schema per frontmatter built-in kind; \
         regenerate with BLESS_SDK_SCHEMAS=1"
    );
    for (name, content) in &fresh {
        assert_eq!(
            &committed[name], content,
            "sdk/schemas/{name} drifted from `temper schema`; regenerate with BLESS_SDK_SCHEMAS=1"
        );
    }
}

#[test]
fn the_sdk_package_files_list_publishes_the_schemas() {
    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(common::sdk_root().join("package.json")).unwrap())
            .unwrap();
    let files: Vec<&str> = manifest["files"]
        .as_array()
        .expect("package.json carries a files list")
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect();
    assert!(
        files
            .iter()
            .any(|entry| entry.trim_end_matches('/') == "schemas"),
        "the published package carries sdk/schemas, got files: {files:?}"
    );
}
