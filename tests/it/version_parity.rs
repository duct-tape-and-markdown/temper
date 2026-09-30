//! The one-version invariant, gated: "the engine and the SDK carry one version
//! and move together" (`specs/distribution.md`, "Versioning").
//!
//! Two homes a cut bumps together (`.claude/rules/release.md`, step 4) —
//! `Cargo.toml`'s version, which the engine reports as [`temper::VERSION`], and
//! `sdk/package.json`'s own `version` plus the per-platform
//! `optionalDependencies` engine pins. Only the engine half is gated elsewhere:
//! a Cargo bump without a lock re-emit fails `emit --frozen`, while an SDK-only
//! bump passes every other gate. The skew is not cosmetic — `src/install.rs`
//! embeds this same manifest to derive the dependency range `install` writes, so
//! a drifted `version` field mis-scaffolds adopters, and a drifted engine pin
//! resolves a binary that never wrote the lock the SDK emits.
//!
//! Deliberately out of scope: `sdk/package-lock.json`, which trails the tag by
//! design (release rule step 9 — the platform packages do not exist on the
//! registry until the publish) and is gated instead by `npm --prefix sdk ci`.

use serde_json::Value as JsonValue;

/// `sdk/package.json`'s own text, embedded at compile time — the same
/// `include_str!` seam `src/install.rs` reads it through, so this gate and the
/// range `install` writes can never disagree about which manifest is the
/// workspace SDK's.
const SDK_PACKAGE_JSON: &str = include_str!("../../sdk/package.json");

/// The remedy every failure below names: the release recipe's one step that
/// moves both homes at once.
const REMEDY: &str = "bump every version home together (.claude/rules/release.md, \
     step 4: Cargo.toml, sdk/package.json's `version`, and its \
     `optionalDependencies` engine pins) — a partial bump is the skew this gate \
     exists to refuse";

/// The parsed manifest. Panics rather than skipping: an unparseable or reshaped
/// manifest is the vacuity this suite is written to fail on, never to pass over.
fn manifest() -> JsonValue {
    serde_json::from_str(SDK_PACKAGE_JSON)
        .expect("sdk/package.json is a committed, well-formed manifest")
}

#[test]
fn the_sdk_manifests_version_equals_the_engines_own() {
    let sdk_version = manifest()["version"]
        .as_str()
        .expect("sdk/package.json declares a string `version` field")
        .to_owned();

    assert_eq!(
        sdk_version,
        temper::VERSION,
        "sdk/package.json `version` is {sdk_version} but the engine (Cargo.toml, \
         temper::VERSION) is {} — {REMEDY}",
        temper::VERSION
    );
}

#[test]
fn every_declared_engine_pin_equals_the_engines_own_version() {
    let manifest = manifest();

    // Non-vacuity (specs/process/engineering.md, "A green verdict is proven
    // non-vacuous"): an emptied or renamed map would otherwise let this lane
    // pass over zero pins, which is exactly the shape a bad edit leaves behind.
    let pins = manifest["optionalDependencies"]
        .as_object()
        .expect("sdk/package.json declares an `optionalDependencies` map of engine pins");
    assert!(
        !pins.is_empty(),
        "sdk/package.json's `optionalDependencies` carries no engine pins — the SDK \
         pins its engine per platform (specs/distribution.md, \"What ships\"), and an \
         empty map collapses this gate to a comparison over nothing"
    );

    for (package, pin) in pins {
        let pin = pin.as_str().unwrap_or_else(|| {
            panic!("sdk/package.json's `optionalDependencies.{package}` is a string version")
        });
        assert_eq!(
            pin,
            temper::VERSION,
            "sdk/package.json pins {package} at {pin} but the engine (Cargo.toml, \
             temper::VERSION) is {} — {REMEDY}",
            temper::VERSION
        );
    }
}
