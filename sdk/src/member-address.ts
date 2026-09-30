/**
 * The member-address grammar — one home for every spelling a member's identity takes and
 * for the reader those writers round-trip against. Three spellings, each the one before
 * it plus a segment:
 *
 * - `<kind>:<name>` — a **host address**, a top-level member's own identity.
 * - `<host-address>/<kind>/<key>` — a **nested-member address**, the identity a member
 *   embedded in a host carries.
 * - `<member>/<kind>/<key>/<leaf>` — a **leaf address**, one authored string beneath a
 *   nested member. A grain of its own, and no member address at all.
 *
 * Because the third is the second plus a `/<leaf>` tail, both are read off one
 * segmentation ({@link segment}) rather than two independently-shaped parses that could
 * come to disagree about where the member ends. The engine's `src/member_address.rs` is
 * the other end of the same grammar: a spelling moves on both sides at once or the seam
 * breaks.
 */

/** The two halves a host address names — the reader's answer, and never re-split by hand. */
export interface HostAddress {
  /** The member's kind. */
  readonly kind: string;
  /** The member's name among its kind. */
  readonly name: string;
}

/** One parsed nested-member address. */
export interface NestedAddress {
  /** The host member's own `<kind>:<name>` address — the segment before the first `/`. */
  readonly host: string;
  /** The nested member's kind. */
  readonly kind: string;
  /** The nested member's key among its host's members of that kind. */
  readonly key: string;
}

/** One parsed leaf address — a nested-member address plus its `/<leaf>` tail. */
export interface LeafAddress {
  /**
   * The member the leaf lives under, verbatim as its author spelled it: the canonical
   * `<kind>:<name>` host address, or the bare member id this SDK's own leaf writer and
   * the committed lock mention targets spell. Which of the two a head is, is resolution's
   * question, not the grammar's.
   */
  readonly member: string;
  /** The nested member's kind. */
  readonly kind: string;
  /** The nested member's key among its host's members of that kind. */
  readonly key: string;
  /** The leaf's path within the nested member — the whole remainder after the third slash. */
  readonly childPath: string;
}

/** Spell a top-level member's own `<kind>:<name>` address. */
export function hostAddress(kind: string, name: string): string {
  return `${kind}:${name}`;
}

/** Spell a nested member's address from its host's address, its own kind and its key. */
export function nestedAddress(host: string, kind: string, key: string): string {
  return `${host}/${kind}/${key}`;
}

/**
 * The three facts a member's own address composes from — its kind, its name, and the host
 * member a nested-file child sits under. Structurally what a composed `Member` carries, so
 * {@link memberAddress} takes one without this module depending on `kind.ts`: the grammar
 * is the bottom of the tree and owes the member model nothing.
 */
export interface AddressedMember {
  /** The member's kind. */
  readonly kind: string;
  /** The member's name among its kind — its key where it sits beneath a host. */
  readonly name: string;
  /** The host a nested-file child composes under; absent at every other locus. */
  readonly host?: { readonly kind: string; readonly name: string } | undefined;
}

/**
 * A member's **own address** — the one home for the discrimination between the two forms a
 * composed member's identity takes, and the twin of the engine's `payload_member_address`
 * over the same member arriving on the seam ({@link AddressedMember.host} is the payload's
 * own `host` column).
 *
 * A top-level member's address is its kind joined onto its name ({@link hostAddress}). A
 * nested **file** child's identity *is* its whole `<host-address>/<kind>/<key>` address,
 * exactly as an embedded member's is: the host segment is the whole of what tells two
 * same-named children under different hosts apart, so two hosts may each carry a `home`
 * and neither claims the other's. Keying one by `<kind>:<name>` alone made a child's name
 * corpus-wide at a grain the grammar scopes to a host.
 *
 * The host segment is a **host** address, never a nested one: a nested-file child composes
 * under a host that owns a directory unit at an `at` locus, which a nested-file kind never
 * does — both faces refuse a deeper host before an address is spelled (`emit.ts`'s
 * `hostUnit`, the engine's `nested_file_path`) — so one level is the whole grammar here.
 *
 * Every site naming "the address of this member" reads the rule here rather than deciding
 * the join again, so the rows a member is declared, cited and mentioned under cannot come
 * to disagree with the address it resolves at.
 */
export function memberAddress(member: AddressedMember): string {
  const { host } = member;
  if (host === undefined) return hostAddress(member.kind, member.name);
  return nestedAddress(hostAddress(host.kind, host.name), member.kind, member.name);
}

/**
 * Whether `spelling` is exactly **one segment** of this grammar — non-empty, and carrying
 * none of the `/` {@link segment} cuts an address at.
 *
 * The predicate every writer's caller judges a key by, because {@link nestedAddress} is
 * infallible and the reader beneath it is not. A key carrying a `/` shifts every segment
 * below it by one: `nestedAddress("spec:alpha", "decision", "authority/rejected")` spells
 * the very address the member keyed `authority` spells for its `rejected` leaf, and every
 * reader tries leaf grain first — so the member's own identity answers its sibling's leaf.
 * An empty key spells an address {@link segment} admits at no grain at all.
 * `specs/model/representation.md` ("member") makes both a malformed lock rather than a
 * precedence rule: resolution is total, and coincident addresses are refused.
 *
 * The engine's `is_one_segment` is the same predicate at the seam's other end.
 */
export function isOneSegment(spelling: string): boolean {
  return spelling !== "" && !spelling.includes("/");
}

/**
 * Whether `spelling` may be joined onto a member's name as a **qualifier** — the text a
 * name carries after its own `:` to tell two members apart that share a collection key (a
 * hook's matcher: `PostToolUse:Edit|Write`).
 *
 * The bar is {@link isOneSegment}'s, less its non-emptiness: a qualifier joins a non-empty
 * key, so the joined name is non-empty whatever the qualifier is, and an **empty**
 * qualifier is a real authored spelling — a group whose `matcher` is `""` is its own group
 * on the wire, and a matcher is carried verbatim, never normalized. What it may not carry
 * is the `/` {@link segment} cuts an address at: the joined name is the first segment of
 * every address beneath the member, so a `/` in it shifts every segment below by one and
 * the member's own handler answers to an address naming no member at all.
 *
 * The engine's `is_name_qualifier` is the same predicate at the seam's other end.
 */
export function isNameQualifier(spelling: string): boolean {
  return !spelling.includes("/");
}

/**
 * Spell one leaf's address beneath a nested member. `member` is carried verbatim, so a
 * writer holding the bare member id spells the short form the lock already commits
 * ({@link LeafAddress.member}).
 */
export function leafAddress(member: string, kind: string, key: string, childPath: string): string {
  return `${nestedAddress(member, kind, key)}/${childPath}`;
}

/**
 * The member-table **lookup key** a bare `<kind>:<key>` reference spells — shaped like a
 * host address and never one: a nested member's address composes through its host, and a
 * corpus-unique key was rejected as the grammar because uniqueness is the resolver's bar,
 * not the grammar's. The engine strips this prefix back off before matching, so the two
 * ends agree and the apparent mismatch is not a defect to fix.
 */
export function bareLookupKey(kind: string, key: string): string {
  return `${kind}:${key}`;
}

/**
 * The key an authored edge address is looked up under: a one-element `to` set resolves a
 * bare address within its one admissible kind, so an unqualified address is lifted into a
 * {@link bareLookupKey}; an address already carrying a colon is the kind-qualified form
 * the author wrote and stands as it is.
 */
export function edgeLookupKey(address: string, to: readonly string[]): string {
  return to.length === 1 && !address.includes(":") ? bareLookupKey(to[0], address) : address;
}

/**
 * The `(kind, name)` a host address spells, or `undefined` when it spells none — the
 * reader half of {@link hostAddress}. Both halves are non-empty: an address names exactly
 * one thing or it names nothing. Splits at the **first** colon, so a name carrying one
 * stays whole.
 *
 * A `/` anywhere is this grammar's own segment separator ({@link segment} splits on it),
 * so an address carrying one is a segmented spelling with its own reader
 * ({@link parseNestedAddress}, {@link parseLeafAddress}) and no host address at all. The
 * refusal lives here rather than at each caller, so a reader that asks "is this a host
 * address?" never has to re-spell the separator to get the answer right.
 *
 * A name carrying a colon is not hypothetical: a hook's name is its lifecycle event, then
 * `:` and its matcher's authored bytes (`hook:PostToolUse:Edit|Write`), because the
 * matcher group — not the event — is the member. The grammar is unchanged by it: the
 * split is at the first colon, so the kind comes off and the rest stays one name, and the
 * qualifier joined onto it is judged by {@link isNameQualifier} rather than given a third
 * segment.
 */
export function parseHostAddress(address: string): HostAddress | undefined {
  if (address.includes("/")) return undefined;
  const colon = address.indexOf(":");
  if (colon <= 0 || colon === address.length - 1) return undefined;
  return { kind: address.slice(0, colon), name: address.slice(colon + 1) };
}

/** The three identity segments beneath a host, plus the `/<leaf>` tail that may follow them. */
interface Segments {
  readonly host: string;
  readonly kind: string;
  readonly key: string;
  /** The `/<leaf>` tail, or `undefined` when the address stops at member grain. */
  readonly tail: string | undefined;
}

/**
 * Cut an address into its segments, or `undefined` when it carries fewer than three or a
 * segment-shaped hole. The tail keeps its own dots and slashes, so it is the whole
 * remainder after the third slash rather than a fourth segment among more.
 */
function segment(address: string): Segments | undefined {
  const parts = address.split("/");
  if (parts.length < 3) return undefined;
  const [host, kind, key] = parts as [string, string, string];
  const tail = parts.length > 3 ? parts.slice(3).join("/") : undefined;
  if (host === "" || kind === "" || key === "" || tail === "") return undefined;
  return { host, kind, key, tail };
}

/**
 * Parse a nested-member address, or `undefined` when `address` is not one: exactly three
 * non-empty segments, the first of them a host address.
 *
 * A `/<leaf>` tail is ruled out here explicitly — a leaf is its own grain, and truncating
 * one to the member that happens to contain it would answer a leaf reference with a member
 * the author never named.
 */
export function parseNestedAddress(address: string): NestedAddress | undefined {
  const segments = segment(address);
  if (segments === undefined || segments.tail !== undefined) return undefined;
  if (parseHostAddress(segments.host) === undefined) return undefined;
  return { host: segments.host, kind: segments.kind, key: segments.key };
}

/**
 * Parse a leaf address, or `undefined` when `target` carries no tail or a segment-shaped
 * hole. The head is carried on verbatim rather than split, because the bare member id is
 * a live short form ({@link LeafAddress.member}).
 */
export function parseLeafAddress(target: string): LeafAddress | undefined {
  const segments = segment(target);
  if (segments?.tail === undefined) return undefined;
  return { member: segments.host, kind: segments.kind, key: segments.key, childPath: segments.tail };
}
