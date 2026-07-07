/**
 * Needs — the capabilities a member's behavior uses, declared as typed values.
 * Emit derives
 * the settings permission list from their union, so a permission is never authored
 * twice: `permissions.allow` is the union of the members' declared `needs`, and a
 * permission with no member is visible as exactly that.
 */

/** A declared capability — its `permission` is the entry it derives in the union. */
export interface Capability {
  /**
   * The permission-list entry this capability derives. The union of every
   * member's needs is the settings `permissions.allow` — the fold hooks and MCP
 * members ride into once those kinds land.
   */
  readonly permission: string;
}

/**
 * A shell-command capability. Its derived permission is the Claude Code allow
 * entry `Bash(<command>)` (code.claude.com/docs/en/settings, retrieved
 * 2026-07-04) — e.g. `bash("git diff")`.
 */
export function bash(command: string): Capability {
  return { permission: `Bash(${command})` };
}

/** Any capability whose permission entry the author states verbatim. */
export function capability(permission: string): Capability {
  return { permission };
}

/**
 * The derived permission list — the union of every capability's entry, deduped
 * and sorted so the derived artifact is byte-stable across runs.
 * The
 * permission is derived here, never authored.
 */
export function permissionUnion(needs: readonly Capability[]): string[] {
  return [...new Set(needs.map((need) => need.permission))].sort();
}
