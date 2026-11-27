/**
 * Needs — the capabilities a member's behavior uses, declared as typed values.
 * Emit derives the settings permission lists from their union, so a permission is never
 * authored twice: `permissions.allow` and `permissions.ask` are the unions of the
 * members' declared `needs`, and a permission with no member is visible as exactly that.
 */

/**
 * The settings permission list a capability's entry joins: `"allow"` runs the tool
 * without a prompt, `"ask"` prompts for confirmation even where an allow rule also
 * matches — rules evaluate deny, then ask, then allow (code.claude.com/docs/en/permissions,
 * "Manage permissions", retrieved 2026-11-26).
 */
export type PermissionList = "allow" | "ask";

/** A declared capability — its `permission` is the entry it derives in the union. */
export interface Capability {
  /** The permission-rule entry this capability derives. */
  readonly permission: string;
  /** The list the entry joins; absent, `"allow"`. */
  readonly list?: PermissionList;
}

/**
 * A shell-command capability. Its derived permission is the Claude Code allow
 * entry `Bash(<command>)` (code.claude.com/docs/en/settings, retrieved
 * 2026-07-04) — e.g. `bash("git diff")`.
 */
export function bash(command: string): Capability {
  return { permission: `Bash(${command})` };
}

/** Any capability whose permission entry the author states verbatim, on `list` (default `"allow"`). */
export function capability(permission: string, opts: { list?: PermissionList } = {}): Capability {
  return opts.list === undefined || opts.list === "allow" ? { permission } : { permission, list: opts.list };
}

/** The derived settings permission lists — each the deduped, sorted union of its entries. */
export interface PermissionLists {
  readonly allow: readonly string[];
  readonly ask: readonly string[];
}

/**
 * The derived permission lists — the union of every capability's entry on its list,
 * deduped and sorted so the derived artifact is byte-stable across runs. The
 * permission is derived here, never authored.
 */
export function permissionUnion(needs: readonly Capability[]): PermissionLists {
  const union = (list: PermissionList) =>
    [...new Set(needs.filter((need) => (need.list ?? "allow") === list).map((need) => need.permission))].sort();
  return { allow: union("allow"), ask: union("ask") };
}
