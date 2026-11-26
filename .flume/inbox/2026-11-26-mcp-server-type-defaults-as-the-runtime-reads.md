# An MCP server entry with no `type` reads as `stdio`, as Claude Code reads it (0101)

Answers the `mcp-server-absent-transport-command` question under class 0101 (a built-in reads as the runtime reads) and class 0103 (a finding is true for its reader). Claude Code reads an absent `type` as `stdio` (code.claude.com/docs/en/mcp, retrieved 2026-07-15, cited at `McpServer` in `sdk/src/builtins.ts`). An entry with only a `url` is therefore read as stdio and skipped. temper's guard `when(enumOf("type", ["stdio"]), [required("command")])` is silent on the absent field, so that entry checks green.

The fix the class entails needs no amendment to guard semantics. The format declares `type`'s documented default, the guard reads the field as the runtime does, `required("command")` fires on a `url`-only entry, and the clause's guidance claims exactly that. Acceptance: `{"mcpServers":{"x":{"url":"…"}}}` draws the stdio `command` finding, and an explicit `type: "http"` with a `url` passes.
