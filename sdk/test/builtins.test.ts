/**
 * The built-in default contracts: every default contract exported from `claude-code.ts` is a
 * well-formed clause array, and every clause carries a non-empty `cite` — the
 * auditability guarantee a maintained default contract exists to keep.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import type { Clause } from "../src/index.js";
import {
  agent,
  agentDefaultContract,
  command,
  commandDefaultContract,
  hookDefaultContract,
  installedPlugin,
  installedPluginDefaultContract,
  knownMarketplace,
  knownMarketplaceDefaultContract,
  marketplace,
  marketplaceDefaultContract,
  mcpServer,
  mcpServerDefaultContract,
  memory,
  memoryAnthropicDefaultContract,
  pluginManifest,
  pluginManifestDefaultContract,
  rule,
  ruleDefaultContract,
  settingsLocal,
  settingsLocalDefaultContract,
  skill,
  skillDefaultContract,
  supportingDoc,
  supportingDocDefaultContract,
} from "../src/claude-code.js";

const DEFAULT_CONTRACTS: ReadonlyArray<readonly Clause[]> = [
  agentDefaultContract,
  skillDefaultContract,
  commandDefaultContract,
  hookDefaultContract,
  installedPluginDefaultContract,
  mcpServerDefaultContract,
  ruleDefaultContract,
  memoryAnthropicDefaultContract,
  pluginManifestDefaultContract,
  marketplaceDefaultContract,
  supportingDocDefaultContract,
  settingsLocalDefaultContract,
  knownMarketplaceDefaultContract,
];

test("every exported default contract is a well-formed clause array", () => {
  for (const defaultContract of DEFAULT_CONTRACTS) {
    assert.ok(Array.isArray(defaultContract));
    for (const entry of defaultContract) {
      assert.ok(entry.predicate && typeof entry.predicate.key === "string" && entry.predicate.key.length > 0);
      assert.ok(entry.severity === "required" || entry.severity === "advisory");
 }
 }
});

test("every default contract clause carries a non-empty cite", () => {
  for (const defaultContract of DEFAULT_CONTRACTS) {
    for (const entry of defaultContract) {
      assert.ok(typeof entry.cite === "string" && entry.cite.length > 0, `clause \`${entry.predicate.key}\` is uncited`);
 }
 }
});

test("skillDefaultContract carries the skill kind's decidable clauses, name-first", () => {
  assert.equal(skillDefaultContract.length, 15);
  assert.equal(skillDefaultContract[0].predicate.key, "required");
  assert.equal(skillDefaultContract[0].predicate.field, "name");
  assert.deepEqual(
    skillDefaultContract.map((c) => c.predicate.key),
    [
      "required",
      "min_len",
      "allowed_chars",
      "max_len",
      "shape",
      "deny",
      "name-matches-dir",
      "required",
      "min_len",
      "max_len",
      "shape",
      "max_len",
      "extent",
      "forbidden_keys",
      "glob-valid",
    ],
  );
  // The `glob-valid` clause ranges over the `paths` scope.
  assert.equal(skillDefaultContract[14].predicate.key, "glob-valid");
  assert.equal(skillDefaultContract[14].predicate.field, "paths");
});

test("commandDefaultContract is skillDefaultContract minus the directory-name clause", () => {
  assert.deepEqual(
    commandDefaultContract.map((c) => c.predicate.key),
    skillDefaultContract.map((c) => c.predicate.key).filter((key) => key !== "name-matches-dir"),
  );
  assert.equal(
    commandDefaultContract.some((c) => c.predicate.key === "name-matches-dir"),
    false,
    "a command is a lone file — no parent directory to match",
  );
  // `name` requiredness rides over unchanged: a command still declares no `name`
  // field for identity (file-stem, like `rule`), but the skill schema's own
  // `required`/`min_len`/`allowed_chars`/`max_len`/`deny` clauses over `name` still
  // apply by import.
  assert.equal(commandDefaultContract[0].predicate.key, "required");
  assert.equal(commandDefaultContract[0].predicate.field, "name");
});

test("ruleDefaultContract forbids Cursor keys, validates path globs, budgets body size, and gates mentions", () => {
  assert.deepEqual(
    ruleDefaultContract.map((c) => c.predicate.key),
    ["forbidden_keys", "glob-valid", "extent", "mention-reachable"],
  );
  assert.deepEqual(ruleDefaultContract[0].predicate.keys, ["description", "globs", "alwaysApply"]);
  // The `glob-valid` clause ranges over the one documented rules key, `paths`.
  assert.equal(ruleDefaultContract[1].predicate.field, "paths");
  // Both ends of `mention-reachable` are `paths`: the rule's own scope is the source,
  // the mentioned member's is the gate. Advisory — literal containment can be wrong, so
  // it must not block (0028).
  assert.equal(ruleDefaultContract[3].predicate.field, "paths");
  assert.equal(ruleDefaultContract[3].predicate.gate, "paths");
  assert.equal(ruleDefaultContract[3].severity, "advisory");
});

test("memoryAnthropicDefaultContract is a single advisory size budget", () => {
  assert.equal(memoryAnthropicDefaultContract.length, 1);
  assert.equal(memoryAnthropicDefaultContract[0].predicate.key, "extent");
  assert.equal(memoryAnthropicDefaultContract[0].severity, "advisory");
});

test("mcpServer is a fields-only manifest kind at the mcpServers.* collection address", () => {
  assert.equal(mcpServer.facts.shape, "fields");
  assert.equal(mcpServer.facts.unitShape, "file");
  assert.equal(mcpServer.facts.format, undefined);
  assert.deepEqual(mcpServer.facts.locus, { kind: "at", root: ".", glob: ".mcp.json" });
  assert.deepEqual(mcpServer.facts.registration, [{ via: "connection" }]);
  assert.deepEqual(mcpServer.facts.collectionAddress, {
    manifest: ".mcp.json",
    keyPath: "mcpServers.*",
    entryShape: "object",
  });
});

test("installedPlugin is a fields-only manifest kind at the enabledPlugins.* collection address", () => {
  assert.equal(installedPlugin.facts.shape, "fields");
  assert.equal(installedPlugin.facts.unitShape, "file");
  assert.equal(installedPlugin.facts.format, undefined);
  assert.deepEqual(installedPlugin.facts.locus, { kind: "at", root: ".claude", glob: "settings.json" });
  // The entry's own presence is the channel — fieldless, as a connection's is.
  assert.deepEqual(installedPlugin.facts.registration, [{ via: "enablement" }]);
  assert.deepEqual(installedPlugin.facts.collectionAddress, {
    manifest: "settings.json",
    keyPath: "enabledPlugins.*",
    entryShape: "scalar(enabled)",
  });
});

test("installedPluginDefaultContract ships empty — an assertion, not an omission", () => {
  // The format documents no gateable schema: an entry is one scalar under a key that is
  // the member's identity rather than a declared field, so an almost-empty format earns
  // an almost-empty contract rather than a clause resting on an unsettled fact.
  assert.deepEqual(installedPluginDefaultContract, []);
});

test("knownMarketplace is a fields-only manifest kind at the extraKnownMarketplaces.* collection address", () => {
  assert.equal(knownMarketplace.facts.shape, "fields");
  assert.equal(knownMarketplace.facts.unitShape, "file");
  assert.equal(knownMarketplace.facts.format, undefined);
  assert.deepEqual(knownMarketplace.facts.locus, { kind: "at", root: ".claude", glob: "settings.json" });
  // The registry entry's own presence is the channel — fieldless, and never provably dead,
  // as a connection's is.
  assert.deepEqual(knownMarketplace.facts.registration, [{ via: "registry" }]);
  assert.deepEqual(knownMarketplace.facts.collectionAddress, {
    manifest: "settings.json",
    keyPath: "extraKnownMarketplaces.*",
    entryShape: "object",
  });
});

test("knownMarketplaceDefaultContract ships empty — an assertion, not an omission", () => {
  // The `source` union and `autoUpdate` boolean are the type's to hold; the key is the
  // marketplace name, an identity rather than a declared field, so no decidable clause
  // survives that the type does not already enforce.
  assert.deepEqual(knownMarketplaceDefaultContract, []);
});

test("mcpServerDefaultContract gates the transport type against the documented set", () => {
  assert.deepEqual(
    mcpServerDefaultContract.map((c) => c.predicate.key),
    ["enum"],
  );
  assert.equal(mcpServerDefaultContract[0].predicate.field, "type");
  assert.deepEqual(mcpServerDefaultContract[0].predicate.values, [
    "stdio",
    "http",
    "streamable-http",
    "sse",
    "ws",
  ]);
  assert.equal(mcpServerDefaultContract[0].severity, "required");
});

test("the default contracts ride alongside their kinds through the claude-code subpath", () => {
  assert.equal(typeof agent, "function");
  assert.equal(typeof skill, "function");
  assert.equal(typeof command, "function");
  assert.equal(typeof rule, "function");
  assert.equal(typeof memory, "function");
  assert.equal(typeof supportingDoc, "function");
});

test("skill templates one file-child layer of supporting-doc at the directory's markdown", () => {
  assert.equal(skill.facts.templates?.length, 1);
  const [reference] = skill.facts.templates ?? [];
  // The child travels by import, never by string — the template holds the kind value.
  assert.equal(reference.kind, supportingDoc);
  assert.equal(reference.kind.key, "supporting-doc");
  // A file layer, so it carries the path its children sit at relative to the skill's
  // own unit: the documented `my-skill/reference.md` placement. A supporting file of
  // another type matches nothing here and stays unmodeled rather than mis-typed.
  assert.equal(reference.path, "*.md");
});

test("supporting-doc is a nested-file kind: fields-free, prose-only, channel-less, identity from the filename", () => {
  assert.deepEqual(supportingDoc.facts.locus, { kind: "nested-file" });
  // Frontmatterless — no declared format, so the whole file is body.
  assert.equal(supportingDoc.facts.format, undefined);
  // A lone file whose identity is its stem: no identityField carries the name.
  assert.equal(supportingDoc.facts.unitShape, "file");
  assert.equal(supportingDoc.facts.identityField, undefined);
  // Channel-less: it reaches the world only through the skill that references it.
  assert.deepEqual(supportingDoc.facts.registration, []);
  // Fields-free, but still body-bearing — never the fields-only registration shape.
  assert.equal(supportingDoc.facts.shape, undefined);
  const member = supportingDoc({ name: "reference", host: skill({ name: "demo", description: "A host." }) });
  assert.deepEqual(member.fields, []);
});

test("supportingDocDefaultContract is one advisory reach clause — the format's one decidable fact", () => {
  // The format documents no frontmatter schema, no required field and no cap, so the
  // only clause is the one that ranges over the graph rather than the file's bytes.
  assert.equal(supportingDocDefaultContract.length, 1);
  const [reach] = supportingDocDefaultContract;
  // An incoming-degree floor: at least one resolved edge must reach the document.
  // Locus-agnostic — any edge from the host skill counts, a mention included — so the
  // bound names no field and leaves the outgoing direction unconstrained.
  assert.deepEqual(reach.predicate, { key: "degree", args: { incoming_min: 1 } });
  // Advisory: a shipped coverage clause enters advisory, and escalation is the
  // adopting corpus's declared act, never this default's.
  assert.equal(reach.severity, "advisory");
  assert.match(reach.guidance ?? "", /never points at/);
  assert.equal(reach.cite, "https://code.claude.com/docs/en/skills (retrieved 2026-07-16)");
});

test("plugin-manifest is a json-document file kind identified by its name key, owning its file", () => {
  assert.deepEqual(pluginManifest.facts.locus, { kind: "at", root: ".claude-plugin", glob: "plugin.json" });
  // The one built-in at the whole-artifact JSON format — never frontmatter over a body.
  assert.equal(pluginManifest.facts.format, "json-document");
  // Identity from the document's own key: every manifest's stem is `plugin`, so the
  // named-field mode is the only one that tells two apart.
  assert.equal(pluginManifest.facts.unitShape, "named-field");
  assert.equal(pluginManifest.facts.identityField, "name");
  // It *is* the manifest rather than surfacing inside one, so it owns its file: no
  // collection address, and never the fields-only registration shape.
  assert.equal(pluginManifest.facts.collectionAddress, undefined);
  assert.equal(pluginManifest.facts.shape, undefined);
  // Channel-less: distribution metadata reaches the installer, never the model.
  assert.deepEqual(pluginManifest.facts.registration, []);
});

test("pluginManifestDefaultContract gates the --strict profile", () => {
  // `name`'s presence, emptiness and charset; the experimental deny-list slice; every
  // wrong-typed field — `keywords`' single kind and the six component paths' documented
  // unions, which a `type` over a set reaches; and the closed key set, which is the rest
  // of `--strict`: the `optional` rows declare every other documented key and
  // `closedKeys()` reads them as the allow-list.
  assert.deepEqual(
    [...new Set(pluginManifestDefaultContract.map((entry) => entry.predicate.key))],
    ["required", "min_len", "allowed_chars", "forbidden_keys", "type", "optional", "closed-keys"],
  );
  // Every clause is an error: `--strict` is the portable bar, so nothing here is a note.
  assert.ok(pluginManifestDefaultContract.every((entry) => entry.severity === "required"));

  const [presence, empty, charset, experimental, keywordsType] = pluginManifestDefaultContract;
  assert.deepEqual(presence.predicate, { key: "required", field: "name" });
  assert.deepEqual(empty.predicate, { key: "min_len", field: "name", args: { min: 1 } });
  // Kebab-case, no spaces — the charset is the whole rule the docs state.
  assert.deepEqual(charset.predicate, {
    key: "allowed_chars",
    field: "name",
    charset: { ranges: ["a-z", "0-9"], chars: "-" },
  });
  assert.deepEqual(experimental.predicate, { key: "forbidden_keys", keys: ["themes", "monitors"] });
  // The declared kinds ride their own field, not the shared `args` bag — the lattice
  // names the engine decodes, in the one spelling that crosses the lock. A single-kind
  // check is the one-element set, no second spelling for it.
  assert.deepEqual(keywordsType.predicate, { key: "type", field: "keywords", value_type: ["list"] });

  // The six component-path fields, each gated over the whole union its documentation
  // states: declaring a subset would reject a documented form, so the set is the clause.
  assert.deepEqual(
    pluginManifestDefaultContract
      .map((entry) => entry.predicate)
      .filter((predicate) => predicate.key === "type" && predicate.field !== "keywords")
      .map((predicate) => [predicate.field, predicate.value_type]),
    [
      ["skills", ["string", "list"]],
      ["commands", ["string", "list", "map"]],
      ["agents", ["string", "list"]],
      ["hooks", ["string", "list", "map"]],
      ["mcpServers", ["string", "list", "map"]],
      ["lspServers", ["string", "list", "map"]],
    ],
  );
  // The runtime divergence rides the guidance, the one channel that can carry it: the
  // clause decides the key's presence, never which world the reader is validating in.
  assert.match(experimental.guidance ?? "", /--strict/);
  assert.match(charset.guidance ?? "", /displayName/);
  // The one clause here the forgiving runtime does not wave through, so its guidance is
  // where the reader learns this is a load error rather than another `--strict` warning.
  assert.match(keywordsType.guidance ?? "", /load error/);

  // Cited and dated, every one — the audit trail a maintained default contract exists
  // for. Two sources appear: the reference page, and the published schema wherever it
  // documents what the reference's tables do not — the `commands` object form, and the
  // `settings` key, which is declared by the schema alone.
  for (const entry of pluginManifestDefaultContract) {
    assert.match(
      entry.cite ?? "",
      /^https:\/\/(code\.claude\.com\/docs\/en\/plugins-reference#\S+|json\.schemastore\.org\/claude-code-plugin-manifest\.json) \(retrieved 2026-07-\d\d\)/,
    );
  }
  const commandsType = pluginManifestDefaultContract.find(
    (entry) => entry.predicate.key === "type" && entry.predicate.field === "commands",
  );
  assert.match(commandsType?.cite ?? "", /json\.schemastore\.org\/claude-code-plugin-manifest\.json \(retrieved 2026-07-16\)$/);
});

test("pluginManifestDefaultContract's closed key set is the union of both documented sources", () => {
  // The allow-list `closedKeys()` consumes, stated here as the key set rather than read
  // back from the clause order: a documented key dropped from the rows above becomes a
  // finding against every manifest that carries it, which is the one failure this
  // widening can produce.
  const declared = pluginManifestDefaultContract
    .map((entry) => entry.predicate)
    .filter((predicate) => predicate.key === "required" || predicate.key === "optional")
    .map((predicate) => predicate.field);

  assert.deepEqual(
    [...declared].sort(),
    [
      "$schema",
      "agents",
      "author",
      "channels",
      "commands",
      "defaultEnabled",
      "dependencies",
      "description",
      "displayName",
      "experimental",
      "homepage",
      "hooks",
      "keywords",
      "license",
      "lspServers",
      "mcpServers",
      "monitors",
      "name",
      "outputStyles",
      "repository",
      "settings",
      "skills",
      "themes",
      "userConfig",
      "version",
    ],
  );

  // `themes`/`monitors` are declared *and* denied: the format recognizes them at the top
  // level and warns about the placement, so the deny-list clause carries that migration
  // and `closedKeys()` stays silent. Dropping them from the allow-list would have it call
  // a documented key unrecognized — a second finding, and a false one.
  const denied = pluginManifestDefaultContract.find((entry) => entry.predicate.key === "forbidden_keys");
  for (const key of denied?.predicate.keys ?? []) {
    assert.ok(declared.includes(key), `\`${key}\` is denied at the top level but still a recognized key`);
  }
});

test("marketplace is a json-document file kind at a glob its plugin-manifest sibling never contends for", () => {
  assert.deepEqual(marketplace.facts.locus, {
    kind: "at",
    root: ".claude-plugin",
    glob: "marketplace.json",
  });
  assert.equal(marketplace.facts.format, "json-document");
  // Identity from the document's own key: every catalog's stem is `marketplace`.
  assert.equal(marketplace.facts.unitShape, "named-field");
  assert.equal(marketplace.facts.identityField, "name");
  // It owns its file, exactly as its sibling does.
  assert.equal(marketplace.facts.collectionAddress, undefined);
  assert.equal(marketplace.facts.shape, undefined);
  // Channel-less: a catalog is read by the installer, never surfaced to the model.
  assert.deepEqual(marketplace.facts.registration, []);
  // The two `.claude-plugin` kinds share a root and are told apart by their globs, so a
  // manifest and a catalog never contend for the same file.
  assert.deepEqual(pluginManifest.facts.locus, {
    kind: "at",
    root: ".claude-plugin",
    glob: "plugin.json",
  });
});

test("marketplaceDefaultContract gates the reserved-names deny list and reaches below the top level", () => {
  // `name`'s presence, emptiness, charset and the reserved deny list, then the two
  // required objects and the rules *inside* them the addressing subset reaches. The one
  // rule still out of reach — the `source` union — is named in the contract's header and
  // in the clause an author meets it at, never forged into a clause.
  assert.deepEqual(
    marketplaceDefaultContract.map((entry) => entry.predicate.key),
    ["required", "min_len", "allowed_chars", "deny", "required", "required", "required", "required", "required"],
  );
  // Every clause is an error: each is a documented rule that stops a catalog loading.
  assert.ok(marketplaceDefaultContract.every((entry) => entry.severity === "required"));

  const [presence, empty, charset, reserved, owner, ownerName, plugins, entryName, entrySource] =
    marketplaceDefaultContract;
  assert.deepEqual(presence.predicate, { key: "required", field: "name" });
  assert.deepEqual(empty.predicate, { key: "min_len", field: "name", args: { min: 1 } });
  assert.deepEqual(charset.predicate, {
    key: "allowed_chars",
    field: "name",
    charset: { ranges: ["a-z", "0-9"], chars: "-" },
  });
  assert.deepEqual(owner.predicate, { key: "required", field: "owner" });
  assert.deepEqual(plugins.predicate, { key: "required", field: "plugins" });

  // The three the addressing subset bought: a name segment walks into `owner`, and `[*]`
  // grains over the catalog so each entry is judged on its own.
  assert.deepEqual(ownerName.predicate, { key: "required", field: "owner.name" });
  assert.deepEqual(entryName.predicate, { key: "required", field: "plugins[*].name" });
  assert.deepEqual(entrySource.predicate, { key: "required", field: "plugins[*].source" });

  // The deny list is the load-bearing clause: it is the documented reserved set entire,
  // transcribed from the page rather than sampled, and every name is kebab-case so each
  // is a value the charset clause above would otherwise pass.
  assert.equal(reserved.predicate.key, "deny");
  assert.equal(reserved.predicate.field, "name");
  assert.deepEqual(reserved.predicate.values, [
    "claude-code-marketplace",
    "claude-code-plugins",
    "claude-plugins-official",
    "claude-plugins-community",
    "claude-community",
    "anthropic-marketplace",
    "anthropic-plugins",
    "agent-skills",
    "anthropic-agent-skills",
    "knowledge-work-plugins",
    "life-sciences",
    "claude-for-legal",
    "claude-for-financial-services",
    "financial-services-plugins",
    "first-party-plugins",
    "healthcare",
  ]);
  // The impersonation rule is real and undecidable, so it can only ride the guidance —
  // a clause that guessed at it would fire on true negatives.
  assert.match(reserved.guidance ?? "", /impersonate/);
  // And the guidance carries why the clause outranks a lint: the list is re-checked on
  // every load, so a name that *becomes* reserved strands users who already added you.
  assert.match(reserved.guidance ?? "", /every load/);
  // The one rule still out of reach is named at the clause an author meets it at — the
  // hold is stated where it bites, not only in the module header.
  assert.match(entrySource.guidance ?? "", /union no clause can yet decide/);

  // Cited and dated, every one.
  for (const entry of marketplaceDefaultContract) {
    assert.match(
      entry.cite ?? "",
      /^https:\/\/code\.claude\.com\/docs\/en\/plugin-marketplaces#.* \(retrieved 2026-07-1[67]\)$/,
    );
  }
});

test("settings-local is a local-locus json-document file kind owning .claude/settings.local.json", () => {
  assert.deepEqual(settingsLocal.facts.locus, {
    kind: "at",
    root: ".claude",
    glob: "settings.local.json",
    commitment: "local",
  });
  // The whole-file JSON format routes it to the document reader, like plugin-manifest.
  assert.equal(settingsLocal.facts.format, "json-document");
  // A singleton at a fixed path: identity is the file stem, so no declared key names it.
  assert.equal(settingsLocal.facts.unitShape, "file");
  assert.equal(settingsLocal.facts.identityField, undefined);
  // It owns its file rather than surfacing inside a manifest, and reaches the model on no
  // channel of its own — machine configuration read by the harness.
  assert.equal(settingsLocal.facts.collectionAddress, undefined);
  assert.equal(settingsLocal.facts.shape, undefined);
  assert.deepEqual(settingsLocal.facts.registration, []);
});

test("settingsLocalDefaultContract types the structural container keys and leaves the rest opaque", () => {
  // Near-empty by design: the residue stays opaque (0036), so no closed-keys clause — only
  // the three documented object-valued keys are gated, each as a `map`.
  assert.deepEqual(
    settingsLocalDefaultContract.map((c) => c.predicate.key),
    ["type", "type", "type"],
  );
  assert.deepEqual(
    settingsLocalDefaultContract.map((c) => [c.predicate.field, c.predicate.value_type]),
    [
      ["permissions", ["map"]],
      ["env", ["map"]],
      ["hooks", ["map"]],
    ],
  );
  assert.ok(settingsLocalDefaultContract.every((c) => c.severity === "required"));
  // A hook registered here is opaque residue, never a modeled member — the guidance says so.
  const hooks = settingsLocalDefaultContract.find((c) => c.predicate.field === "hooks");
  assert.match(hooks?.guidance ?? "", /opaque residue/);
  // Cited and dated, every one — to the live settings docs.
  for (const entry of settingsLocalDefaultContract) {
    assert.match(entry.cite ?? "", /^https:\/\/code\.claude\.com\/docs\/en\/settings#\S+ \(retrieved 2026-07-16\)$/);
  }
});

test("command is a file-shaped unit with no identityField, unlike the directory-shaped skill", () => {
  assert.equal(command.facts.unitShape, "file");
  assert.equal(command.facts.identityField, undefined);
  assert.equal(skill.facts.unitShape, "directory");
  assert.equal(skill.facts.identityField, "name");
});

test("agent is a named-field unit whose identity comes from its own name field", () => {
  assert.equal(agent.facts.unitShape, "named-field");
  assert.equal(agent.facts.identityField, "name");
  assert.equal(agent.facts.format, "yaml-frontmatter");
  assert.deepEqual(agent.facts.locus, { kind: "at", root: ".claude/agents", glob: "**/*.md" });
});

test("skill/command register on both documented invocation channels; agent/rule/memory carry a singleton set", () => {
  assert.deepEqual(skill.facts.registration, [
    { via: "user-invoked" },
    { via: "description-trigger", field: "description" },
  ]);
  assert.deepEqual(command.facts.registration, [
    { via: "user-invoked" },
    { via: "description-trigger", field: "description" },
  ]);
  assert.deepEqual(agent.facts.registration, [{ via: "description-trigger", field: "description" }]);
  assert.deepEqual(rule.facts.registration, [{ via: "paths-match", field: "paths" }]);
  assert.deepEqual(memory.facts.registration, [{ via: "always" }]);
});

test("agentDefaultContract requires name and description, gates the lowercase-hyphen charset, and pins per-scope uniqueness", () => {
  assert.deepEqual(
    agentDefaultContract.map((c) => c.predicate.key),
    ["required", "allowed_chars", "unique-name", "required"],
  );
  assert.deepEqual(
    agentDefaultContract.map((c) => c.predicate.field),
    ["name", "name", undefined, "description"],
  );
  const charset = agentDefaultContract[1].predicate.charset;
  assert.deepEqual(charset, { ranges: ["a-z"], chars: "-" });
});

test("an agent member's identity field writes name first, then the typed description", () => {
  const member = agent({
    name: "code-reviewer",
    description: "Use when reviewing a pull request for correctness.",
  });
  assert.deepEqual(member.fields, [
    ["name", "code-reviewer"],
    ["description", "Use when reviewing a pull request for correctness."],
  ]);
});

test("disable-model-invocation/user-invocable/paths are ordinary declared fields on a skill member", () => {
  const member = skill({
    name: "demo",
    description: "Use when demonstrating a skill's modulating fields.",
    "disable-model-invocation": true,
    "user-invocable": false,
    paths: ["src/**"],
  });
  assert.deepEqual(member.fields, [
    ["name", "demo"],
    ["description", "Use when demonstrating a skill's modulating fields."],
    ["disable-model-invocation", true],
    ["user-invocable", false],
    ["paths", ["src/**"]],
  ]);
  // paths gates the existing invocation channels, so it adds no registration
  // channel of its own — unlike a rule's paths-match.
  assert.deepEqual(
    skill.facts.registration,
    [{ via: "user-invoked" }, { via: "description-trigger", field: "description" }],
  );
});
