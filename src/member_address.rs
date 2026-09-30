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
///
/// A name carrying a colon is not hypothetical: a hook's name is its lifecycle event,
/// then `:` and its matcher's authored bytes (`hook:PostToolUse:Edit|Write`, decision
/// 0074), because the matcher group — not the event — is the member
/// (`specs/model/representation.md`, "member"). That name is still **one** name: the
/// split here is at the first colon, so the kind comes off and the rest stays whole,
/// and no documented lifecycle event carries a colon of its own for the two to be
/// confused over. The grammar is unchanged by it — which is why the qualifier the name
/// joins is judged by [`is_name_qualifier`] rather than given a third segment.
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

/// Whether `spelling` is exactly **one segment** of this grammar — non-empty, and carrying
/// none of the `/` [`segment`] cuts an address at.
///
/// The predicate every writer's caller judges a key by, because [`nested_address`] is
/// infallible and the reader beneath it is not. A key carrying a `/` shifts every segment
/// below it by one: `nested_address("spec:alpha", "decision", "authority/rejected")` spells
/// the very address the member keyed `authority` spells for its `rejected` leaf, and the
/// leaf grain is what every reader tries first ([`crate::graph`]'s `node_from_address`,
/// [`crate::read`]'s species split), so the member's own identity answers its sibling's
/// leaf. An empty key spells an address [`segment`] admits at no grain at all.
/// `specs/model/representation.md` ("member") makes both a malformed lock rather than a
/// precedence rule: resolution is total, and coincident addresses are refused.
///
/// It lives here, beside the writer and the reader that must agree with it, rather than as
/// a `contains('/')` per refusing caller — a hand-rolled spelling of this grammar anywhere
/// else is a second implementation of one job.
#[must_use]
pub fn is_one_segment(spelling: &str) -> bool {
    !spelling.is_empty() && !spelling.contains('/')
}

/// Whether `spelling` may be joined onto a member's name as a **qualifier** — the text a
/// name carries after its own `:` to tell two members apart that share a collection key
/// (a hook's matcher: `PostToolUse:Edit|Write`, decision 0074).
///
/// The bar is the one [`is_one_segment`] enforces for a key, less its non-emptiness: a
/// qualifier joins a non-empty key, so the joined name is non-empty whatever the
/// qualifier is, and an **empty** qualifier is a real authored spelling — a group whose
/// `matcher` is `""` is its own group on the wire, and a matcher is carried verbatim,
/// never normalized. What it may not carry is the `/` [`segment`] cuts an address at:
/// the joined name is the first segment of every address beneath the member, so a `/`
/// in it shifts every segment below by one and the member's own handler answers to an
/// address naming no member at all.
///
/// It lives here, beside the writer and the reader that must agree with it, rather than
/// as a `contains('/')` at each refusing caller — the reason [`is_one_segment`] does.
#[must_use]
pub fn is_name_qualifier(spelling: &str) -> bool {
    !spelling.contains('/')
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
    fn one_segment_is_non_empty_and_carries_no_separator() {
        for spelling in ["on-enter", "rejected.baked.because", "a:b", "x"] {
            assert!(is_one_segment(spelling), "`{spelling}` is one segment");
        }
        for spelling in ["", "authority/rejected", "/", "a/"] {
            assert!(!is_one_segment(spelling), "`{spelling}` is not one segment");
        }

        // Why the predicate exists, stated against the writer and the reader it sits
        // between: a key that is not one segment does not come back out of the reader as
        // the member it was written for. It reads at *leaf* grain instead, as the
        // `rejected` leaf of the sibling keyed `authority` — the same address, one grain
        // down, which the reader half already pins from the other side
        // (`a_leaf_path_keeps_its_dots_and_its_deeper_slashes`).
        let shifted = nested_address("spec:alpha", "decision", "authority/rejected");
        assert!(parse_nested_address(&shifted).is_none());
        assert_eq!(
            parse_leaf_address(&shifted).map(|leaf| (leaf.key, leaf.child_path)),
            Some(("authority", "rejected"))
        );
    }

    #[test]
    fn a_name_qualifier_admits_the_empty_spelling_and_refuses_the_separator() {
        // The qualifier bar is `is_one_segment`'s, less its non-emptiness: a hook whose
        // `matcher` is `""` is its own group on the wire, and the name it joins onto is
        // non-empty whatever the qualifier holds.
        for spelling in ["", "Edit|Write", "Edit, Write", "*", ".*", "a:b"] {
            assert!(is_name_qualifier(spelling), "`{spelling}` qualifies a name");
        }
        assert!(!is_one_segment(""), "the empty spelling is no key");

        // The one refusal: a `/` in the qualifier is a `/` in the name, and the name is
        // the first segment of every address beneath the member.
        for spelling in ["a/b", "/", "Edit|Write/"] {
            assert!(
                !is_name_qualifier(spelling),
                "`{spelling}` carries the separator"
            );
        }

        // Why: the joined name round-trips as one name, and its handler addresses beneath
        // it — but only while the qualifier carries no separator.
        let joined = host_address("hook", "PostToolUse:Edit|Write");
        assert_eq!(
            parse_host_address(&joined),
            Some(("hook", "PostToolUse:Edit|Write"))
        );
        let handler = nested_address(&joined, "handler", "0");
        assert_eq!(
            parse_nested_address(&handler).map(|nested| (nested.host, nested.kind, nested.key)),
            Some((joined.as_str(), "handler", "0"))
        );

        // And with one, every segment below shifts: the address the handler was written
        // for reads at *leaf* grain instead, naming no member at all.
        let shifted = nested_address(&host_address("hook", "PostToolUse:a/b"), "handler", "0");
        assert!(parse_nested_address(&shifted).is_none());
        assert_eq!(
            parse_leaf_address(&shifted).map(|leaf| (
                leaf.member,
                leaf.kind,
                leaf.key,
                leaf.child_path
            )),
            Some(("hook:PostToolUse:a", "b", "handler", "0"))
        );
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
