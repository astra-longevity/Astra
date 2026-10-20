//! Tests: manifest checks, plan split, hash check and size check, including
//! the known-bad cases (wrong hash, missing licence, oversize file).

use std::fs;
use std::path::PathBuf;

use astra_db::sql::sha256_hex;
use serde_json::{Value, json};

use crate::plan::{Plan, Skipped};
use crate::sizes::{LIMIT_BYTES, oversize, read_allow_list};
use crate::verify::{pin, verify};
use crate::{check, load_all};

/// Fresh empty temp folder for test `name`.
fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("astra-fetch-test-{name}-{}", std::process::id()));
    if dir.exists() {
        fs::remove_dir_all(&dir).unwrap();
    }
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Content of the fake downloaded file.
const BODY: &[u8] = b"cohort,sex,days\nC2004,m,812\n";

/// A valid, approved, pinned manifest for `BODY`.
fn good() -> Value {
    json!({
        "id": "itp-demo",
        "licence": "CC0-1.0",
        "licence_status": "approved",
        "files": [{
            "url": "https://example.org/itp.csv",
            "sha256": sha256_hex(BODY),
            "size_bytes": BODY.len(),
            "fetched_at": "2026-10-20"
        }]
    })
}

/// Replaces the value at JSON `pointer` (which must exist).
fn set(value: &mut Value, pointer: &str, new: Value) {
    *value.pointer_mut(pointer).unwrap() = new;
}

/// `good()` with the given replacements.
fn with(changes: &[(&str, Value)]) -> Value {
    let mut value = good();
    for change in changes {
        set(&mut value, change.0, change.1.clone());
    }
    value
}

/// Errors of `check` on `value`, joined.
fn errors(value: &Value) -> String {
    check("itp-demo", value).err().unwrap_or_default().join("\n")
}

#[test]
fn good_manifest_passes() {
    let manifest = check("itp-demo", &good()).unwrap();
    assert!(manifest.approved(), "approved");
    assert_eq!(manifest.files.len(), 1, "one file");
}

#[test]
fn missing_licence_is_refused() {
    let mut value = good();
    let removed = value.as_object_mut().unwrap().remove("licence");
    assert!(removed.is_some(), "licence was present");
    assert!(errors(&value).contains("licence: missing"), "known-bad: no licence");
}

#[test]
fn bad_fields_are_refused() {
    let value = with(&[
        ("/id", json!("other")),
        ("/licence_status", json!("ok")),
        ("/files/0/url", json!("http://example.org/x")),
        ("/files/0/sha256", json!("ABC")),
    ]);
    let found = errors(&value);
    for needle in ["file name", "licence_status", "https://", "64 lowercase hex"] {
        assert!(found.contains(needle), "expected {needle:?} in {found}");
    }
    let half = with(&[("/files/0/size_bytes", Value::Null)]);
    assert!(errors(&half).contains("together"), "pinned without size");
}

#[test]
fn plan_skips_unapproved_and_flags_unpinned() {
    let approved = check("itp-demo", &good()).unwrap();
    let pending_value = with(&[("/id", json!("pending-set")), ("/licence_status", json!("pending"))]);
    let pending = check("pending-set", &pending_value).unwrap();
    let unpinned_value = with(&[
        ("/id", json!("new-set")),
        ("/files/0/sha256", Value::Null),
        ("/files/0/size_bytes", Value::Null),
        ("/files/0/fetched_at", Value::Null),
    ]);
    let unpinned = check("new-set", &unpinned_value).unwrap();
    let plan = Plan::new(&[approved, pending, unpinned]);
    assert_eq!(plan.fetch.len(), 1, "one file to fetch");
    let skipped = Skipped { dataset: "pending-set".to_owned(), status: "pending".to_owned() };
    assert_eq!(plan.skipped, vec![skipped], "pending skipped");
    assert_eq!(plan.unpinned.len(), 1, "one unpinned");
    let text = plan.render();
    assert!(text.contains("fetch\titp-demo\t0\t0\t") && text.contains("skipped\tpending-set\tpending"), "{text}");
}

#[test]
fn files_above_the_limit_are_left_to_their_job() {
    let manifest = check("itp-demo", &good()).unwrap();
    let size = u64::try_from(BODY.len()).unwrap();
    let under = Plan::limited(std::slice::from_ref(&manifest), Some(size));
    assert_eq!((under.fetch.len(), under.large.len()), (1, 0), "at the limit: cached");
    let over = Plan::limited(std::slice::from_ref(&manifest), Some(size - 1));
    assert_eq!((over.fetch.len(), over.large.len()), (0, 1), "above the limit: large");
    assert!(over.render().starts_with(&format!("large\titp-demo\t0\t{size}\t")), "{}", over.render());
    assert_eq!(Plan::new(&[manifest]).large.len(), 0, "no limit: everything fetched");
}

#[test]
fn min_interval_is_carried_and_checked() {
    let mut slow = good();
    let inserted = slow.as_object_mut().unwrap().insert("min_interval_seconds".to_owned(), json!(2u64));
    assert!(inserted.is_none(), "new field");
    let plan = Plan::new(&[check("itp-demo", &slow).unwrap()]);
    assert!(plan.render().starts_with("fetch\titp-demo\t0\t2\t"), "interval column");
    let mut bad = good();
    let replaced = bad.as_object_mut().unwrap().insert("min_interval_seconds".to_owned(), json!(-1i64));
    assert!(replaced.is_none(), "new field");
    assert!(errors(&bad).contains("min_interval_seconds"), "negative interval refused");
}

#[test]
fn database_manifest_shape_is_accepted() {
    let sha = sha256_hex(BODY);
    let shaped = json!({
        "id": "itp-demo",
        "licence": {"spdx_or_name": "MPD terms of use", "url": "https://e.org/t", "citation_required": true},
        "licence_status": "approved",
        "fetch": {"min_interval_seconds": 2u64},
        "fetched_at": "2026-10-20",
        "files": [{"url": "https://e.org/a.xlsx", "sha256": sha, "size_bytes": BODY.len(), "format": "xlsx"}]
    });
    let manifest = check("itp-demo", &shaped).unwrap();
    assert_eq!(manifest.licence, "MPD terms of use", "licence object");
    assert_eq!(manifest.files.first().and_then(|f| f.fetched_at.as_deref()), Some("2026-10-20"), "manifest date");
    assert!(Plan::new(&[manifest]).render().starts_with("fetch\titp-demo\t0\t2\t"), "nested interval");
    let empty =
        json!({"id": "itp-demo", "licence": {"spdx_or_name": "CC-BY-4.0"}, "licence_status": "approved", "files": []});
    assert!(check("itp-demo", &empty).unwrap().files.is_empty(), "files: [] allowed");
    let no_name =
        json!({"id": "itp-demo", "licence": {"url": "https://e.org"}, "licence_status": "approved", "files": []});
    assert!(errors(&no_name).contains("licence: missing"), "licence object without a name");
}

#[test]
fn verify_accepts_right_hash() {
    let cache = temp("ok");
    fs::write(cache.join(sha256_hex(BODY)), BODY).unwrap();
    let plan = Plan::new(&[check("itp-demo", &good()).unwrap()]);
    assert_eq!(verify(&plan, &cache), Ok(1), "one verified file");
}

#[test]
fn wrong_hash_fails_with_issue_message() {
    let cache = temp("bad");
    fs::write(cache.join(sha256_hex(BODY)), b"tampered upstream file").unwrap();
    let plan = Plan::new(&[check("itp-demo", &good()).unwrap()]);
    let found = verify(&plan, &cache).unwrap_err().join("\n");
    assert!(found.starts_with("Hash mismatch: dataset `itp-demo` files[0]"), "known-bad: {found}");
    assert!(found.contains("expected sha256") && found.contains("got      sha256"), "both hashes shown");
}

#[test]
fn missing_download_and_unpinned_fail() {
    let cache = temp("missing");
    let mut value = good();
    value
        .pointer_mut("/files")
        .and_then(Value::as_array_mut)
        .unwrap()
        .push(json!({ "url": "https://example.org/new.csv" }));
    let plan = Plan::new(&[check("itp-demo", &value).unwrap()]);
    let found = verify(&plan, &cache).unwrap_err().join("\n");
    assert!(found.contains("Not downloaded") && found.contains("Not pinned"), "{found}");
}

#[test]
fn pin_reports_hash_and_size() {
    let dir = temp("pin");
    let path = dir.join("f.csv");
    fs::write(&path, BODY).unwrap();
    let got = pin(&path).unwrap();
    assert_eq!(got.sha256, sha256_hex(BODY), "hash");
    assert_eq!(got.size_bytes, u64::try_from(BODY.len()).unwrap(), "size");
    assert_eq!(got.fetched_at.len(), 10, "date");
}

#[test]
fn oversize_file_is_refused_unless_allowed() {
    let root = temp("sizes");
    fs::write(root.join("small.txt"), b"x").unwrap();
    let big = fs::File::create(root.join("big.bin")).unwrap();
    big.set_len(LIMIT_BYTES + 1).unwrap();
    let files = vec!["small.txt".to_owned(), "big.bin".to_owned(), "gone.txt".to_owned()];
    let found = oversize(&root, &files, &[], LIMIT_BYTES);
    assert_eq!(found, vec![("big.bin".to_owned(), LIMIT_BYTES + 1)], "known-bad: oversize");
    fs::write(root.join("allow.txt"), "# comment\nbig.bin  # reason\n\n").unwrap();
    let allow = read_allow_list(&root.join("allow.txt")).unwrap();
    assert!(oversize(&root, &files, &allow, LIMIT_BYTES).is_empty(), "allow-listed");
}

#[test]
fn load_all_reads_folder_and_reports_every_problem() {
    let root = temp("load");
    let dir = root.join("data/datasets");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("b-set.yaml"), "id: b-set\nlicence_status: pending\nfiles:\n  - url: https://e.org/b\n")
        .unwrap();
    fs::write(
        dir.join("a-set.yaml"),
        "id: a-set\nlicence: CC0-1.0\nlicence_status: refused\nfiles:\n  - url: https://e.org/a\n",
    )
    .unwrap();
    let found = load_all(&root).unwrap_err().join("\n");
    assert!(found.contains("b-set: licence: missing"), "{found}");
    fs::write(
        dir.join("b-set.yaml"),
        "id: b-set\nlicence: CC0-1.0\nlicence_status: pending\nfiles:\n  - url: https://e.org/b\n",
    )
    .unwrap();
    let ids: Vec<String> = load_all(&root).unwrap().into_iter().map(|m| m.id).collect();
    assert_eq!(ids, vec!["a-set".to_owned(), "b-set".to_owned()], "sorted");
}

#[test]
fn repository_manifests_are_valid() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let loaded = load_all(&root);
    assert!(loaded.is_ok(), "data/datasets: {:?}", loaded.err());
}
