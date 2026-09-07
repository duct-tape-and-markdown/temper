//! The **member-address grammar** — one home for every spelling
//! `specs/model/representation.md` ("member") gives a member's identity, and for every
//! reader and writer of one.
//!
//! Three spellings, each a suffix of the one before it:
//!
//! - `<kind>:<name>` — a **host address**, a top-level member's own identity.
//! - `<host-address>/<kind>/<key>` — a **nested-member address**, the identity a member
//!   embedded in a host carries.
//! - `<host-address>/<kind>/<key>/<leaf>` — a **leaf address**, one authored string
//!   beneath a nested member. A grain of its own, and no member address at all.
//!
//! Because the third is the second plus a `/<leaf>` tail, the two are read by **one**
//! segmentation ([`segment`]) rather than two independently-shaped parses that could come
//! to disagree about what the first segment is. Every writer and every reader in the tree
//! goes through this module: a `format!` or a `split_once(':')` spelling this grammar
//! anywhere else is a second implementation of one job
//! (`specs/process/engineering.md`, "One job, one home").
//!
//! Distinct from [`crate::address`], which is **field** addressing — the dotted path a
//! clause's `field` names inside one member's values. This module addresses members.

/// The two halves an address names something by, borrowed out of the address they were cut
/// from: a host address's `(kind, name)`, or a nested member's own `(kind, key)`. The owned
/// twin is [`crate::graph::Node`]; this module is the bottom of the tree and owes `graph`
/// nothing, so the callers that want a node own the halves themselves.
pub type AddressPair<'a> = (&'a str, &'a str);

/// This host member's own `kind:name` address — the key
/// [`crate::drift::NestedMemberRow::host`] carries, the identical `${kind}:${name}` form
/// `sdk/src/declarations.ts`'s `nestedMemberRows` writes it in.
#[must_use]
pub fn host_address(kind: &str, id: &str) -> String {
    format!("{kind}:{id}")
}

/// The `(kind, name)` a host address spells, or `None` when `address` is no host address
/// — the reader half of [`host_address`], and the one home for the split every consumer
/// of the `<kind>:<name>` form used to hand-roll.
///
/// Both halves are non-empty: an address names exactly one thing or the verb refuses, so
/// `:name` and `kind:` name nothing rather than a member under an anonymous kind. Splits
/// at the **first** colon, so a name carrying one stays whole.
#[must_use]
pub fn parse_host_address(address: &str) -> Option<AddressPair<'_>> {
    let (kind, name) = address.split_once(':')?;
    (!kind.is_empty() && !name.is_empty()).then_some((kind, name))
}

/// Spell a nested member's address from its host address, kind and key — the writer beside
/// [`parse_nested_address`], so the grammar has one home rather than a `format!` per
/// producer.
#[must_use]
pub fn nested_address(host: &str, kind: &str, key: &str) -> String {
    format!("{host}/{kind}/{key}")
}

/// One parsed **nested-member address** — `<host-address>/<kind>/<key>`
/// (`skill:use-when-x/hook/on-enter`), the identity `specs/model/representation.md`
/// ("member") spells for a nested member and the one
/// [`crate::compose::embedded_features_by_kind`] writes onto every embedded member it
/// lifts. One parser reads the grammar for every site that reads it — which declared kind
/// an address names, which member it is, and which host an embedded member's edge belongs
/// to (`crate::graph`'s `target_identity`, `member_named` and `edge_host`) — so the
/// readers cannot come to disagree about what an address is.
pub struct NestedAddress<'a> {
    /// The host member's own `<kind>:<name>` address — the segment before the first `/`.
    pub host: &'a str,
    /// The host member's kind — the half of `host` before its `:`.
    host_kind: &'a str,
    /// The host member's name — the half of `host` after its `:`.
    host_name: &'a str,
    /// The nested member's kind.
    pub kind: &'a str,
    /// The nested member's key among its host's members of that kind.
    pub key: &'a str,
}

/// A parsed **leaf address** — the `<host-address>/<kind>/<key>/<leaf>` spelling `explain`
/// accepts to name a single nested member's leaf. The three identity segments ahead of it
/// are exactly a nested-member address, which is why both are read off [`segment`]: the
/// leaf address is that address plus a tail, never a second grammar.
///
/// The leaf path keeps its own dots and slashes (`rejected.baked-projection.because`), so
/// it is the whole remainder after the third slash — `splitn(4, '/')`, never a plain split
/// that would mangle a dotted collection path.
pub struct ParsedLeaf<'a> {
    /// The member the leaf lives under, verbatim as its author spelled it: the canonical
    /// `<kind>:<name>` host address, or the **bare** member id, the short form the SDK's
    /// own leaf writer and the committed lock mention targets spell (0024 — a spelling the
    /// corpus commits is never retired under a reader's feet). Resolution accepts both
    /// (`crate::read`'s `resolve_leaf`).
    pub member: &'a str,
    /// The nested member's kind.
    pub kind: &'a str,
    /// The nested member's key among its host's members of that kind.
    pub key: &'a str,
    /// The leaf's path within the nested member — the whole remainder after the third
    /// slash, dots intact.
    pub child_path: &'a str,
}

/// The three identity segments a member address beneath a host carries, and the `/<leaf>`
/// tail that may follow them — the **one** segmentation both grains parse off, so a
/// nested-member address and the leaf address beneath it can never disagree about where
/// the member ends.
struct Segments<'a> {
    host: &'a str,
    kind: &'a str,
    key: &'a str,
    /// The `/<leaf>` tail, or `None` when the address stops at member grain.
    tail: Option<&'a str>,
}

/// Cut an address into its segments, or `None` when it carries fewer than three or a
/// segment-shaped hole — an address names exactly one thing or the verb refuses, so an
/// empty segment names nothing.
fn segment(address: &str) -> Option<Segments<'_>> {
    let mut parts = address.splitn(4, '/');
    let host = parts.next()?;
    let kind = parts.next()?;
    let key = parts.next()?;
    let tail = parts.next();
    if host.is_empty() || kind.is_empty() || key.is_empty() || tail == Some("") {
        return None;
    }
    Some(Segments {
        host,
        kind,
        key,
        tail,
    })
}

/// Parse a nested-member address, or `None` when `address` is not one.
///
/// The grammar is **exactly three** segments — a `<kind>:<name>` host address, the nested
/// kind, the key — each of them non-empty.
///
/// The **leaf tail is ruled out here, explicitly**: `representation.md` spells `/<leaf>`
/// *beneath* a nested address, and a leaf is its own grain — one addressable authored
/// string, parsed by [`parse_leaf_address`] off this very segmentation and resolved
/// against the serialized leaves. So a fourth segment is no member address: it resolves to
/// no member and dangles under the name its author wrote, rather than truncating to the
/// member that happens to contain the leaf — which would answer a leaf reference with a
/// member and put an arc the author never wrote into the graph.
#[must_use]
pub fn parse_nested_address(address: &str) -> Option<NestedAddress<'_>> {
    let segments = segment(address)?;
    // Three segments and no more: a fourth is the `/<leaf>` tail, a different grain.
    if segments.tail.is_some() {
        return None;
    }
    // The host segment is itself a member address, so it carries a kind and a name.
    let (host_kind, host_name) = parse_host_address(segments.host)?;
    Some(NestedAddress {
        host: segments.host,
        host_kind,
        host_name,
        kind: segments.kind,
        key: segments.key,
    })
}

/// Parse a leaf address — a nested-member address plus its `/<leaf>` tail — or `None` when
/// `target` carries no tail or a segment-shaped hole (a malformed address the caller
/// reports as such). Keyed and structural: the address rides the shape the author already
/// wrote, stable under content edits.
///
/// The head segment is the canonical `<kind>:<name>` host address the nested grain spells,
/// and it is carried on **verbatim** rather than split here, because the bare member id is
/// a live short form the lock already commits ([`ParsedLeaf::member`]). Which of the two a
/// head is, is resolution's question, not the grammar's.
#[must_use]
pub fn parse_leaf_address(target: &str) -> Option<ParsedLeaf<'_>> {
    let segments = segment(target)?;
    let child_path = segments.tail?;
    Some(ParsedLeaf {
        member: segments.host,
        kind: segments.kind,
        key: segments.key,
        child_path,
    })
}

/// The two `(kind, name)` pairs an embedded member's own address names: its own
/// `(kind, key)` — the short spelling a declaration row uses when it names an embedded
/// member by key alone — and its **host**'s `(kind, name)`. `None` when `address` is no
/// nested-member address.
///
/// The reader half of the grammar [`nested_address`] writes: every consumer that needs a
/// nested member's host reads it here, off the member's own identity, rather than
/// re-splitting the address on its own (`crate::graph`'s `edge_host` and its citation-
/// scoping index `embedded_hosts_by_key`).
#[must_use]
pub fn embedded_source_host(address: &str) -> Option<(AddressPair<'_>, AddressPair<'_>)> {
    let nested = parse_nested_address(address)?;
    Some((
        (nested.kind, nested.key),
        (nested.host_kind, nested.host_name),
    ))
}

/// The **key** segment of a nested member's own address, or `None` when `address` is no
/// nested-member address — the bare short form a reference may spell, read through the
/// one parser rather than a fresh split at each reader (`crate::read`'s `explain`
/// resolution is the other one).
#[must_use]
pub fn nested_key(address: &str) -> Option<&str> {
    parse_nested_address(address).map(|nested| nested.key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_address_is_kind_colon_id() {
        assert_eq!(host_address("rule", "collaboration"), "rule:collaboration");
    }

    #[test]
    fn a_host_address_round_trips_through_its_own_reader() {
        let written = host_address("rule", "collaboration");
        assert_eq!(
            parse_host_address(&written),
            Some(("rule", "collaboration"))
        );

        // A name carrying its own colon stays whole: the split is at the first one.
        assert_eq!(parse_host_address("rule:a:b"), Some(("rule", "a:b")));

        // A segment-shaped hole names nothing.
        assert_eq!(parse_host_address("collaboration"), None);
        assert_eq!(parse_host_address(":collaboration"), None);
        assert_eq!(parse_host_address("rule:"), None);
    }

    #[test]
    fn a_leaf_address_is_the_nested_address_it_is_a_tail_of_plus_its_leaf() {
        // The point of the shared segmentation: one address, cut once, read at two grains.
        let nested = "skill:use-when-x/hook/on-enter";
        let leaf = "skill:use-when-x/hook/on-enter/command";

        let member = parse_nested_address(nested).expect("three segments is member grain");
        assert_eq!(
            (member.host, member.kind, member.key),
            ("skill:use-when-x", "hook", "on-enter")
        );
        assert!(
            parse_leaf_address(nested).is_none(),
            "member grain carries no leaf"
        );

        let tail = parse_leaf_address(leaf).expect("four segments is leaf grain");
        assert_eq!(
            (tail.member, tail.kind, tail.key, tail.child_path),
            ("skill:use-when-x", "hook", "on-enter", "command")
        );
        assert!(
            parse_nested_address(leaf).is_none(),
            "a leaf tail is no member address"
        );
    }

    #[test]
    fn a_leaf_path_keeps_its_dots_and_its_deeper_slashes() {
        let parsed =
            parse_leaf_address("spec:20-surface/decision/authority/rejected.baked.because")
                .expect("a dotted collection path is one leaf path");
        assert_eq!(parsed.child_path, "rejected.baked.because");

        // Whatever follows the third slash is the leaf path, slashes included — the fourth
        // segment is never re-cut into a fifth.
        let deeper =
            parse_leaf_address("spec:20-surface/decision/authority/a/b").expect("still a leaf");
        assert_eq!(deeper.child_path, "a/b");
    }

    #[test]
    fn a_bare_member_head_parses_at_leaf_grain_and_never_at_member_grain() {
        // The committed short form: a leaf address whose head is a bare member id parses,
        // and resolution accepts it. The same head at member grain does not — a nested
        // member's host segment is a host address, always.
        let parsed = parse_leaf_address("note/requirement/my-req/chosen")
            .expect("the bare short form is a leaf address");
        assert_eq!(parsed.member, "note");
        assert!(parse_nested_address("note/requirement/my-req").is_none());
    }

    #[test]
    fn a_segment_shaped_hole_names_nothing_at_either_grain() {
        for address in [
            "/hook/on-enter",
            "skill:x//on-enter",
            "skill:x/hook/",
            "skill:x/hook",
        ] {
            assert!(
                parse_nested_address(address).is_none(),
                "`{address}` is no nested-member address"
            );
        }
        for address in [
            "/hook/on-enter/command",
            "skill:x//on-enter/command",
            "skill:x/hook//command",
            "skill:x/hook/on-enter/",
        ] {
            assert!(
                parse_leaf_address(address).is_none(),
                "`{address}` is no leaf address"
            );
        }
    }

    #[test]
    fn the_two_readers_built_on_the_parser_read_the_same_grammar() {
        let address = "skill:use-when-x/hook/on-enter";
        assert_eq!(nested_key(address), Some("on-enter"));
        assert_eq!(
            embedded_source_host(address),
            Some((("hook", "on-enter"), ("skill", "use-when-x")))
        );

        // Both are the parser's own verdict, leaf tail included.
        let leaf = "skill:use-when-x/hook/on-enter/command";
        assert_eq!(nested_key(leaf), None);
        assert_eq!(embedded_source_host(leaf), None);
    }
}
