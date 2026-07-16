/**
 * The package root's export face, held mechanically rather than by a hand-list.
 *
 * A hand-list re-rots the next time a type is added; so does an enumerated import
 * test, which only moves the trap one level down — an exported signature naming an
 * unexported type still type-checks. So the assertion walks the emitted
 * `dist/src/index.d.ts` instead: every type a root-exported declaration names must
 * itself be root-exported. `pnpm --dir sdk test` runs `tsc -p` before `node --test`,
 * so the `.d.ts` is on disk by the time this file runs.
 *
 * The seam's generated row family is out of scope here and excluded by declaration
 * file. The rows are internal, versioned in lockstep with the engine, and reach the
 * face only by inference through `EmitResult` — nameability was never the boundary,
 * the stability promise the root never made is. A row an author can infer is not a
 * row the root owes an export.
 */
import assert from "node:assert/strict";
import { dirname, resolve } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import ts from "typescript";

const DIST_SRC = resolve(dirname(fileURLToPath(import.meta.url)), "../src");
const INDEX_DTS = resolve(DIST_SRC, "index.d.ts");
const GENERATED = resolve(DIST_SRC, "generated");

/** The root module's symbol plus the checker that resolves references out of it. */
function rootModule(): { checker: ts.TypeChecker; symbol: ts.Symbol } {
  const program = ts.createProgram([INDEX_DTS], {
    module: ts.ModuleKind.NodeNext,
    moduleResolution: ts.ModuleResolutionKind.NodeNext,
    target: ts.ScriptTarget.ES2022,
    skipLibCheck: true,
  });
  const checker = program.getTypeChecker();
  const source = program.getSourceFile(INDEX_DTS);
  assert.ok(source, `${INDEX_DTS} should exist — \`tsc -p\` runs before \`node --test\``);
  const symbol = checker.getSymbolAtLocation(source);
  assert.ok(symbol, "the root's emitted declaration file should resolve as a module");
  return { checker, symbol };
}

/** Follow an export alias to the declaration it ultimately names. */
function target(checker: ts.TypeChecker, symbol: ts.Symbol): ts.Symbol {
  return symbol.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(symbol) : symbol;
}

/**
 * Every type reference under a declaration, by the *symbol* each names — a bare
 * identifier match would confuse a type parameter `T` for a type named `T`.
 */
function referencedTypes(checker: ts.TypeChecker, declaration: ts.Node): ts.Symbol[] {
  const found: ts.Symbol[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isTypeReferenceNode(node)) {
      const named = ts.isIdentifier(node.typeName) ? node.typeName : node.typeName.right;
      const symbol = checker.getSymbolAtLocation(named);
      if (symbol) found.push(target(checker, symbol));
    }
    ts.forEachChild(node, visit);
  };
  ts.forEachChild(declaration, visit);
  return found;
}

/**
 * Whether a referenced symbol is this package's own authoring vocabulary — the
 * only thing the root owes an export. A type parameter, a `lib.d.ts` built-in, a
 * `@types/node` type and the excluded generated row family each answer no.
 */
function isAuthoringNoun(symbol: ts.Symbol): boolean {
  const declarations = symbol.getDeclarations() ?? [];
  if (declarations.length === 0) return false;
  if (declarations.some((d) => ts.isTypeParameterDeclaration(d))) return false;
  return declarations.some((d) => {
    const path = resolve(d.getSourceFile().fileName);
    return path.startsWith(`${DIST_SRC}/`) && !path.startsWith(`${GENERATED}/`);
  });
}

test("every type a root-exported declaration names is itself root-exported", () => {
  const { checker, symbol } = rootModule();
  const exported = checker.getExportsOfModule(symbol);
  const face = new Set(exported.map((e) => target(checker, e)));

  const holes: string[] = [];
  for (const entry of exported) {
    for (const declaration of target(checker, entry).getDeclarations() ?? []) {
      for (const named of referencedTypes(checker, declaration)) {
        if (!isAuthoringNoun(named) || face.has(named)) continue;
        holes.push(`\`${entry.getName()}\` names \`${named.getName()}\`, which the root never exports`);
      }
    }
  }

  assert.deepEqual(
    [...new Set(holes)].sort(),
    [],
    "a type an author cannot import defeats a surface typed at the keystroke",
  );
});

test("the root carries the whole closed predicate vocabulary", () => {
  const { checker, symbol } = rootModule();
  const face = new Set(checker.getExportsOfModule(symbol).map((e) => e.getName()));

  // The enum in code is the authority; a vocabulary 23/24 wide silently disagrees
  // with it, so the walk is over `contract.ts`'s own constructors, not a copy of them.
  const contract = resolve(DIST_SRC, "contract.d.ts");
  const program = ts.createProgram([contract], { skipLibCheck: true });
  const source = program.getSourceFile(contract);
  assert.ok(source, `${contract} should exist`);
  const contractSymbol = program.getTypeChecker().getSymbolAtLocation(source);
  assert.ok(contractSymbol, "contract.d.ts should resolve as a module");

  const predicates = program
    .getTypeChecker()
    .getExportsOfModule(contractSymbol)
    .filter((e) =>
      (e.getDeclarations() ?? []).some(
        (d) => ts.isVariableDeclaration(d) && d.type !== undefined && /\bPredicate\b/.test(d.type.getText()),
      ),
    )
    .map((e) => e.getName());

  assert.ok(predicates.includes("globValid"), "the walk should reach `globValid` itself");
  assert.deepEqual(
    predicates.filter((p) => !face.has(p)).sort(),
    [],
    "an author spells `clause(globValid(...), ...)` over their own kind off the root alone",
  );
});
