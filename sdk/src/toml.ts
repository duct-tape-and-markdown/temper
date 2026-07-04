/**
 * TOML value/key encoding and table layout — a faithful port of `toml_write`
 * 0.1.2 (`src/string.rs`) and `toml_edit` 0.22.27's `visit_table`
 * (`specs/architecture/20-surface.md`, "Content-faithful, deterministically
 * emitted (law 5)"). Shared by the manifest emitter (`emit.ts`) and the lock
 * stamper (`lock.ts`) so every `key = value` line and every table header the SDK
 * writes is byte-identical to the Rust `toml_edit` output — the manifest, the
 * projection frontmatter, and the lock all agree to the byte.
 */

type Encoding = "literal" | "basic" | "mlliteral" | "mlbasic";

interface StringMetrics {
  readonly maxSingle: number;
  readonly maxDouble: number;
  readonly escapeCodes: boolean;
  readonly escape: boolean;
  readonly newline: boolean;
}

/** `ValueMetrics::calculate` — the run-length and escape facts style choice reads. */
function stringMetrics(s: string): StringMetrics {
  let maxSingle = 0;
  let maxDouble = 0;
  let escapeCodes = false;
  let escape = false;
  let newline = false;
  let prevSingle = 0;
  let prevDouble = 0;
  for (const ch of s) {
    const cp = ch.codePointAt(0)!;
    if (cp === 0x27) {
      prevSingle += 1;
      maxSingle = Math.max(maxSingle, prevSingle);
    } else {
      prevSingle = 0;
    }
    if (cp === 0x22) {
      prevDouble += 1;
      maxDouble = Math.max(maxDouble, prevDouble);
    } else {
      prevDouble = 0;
    }
    // The arm order mirrors the Rust match: `\` then `\t` (allowed) then `\n`
    // then the general control range.
    if (cp === 0x5c) escape = true;
    else if (cp === 0x09) {
      /* horizontal tab is always allowed — neutral */
    } else if (cp === 0x0a) newline = true;
    else if (cp <= 0x1f || cp === 0x7f) escapeCodes = true;
  }
  return { maxSingle, maxDouble, escapeCodes, escape, newline };
}

/** `TomlStringBuilder::as_default` — the fall-through style preference. */
function chooseEncoding(m: StringMetrics): Encoding {
  // as_basic_pretty
  if (!(m.escapeCodes || m.escape || m.maxDouble > 0 || m.newline)) return "basic";
  // as_literal
  if (!(m.escapeCodes || m.maxSingle > 0 || m.newline)) return "literal";
  // as_ml_basic_pretty
  if (!(m.escapeCodes || m.escape || m.maxDouble > 2)) return "mlbasic";
  // as_ml_literal
  if (!(m.escapeCodes || m.maxSingle > 2)) return "mlliteral";
  // fallback: the escaped forms
  return m.newline ? "mlbasic" : "basic";
}

/** The basic/multiline-basic escaper from `write_toml_value` (the `escaped` branch). */
function escapeBasic(s: string, isMl: boolean): string {
  const maxSeqDouble = isMl ? 2 : 0;
  let out = "";
  let seqDouble = 0;
  for (const ch of s) {
    const cp = ch.codePointAt(0)!;
    if (cp === 0x22) {
      seqDouble += 1;
      if (seqDouble > maxSeqDouble) {
        out += '\\"';
        seqDouble = 0;
        continue;
      }
      out += '"';
      continue;
    }
    seqDouble = 0;
    switch (cp) {
      case 0x08:
        out += "\\b";
        break;
      case 0x09:
        out += "\\t";
        break;
      case 0x0a:
        // A literal newline survives inside a multiline string; a basic string
        // escapes it.
        out += isMl ? "\n" : "\\n";
        break;
      case 0x0c:
        out += "\\f";
        break;
      case 0x0d:
        out += "\\r";
        break;
      case 0x5c:
        out += "\\\\";
        break;
      default:
        if (cp <= 0x1f || cp === 0x7f) {
          out += "\\u" + cp.toString(16).toUpperCase().padStart(4, "0");
        } else {
          out += ch;
        }
    }
  }
  return out;
}

/** A TOML string *value* — the exact bytes `toml_edit`'s `value(String)` emits. */
export function encodeString(s: string): string {
  const m = stringMetrics(s);
  const enc = chooseEncoding(m);
  const delimiter =
    enc === "literal" ? "'" : enc === "basic" ? '"' : enc === "mlliteral" ? "'''" : '"""';
  const isMl = enc === "mlliteral" || enc === "mlbasic";
  const escaped = enc === "basic" || enc === "mlbasic";
  let out = delimiter;
  if (m.newline && isMl) out += "\n";
  out += escaped ? escapeBasic(s, isMl) : s;
  out += delimiter;
  return out;
}

interface KeyMetrics {
  readonly unquoted: boolean;
  readonly single: boolean;
  readonly double: boolean;
  readonly escapeCodes: boolean;
  readonly escape: boolean;
}

/** `KeyMetrics::calculate` — whether a key may be bare, and its escape facts. */
function keyMetrics(s: string): KeyMetrics {
  let unquoted = s.length > 0;
  let single = false;
  let double = false;
  let escapeCodes = false;
  let escape = false;
  for (const ch of s) {
    const cp = ch.codePointAt(0)!;
    const wordByte =
      (cp >= 0x61 && cp <= 0x7a) ||
      (cp >= 0x41 && cp <= 0x5a) ||
      (cp >= 0x30 && cp <= 0x39) ||
      cp === 0x2d ||
      cp === 0x5f;
    if (!wordByte) unquoted = false;
    if (cp === 0x27) single = true;
    else if (cp === 0x22) double = true;
    else if (cp === 0x5c) escape = true;
    else if (cp === 0x09) {
      /* tab allowed */
    } else if (cp <= 0x1f || cp === 0x7f) escapeCodes = true;
  }
  return { unquoted, single, double, escapeCodes, escape };
}

/** A TOML *key* — bare where it can be, else `toml_edit`'s `TomlKeyBuilder::as_default`. */
export function encodeKey(s: string): string {
  const m = keyMetrics(s);
  if (m.unquoted) return s;
  // as_basic_pretty
  if (!(m.escapeCodes || m.escape || m.double)) return '"' + escapeBasic(s, false) + '"';
  // as_literal
  if (!(m.escapeCodes || m.single)) return "'" + s + "'";
  // as_basic (fallback)
  return '"' + escapeBasic(s, false) + '"';
}

/** One `key = value\n` line, the key/value decor `toml_edit` renders (`key = value`). */
export function keyValue(key: string, valueRepr: string): string {
  return `${encodeKey(key)} = ${valueRepr}\n`;
}

/** A TOML string array — `["a", "b"]`, no leading space, `, ` between elements. */
export function stringArray(values: readonly string[]): string {
  return "[" + values.map(encodeString).join(", ") + "]";
}

/** Sorted keys — the stable order `toml_edit` gets for free from its `BTreeMap`s. */
export function sortedKeys(record: Readonly<Record<string, unknown>>): string[] {
  return Object.keys(record).sort();
}

/**
 * Join an ordered list of table sections the `toml_edit` way — exactly one blank
 * line before every table header but the document's first
 * (`DEFAULT_TABLE_DECOR = ("\n", "")`, the first table `("", …)`). Each section is
 * a header line plus its `key = value\n` lines, already newline-terminated.
 */
export function joinSections(sections: readonly string[]): string {
  return sections.map((section, i) => (i === 0 ? "" : "\n") + section).join("");
}
