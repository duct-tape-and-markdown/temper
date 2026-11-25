import { test } from "node:test";
import assert from "node:assert/strict";
import { carries, fileOf, lineRegex, parseName, readEvents, squash, verdicts } from "./runner.mjs";

const onDisk = (paths) => (p) => paths.includes(p.replace(/^\.\//, ""));

test("a line and a snake_case name squash to the same letters", () => {
  assert.equal(squash("a layout's frontmatter reads"), squash("a_layouts_frontmatter_reads"));
  assert.equal(squash("a layout's frontmatter reads"), squash("a_layout_s_frontmatter_reads"));
});

test("the filter regex matches the name the verdict accepts", () => {
  const re = new RegExp(lineRegex("a layout's frontmatter reads"));
  assert.ok(re.test("temper::it$layout_kind::a_layouts_frontmatter_reads"));
  assert.ok(re.test("temper::it$layout_kind::a_layout_s_frontmatter_reads"));
  assert.ok(!re.test("temper::it$layout_kind::a_layout_reads"));
});

test("a libtest-json name splits into binary and module path", () => {
  assert.deepEqual(parseName("temper::it$acceptance::a_case"), { binary: "temper::it", path: ["acceptance", "a_case"] });
  assert.deepEqual(parseName("temper::temper$import::tests::x"), { binary: "temper::temper", path: ["import", "tests", "x"] });
});

test("an integration test maps to its module file under tests/it", () => {
  const exists = onDisk(["tests/it/acceptance.rs", "tests/it/common/mod.rs"]);
  assert.equal(fileOf("temper::it", ["acceptance", "a_case"], exists), "tests/it/acceptance.rs");
  assert.equal(fileOf("temper::it", ["common", "a_case"], exists), "tests/it/common/mod.rs");
  assert.equal(fileOf("temper::it", ["a_case"], exists), "tests/it/main.rs");
});

test("a lib test maps to its flat src module, past the tests submodule", () => {
  const exists = onDisk(["src/import.rs"]);
  assert.equal(fileOf("temper::temper", ["import", "tests", "x"], exists), "src/import.rs");
  assert.equal(fileOf("temper::temper", ["tests", "x"], exists), "src/lib.rs");
});

test("only ok events count as passing", () => {
  const out = [
    `{"type":"test","event":"ok","name":"temper::it$a::passes"}`,
    `{"type":"test","event":"failed","name":"temper::it$a::fails"}`,
    `{"type":"test","event":"ignored","name":"temper::it$a::skipped"}`,
    "not json",
  ].join("\n");
  const { passed, ignored } = readEvents(out);
  assert.deepEqual(passed, ["temper::it$a::passes"]);
  assert.deepEqual(ignored, ["temper::it$a::skipped"]);
});

test("every requested line is answered once, pass with its file or fail with none", () => {
  const exists = onDisk(["tests/it/a.rs"]);
  const lines = verdicts(["passes", "fails", ""], ["temper::it$a::passes"], exists);
  assert.equal(lines.length, 3);
  assert.equal(lines[0], "flume\tpass\ttests/it/a.rs\tpasses");
  assert.equal(lines[1], "flume\tfail\t\tfails");
  assert.equal(lines[2], "flume\tfail\t\t");
});

test("a name may spell out punctuation an identifier cannot hold", () => {
  const line = "the pre edge names an undeclared CLAUDE.md written under a `.`-rooted locus";
  const fn = "the_pre_edge_names_an_undeclared_claude_md_written_under_a_dot_rooted_locus";
  assert.ok(carries(fn, line));
  assert.ok(new RegExp(lineRegex(line)).test(`temper::it$install::${fn}`));
});

test("words out of order or missing do not carry a line", () => {
  assert.ok(!carries("a_rooted_dot_locus_under", "under a `.`-rooted locus"));
  assert.ok(!carries("the_pre_edge_names_a_document", "the pre edge names an undeclared document"));
  assert.ok(!carries("anything", ""));
});
