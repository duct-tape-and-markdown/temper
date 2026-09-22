// Plan-prompt input: for every pickable entry, the paths its own files[]
// omits but its named symbols reach. Information to plan, never a gate —
// plan decides whether a hit is a real consumer (widen files[]) or noise.
// Symbols are the backticked identifiers in files[].description, summary,
// and acceptance; a hit is any src/, tests/, sdk/, or examples/ file containing the
// identifier as a whole word that files[] does not already list. examples/
// is walked too: it holds live in-tree consumers of the SDK's authoring surface.
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
const raw = JSON.parse(readFileSync(".flume/plan/pending.json", "utf8"));
const entries = Array.isArray(raw) ? raw : Object.values(raw).find(Array.isArray) ?? [];
const files = [];
const walk = (d) => { for (const n of readdirSync(d)) { const p = join(d, n); const s = statSync(p); if (s.isDirectory()) { if (n !== "node_modules" && n !== "dist" && n !== "target") walk(p); } else if (/\.(rs|ts|toml|snap)$/.test(n)) files.push(p); } };
for (const d of ["src", "tests", "sdk/src", "sdk/test", "examples"]) { try { walk(d); } catch {} }
const text = new Map(files.map((f) => [f, readFileSync(f, "utf8")]));
// The files among `where` holding an exhaustive `Name { … }` struct literal: no
// `..` spread at the literal's own depth, and not a declaration, impl block, or
// return type. These are the sites a new field breaks.
const exhaustiveLiterals = (name, where) => where.filter((f) => {
  if (!f.endsWith(".rs")) return false;
  const t = text.get(f);
  const re = new RegExp(`(^|[^\\w])(impl|struct|enum|for|fn|trait)?\\s*\\b${name}\\s*\\{`, "g");
  let m;
  while ((m = re.exec(t))) {
    if (m[2]) continue;
    const lineStart = t.lastIndexOf("\n", m.index) + 1;
    if (t.slice(lineStart, m.index + m[0].length).includes("->")) continue;
    let depth = 1, spread = false;
    for (let i = re.lastIndex; i < t.length && depth > 0; i++) {
      const c = t[i];
      if (c === "{") depth++;
      else if (c === "}") depth--;
      else if (depth === 1 && c === "." && t[i + 1] === ".") spread = true;
    }
    if (!spread) return true;
  }
  return false;
});
const STOP = new Set(["src", "tests", "sdk", "true", "false", "None", "Some", "Ok", "Err", "String", "Vec", "Option", "Result", "self", "Self", "kind", "name", "path", "field", "key", "host", "check", "emit", "guard", "explain", "install", "temper", "rust", "tsc", "cargo", "insta"]);
let out = "";
for (const e of entries) {
  const g = e.gate?.kind ?? "";
  if (g !== "open" && g !== "blockedBy") continue;
  const declared = new Set([...(e.files?.edit ?? []), ...(e.files?.new ?? [])].map((f) => f.path));
  const blobs = [e.summary ?? "", e.acceptance ?? "", ...(e.files?.edit ?? []).map((f) => f.description ?? "")].join("\n");
  // Code-shaped only: snake_case, a `::`/`.` path's last segment, CamelCase
  // with two humps, or a call form — never a plain word, which is vocabulary.
  const shaped = (m) => /_/.test(m) || /::|\./.test(m) || /\(\)$/.test(m) || /[a-z][A-Z].*[a-z][A-Z]|^[A-Z][a-z]+[A-Z]/.test(m);
  const ids = new Set([...blobs.matchAll(/`([A-Za-z_][A-Za-z0-9_:.]*(?:\(\))?)`/g)].map((m) => m[1]).filter(shaped).map((m) => m.replace(/\(\)$/, "").split(/[:.]/).pop()).filter((s) => s.length >= 5 && !STOP.has(s)));
  const hits = new Map();
  for (const id of ids) {
    const re = new RegExp(`\\b${id.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\b`);
    const where = [...text].filter(([f, t]) => re.test(t)).map(([f]) => f);
    if (where.length > 10) {
      // Too common to list every site: vocabulary, or a widely used type. For a
      // type, the sites a new column breaks are its exhaustive literals (no
      // `..` spread), which stay few, so report those instead of dropping it.
      if (/^[A-Z][a-z]/.test(id)) {
        for (const f of exhaustiveLiterals(id, where)) if (!declared.has(f)) hits.set(f, [...(hits.get(f) ?? []), `${id} {…} (exhaustive literal)`]);
      }
      continue;
    }
    for (const f of where) if (!declared.has(f)) hits.set(f, [...(hits.get(f) ?? []), id]);
  }
  if (hits.size === 0) continue;
  out += `== ${e.tag} (files[] lists ${declared.size})\n`;
  for (const [f, syms] of [...hits].sort()) out += `  ${f}  ← ${[...new Set(syms)].slice(0, 4).join(", ")}\n`;
}
process.stdout.write(out || "(no ripple: every named symbol's match sites are inside files[])\n");
