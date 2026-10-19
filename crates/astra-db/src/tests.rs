//! Tests on `data/examples/`: validity, determinism, error cases.

use std::fs;
use std::path::{Path, PathBuf};

use crate::rules::{civil, today};
use crate::{DATABASE_FILE, Failure, Options, build};

/// Repository root (two levels above this crate).
fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Options for the examples folder.
fn examples(check: bool) -> Options {
    Options { root: repo(), data_dir: "data/examples".to_owned(), check, strict: false }
}

/// Fresh fake repository in the temp dir: `data/schema` + a copy of the examples as `data/`.
fn fake_repo(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("astra-db-{name}-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    copy_dir(&repo().join("data/schema"), &root.join("data/schema"));
    for entry in fs::read_dir(repo().join("data/examples")).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            copy_dir(&path, &root.join("data").join(path.file_name().unwrap()));
        }
    }
    root
}

/// Copies the files of one folder.
fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            let _bytes: u64 = fs::copy(&path, to.join(path.file_name().unwrap())).unwrap();
        }
    }
}

/// Options for a fake repository.
fn fake(root: &Path) -> Options {
    Options { root: root.to_path_buf(), data_dir: "data".to_owned(), check: false, strict: false }
}

/// Codes of the diagnostics of a failed build.
fn codes(failure: Failure) -> Vec<&'static str> {
    match failure {
        Failure::Invalid(diags) => diags.items.iter().map(|diag| diag.code).collect(),
        Failure::Usage(_) | Failure::Internal(_) => Vec::new(),
    }
}

#[test]
fn examples_are_valid() {
    let outcome = build(&examples(true), "2026-10-19").unwrap();
    assert_eq!(outcome.diags.errors(), 0, "no error");
    assert!(outcome.written.is_none(), "check writes nothing");
    let claims = outcome.counts.iter().find(|count| count.0 == "claims").map(|count| count.1);
    assert_eq!(claims, Some(2), "two example claims");
}

#[test]
fn build_is_deterministic() {
    let root = fake_repo("determinism");
    let first = build(&fake(&root), "2026-10-19").unwrap().written.unwrap();
    let bytes = fs::read(&first).unwrap();
    let second = build(&fake(&root), "2026-10-19").unwrap().written.unwrap();
    assert_eq!(first, second, "same path");
    assert_eq!(bytes, fs::read(&second).unwrap(), "byte-identical");
    assert!(root.join("data").join(DATABASE_FILE).is_file(), "written in data/");
    fs::remove_dir_all(&root).unwrap();
}

#[test]
fn broken_reference_is_reported() {
    let root = fake_repo("broken");
    let mut claims: Vec<PathBuf> =
        fs::read_dir(root.join("data/claims")).unwrap().map(|entry| entry.unwrap().path()).collect();
    claims.sort();
    let claim = claims.first().unwrap().clone();
    let text = fs::read_to_string(&claim).unwrap().replace("mission: aging-map", "mission: nowhere");
    fs::write(&claim, text).unwrap();
    fs::write(root.join("data/claims/notes.txt"), "x").unwrap();
    let failure = build(&fake(&root), "2026-10-19").unwrap_err();
    assert_eq!(failure.exit_code(), 1, "validation error");
    let found = codes(failure);
    assert!(found.contains(&"E020"), "missing mission: {found:?}");
    assert!(found.contains(&"E002"), "unexpected file: {found:?}");
    assert!(!root.join("data").join(DATABASE_FILE).exists(), "nothing written");
    fs::remove_dir_all(&root).unwrap();
}

#[test]
fn bad_yaml_and_nfc_are_reported() {
    let root = fake_repo("yaml");
    fs::write(root.join("data/missions/aging-map.yaml"), "id: &a aging-map\ntitle: *a\n").unwrap();
    fs::write(root.join("data/concepts/polg.yaml"), "id: polg\nname: \"Pole\u{301}\"\n").unwrap();
    let failure = build(&fake(&root), "2026-10-19").unwrap_err();
    let found = codes(failure);
    let e010 = found.iter().filter(|code| **code == "E010").count();
    assert_eq!(e010, 2, "anchor and NFC: {found:?}");
    fs::remove_dir_all(&root).unwrap();
}

#[test]
fn usage_and_strict() {
    let mut opts = examples(true);
    opts.data_dir = "nope".to_owned();
    assert_eq!(build(&opts, "2026-10-19").unwrap_err().exit_code(), 2, "missing data dir");
    let mut strict = examples(true);
    strict.strict = true;
    let lenient = build(&examples(true), "2026-10-19").unwrap();
    let result = build(&strict, "2026-10-19");
    assert_eq!(result.is_err(), lenient.diags.warnings() > 0, "warnings block in strict mode");
}

#[test]
fn civil_dates() {
    assert_eq!(civil(0).as_deref(), Some("1970-01-01"), "epoch");
    assert_eq!(civil(20_745).as_deref(), Some("2026-10-19"), "today");
    assert_eq!(civil(11_016).as_deref(), Some("2000-02-29"), "leap day");
    assert_eq!(today().len(), 10, "AAAA-MM-JJ");
}
