# A declared collection key whose value is not an object: finding or silence?

**Site.** `src/json_manifest.rs` `manifest_members` (the `let Some(JsonValue::Object(collection)) … else { return Vec::new() }` guard) together with `Manifest::parse`, which adds the collection key to `consumed` before that guard runs. A manifest spelling `"mcpServers": []`, or `"hooks": "x"` in `settings.json`, therefore reads as zero registration members and no opaque field. No clause ever sees the value, and `check` is green. A represented manifest's `write_manifest` then regenerates it without the key.

**What I checked.** `specs/builtins.md` (intro: "A document the runtime loads is never a load fault; one it skips is"), invariant 6 (loud or nothing), and `tests/it/manifest_schema_oracle.rs`, which reports and does not gate. No test pins the non-object case. The posture pages have no section covering a reader that silently drops a malformed shape. "A green verdict is proven non-vacuous" governs judge tests, not readers.

**The fork.**
1. A load fault or finding that names the collection key and the JSON type it carries.
2. Correct silence, if Claude Code itself ignores a non-object collection.

Which one holds is an external fact, uncited and UNVERIFIED here.

**Recommendation.** Option 1 if the settings/MCP docs say the file fails validation. In either case, a value that is consumed and never judged should not also be dropped on regenerate. Also proposed: a posture section, "A reader refuses or reports what it consumes". Its tell would be a key a reader consumes into no member, no field and no finding. That is the sentence that would have named this site.
