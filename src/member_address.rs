//! The **member-address grammar** — one home for every spelling
//! `specs/model/representation.md` ("member") gives a member's identity, and for every
//! reader and writer of one.
//!
//! Three spellings, each built on the one before it:
//!
//! - `<kind>:<name>` — a **host address**, a top-level member's own identity.
//! - `<host-address>/<kind>/<key>` — a **nested-member address**, the identity a member
//!   beneath a host carries. Any member's address may host, so the host segment is itself
//!   a member address and the spelling nests as deep as the model does.
//! - `<member-address>/<leaf>` — a **leaf address**, one authored string beneath a nested
//!   member. A grain of its own, and no member address at all.
//!
//! Every name, key and leaf is **one segment** — a leaf's child path joins its own layers
//! with `.`, never `/` ([`crate::extract::EmbeddedMember::addressed_leaves`]) — so the
//! segment **count** decides the grain: a member address has an odd count, a leaf address
//! an even one (`specs/model/representation.md`, "member"). That parity is the whole
//! discrimination, which is why both grains are read off **one** segmentation
//! ([`segment`]) rather than two independently-shaped parses that could come to disagree
//! about where the member ends. Every writer and every reader in the tree goes through
//! this module: a `format!` or a `split_once(':')` spelling this grammar anywhere else is
//! a second implementation of one job
//! (`specs/process/engineering.md`, "One job, one home").
//!
//! Distinct from [`crate::address`], which is **field** addressing — the dotted path a
//! clause's `field` names inside one member's values. This module addresses members.

/// The two halves an address names something by, borrowed out of the address they were cut
/// from: a host address's `(kind, name)`, or a nested member's own `(kind, key)`. The owned
/// twin is [`crate::graph::Node`]; this module is the bottom of the tree and owes `graph`
/// nothing, so the callers that want a node own the halves themselves.
pub type AddressPair<'a> = (&'a str, &'a str);

/// A **top-level** member's own `kind:name` address — the one-segment floor the nesting
/// [`nested_address`] spells is built on, and the writer half of [`parse_host_address`].
///
/// It is what [`crate::drift::NestedMemberRow::host`] carries for a host that is itself
/// top-level. A host may be nested, and then that column carries the host's own whole
/// address instead — `sdk/src/declarations.ts`'s `nestedMemberRows` spells it through
/// `memberAddress`, which host-qualifies a nested member — so the column's deeper
/// spelling is [`nested_address`]'s, never this form with a `/` smuggled into `id`.
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

/// The member address an identity spells — the **one** home for the discrimination
/// between the two forms a member's id may already hold.
///
/// A top-level member's identity is its bare name, so its address is its kind joined onto
/// it ([`host_address`]). A **nested** member's identity *is* its whole
/// `<host-address>/<kind>/<key>` address — the host segment is the whole of what tells two
/// same-keyed children under different hosts apart — so joining a kind onto it a second
/// time spells an address no member wears and no reader resolves.
///
/// Every site that must name "the address of the member this id belongs to" reads the rule
/// here rather than deciding the split again: an embedded member, a nested **file** child,
/// and a top-level member all reach it through one call.
#[must_use]
pub fn address_of(kind: &str, id: &str) -> String {
    if parse_nested_address(id).is_some() {
        id.to_string()
    } else {
        host_address(kind, id)
    }
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
/// the very address the member keyed `authority` spells for its `rejected` leaf — and
/// since the count decides the grain, that address is **even** and so a leaf's by
/// construction, no member address at all. The member's own identity answers its sibling's
/// leaf and nothing answers the member. An empty key spells an address [`segment`] admits
/// at no grain at all.
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
    /// The host member's own address — every segment but the final two, verbatim, and
    /// itself a member address (`area:a`, or `area:a/page/b` one layer down).
    pub host: &'a str,
    /// The host member's kind, read at the host's own grain ([`host_identity`]).
    host_kind: &'a str,
    /// The host member's identity: its name when the host is top-level, its whole address
    /// when the host is itself nested — the form [`address_of`] hands back.
    host_name: &'a str,
    /// The nested member's kind.
    pub kind: &'a str,
    /// The nested member's key among its host's members of that kind.
    pub key: &'a str,
}

/// A parsed **leaf address** — the `<member-address>/<leaf>` spelling `explain` accepts to
/// name a single nested member's leaf. The segments ahead of the leaf are exactly a
/// member address, which is why both grains are read off [`segment`]: the leaf address is
/// that address plus a tail, never a second grammar.
pub struct ParsedLeaf<'a> {
    /// The member the nested member carrying this leaf lives under, verbatim as its
    /// author spelled it: the canonical `<kind>:<name>` host address, that host's own
    /// whole address one layer down, or the **bare** member id, the short form the SDK's
    /// own leaf writer and the committed lock mention targets spell (0024 — a spelling the
    /// corpus commits is never retired under a reader's feet). Resolution accepts all of
    /// them (`crate::read`'s `resolve_leaf`).
    pub member: &'a str,
    /// The nested member's kind.
    pub kind: &'a str,
    /// The nested member's key among its host's members of that kind.
    pub key: &'a str,
    /// The leaf's path within the nested member — the address's **final segment**, dots
    /// intact and carrying no `/`: a child path joins its own layers with `.`
    /// (`crate::extract`'s `addressed_leaves`), so a further `/` is a further segment and
    /// the count reads the whole as a member address instead.
    pub child_path: &'a str,
}

/// The identity segments a member address beneath a host carries — its host address, and
/// the nested kind and key the final two segments spell — plus the `/<leaf>` tail an
/// even-count address ends with. The **one** segmentation both grains parse off, so a
/// member address and the leaf address beneath it can never disagree about where the
/// member ends.
struct Segments<'a> {
    /// Every segment but the final two of the member address: the host, verbatim.
    host: &'a str,
    kind: &'a str,
    key: &'a str,
    /// The `/<leaf>` tail, or `None` when the address stops at member grain.
    tail: Option<&'a str>,
}

/// Cut an address into its segments, or `None` when it carries fewer than three or a
/// segment-shaped hole — an address names exactly one thing or the verb refuses, so an
/// empty segment names nothing.
///
/// The cut is at **every** `/`, and the count decides the grain: every name, key and leaf
/// is one segment, so an even count ends in a `/<leaf>` tail and an odd one stops at
/// member grain (`specs/model/representation.md`, "member"). The tail is therefore the
/// final segment alone, not the remainder past a fixed count.
///
/// Three is the cut's floor: the shallowest member address beneath a host is
/// `<kind>:<name>/<kind>/<key>`, and the shallowest leaf address names a nested member's
/// kind and key ahead of its leaf ([`ParsedLeaf`]), so it is four.
fn segment(address: &str) -> Option<Segments<'_>> {
    let count = address.split('/').count();
    if count < 3 || address.split('/').any(str::is_empty) {
        return None;
    }
    // Even is leaf grain: the final segment is the whole leaf, and everything ahead of it
    // is a member address of odd count — three segments at least, which is what the cut
    // below takes apart.
    let (member, tail) = if count.is_multiple_of(2) {
        let (member, leaf) = address.rsplit_once('/')?;
        (member, Some(leaf))
    } else {
        (address, None)
    };
    // A member address ends in its own kind and key; everything ahead of those two is the
    // host address, carried whole however deep it runs.
    let (rest, key) = member.rsplit_once('/')?;
    let (host, kind) = rest.rsplit_once('/')?;
    Some(Segments {
        host,
        kind,
        key,
        tail,
    })
}

/// The `(kind, identity)` a **host** segment names, read at the host's own grain: a
/// top-level host spells its kind and its name ([`parse_host_address`]), while a nested
/// host's kind is its own and its identity *is* its whole address — the same
/// discrimination [`address_of`] makes from the writer's side, so the pair round-trips
/// back to the address it was read from.
///
/// Read through the one parser rather than off a first-colon split, which would take
/// `area` / `a/page/b` out of `area:a/page/b` — a kind that host does not instantiate and
/// a name no member bears. A host that is neither grain names nothing: the top-level
/// split is refused unless the host is exactly one segment, since [`parse_host_address`]
/// cuts at the first colon and would otherwise read `a/b:c/d` as one.
fn host_identity(host: &str) -> Option<AddressPair<'_>> {
    if let Some(nested) = parse_nested_address(host) {
        return Some((nested.kind, host));
    }
    if !is_one_segment(host) {
        return None;
    }
    parse_host_address(host)
}

/// Parse a nested-member address, or `None` when `address` is not one.
///
/// The grammar is an **odd** count of non-empty segments, three or more: a host address —
/// every segment but the final two, itself a member address at any depth — then the nested
/// kind and the key. Depth is the model's to choose, not the grammar's: any member's
/// address may host (`representation.md`, "member"), so the reader reads what the model
/// nests rather than capping at one layer.
///
/// The **leaf tail is ruled out by the count**: `representation.md` spells `/<leaf>`
/// beneath a member address, and a leaf is its own grain — one addressable authored
/// string, parsed by [`parse_leaf_address`] off this very segmentation and resolved
/// against the serialized leaves. So an even count is no member address: it resolves to no
/// member and dangles under the name its author wrote, rather than truncating to the
/// member that happens to contain the leaf — which would answer a leaf reference with a
/// member and put an arc the author never wrote into the graph.
#[must_use]
pub fn parse_nested_address(address: &str) -> Option<NestedAddress<'_>> {
    let segments = segment(address)?;
    // An odd count and no tail: an even count ends in a `/<leaf>`, a different grain.
    if segments.tail.is_some() {
        return None;
    }
    // The host is itself a member address, so its identity is read at its own grain.
    let (host_kind, host_name) = host_identity(segments.host)?;
    Some(NestedAddress {
        host: segments.host,
        host_kind,
        host_name,
        kind: segments.kind,
        key: segments.key,
    })
}

/// Parse a leaf address — a member address plus its `/<leaf>` tail, so an **even** count
/// of segments — or `None` when `target` carries an odd count or a segment-shaped hole (a
/// malformed address the caller reports as such). Keyed and structural: the address rides
/// the shape the author already wrote, stable under content edits.
///
/// The head is the host address the member grain spells, and it is carried on **verbatim**
/// rather than split here, because the bare member id is a live short form the lock
/// already commits ([`ParsedLeaf::member`]). Which spelling a head is, is resolution's
/// question, not the grammar's.
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
/// member by key alone — and its **host**'s, read at the host's own grain
/// ([`host_identity`]): a top-level host's `(kind, name)`, or a nested host's own kind and
/// its whole address. `None` when `address` is no nested-member address.
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
    fn an_identity_already_holding_a_whole_address_is_never_joined_onto_twice() {
        // A top-level member's id is a bare name: the address is its kind joined onto it.
        assert_eq!(
            address_of("rule", "collaboration"),
            "rule:collaboration",
            "a bare identity takes its kind"
        );

        // A nested member's id — an embedded value's or a nested **file** child's alike —
        // is already the whole address, so it comes back verbatim. Joining the kind on
        // again would spell `supporting-doc:skill:coordinate/supporting-doc/checklist`,
        // an address no member wears and neither membership lookup resolves.
        let nested = nested_address(
            &host_address("skill", "coordinate"),
            "supporting-doc",
            "checklist",
        );
        assert_eq!(address_of("supporting-doc", &nested), nested);
        assert_eq!(
            parse_nested_address(&address_of("supporting-doc", &nested)).map(|parsed| (
                parsed.host,
                parsed.kind,
                parsed.key
            )),
            Some(("skill:coordinate", "supporting-doc", "checklist"))
        );

        // A leaf address is no member address, so it takes the bare-identity branch: the
        // discrimination is the member-grain parser's verdict, never a `contains('/')`.
        let leaf = format!("{nested}/body");
        assert_eq!(
            address_of("supporting-doc", &leaf),
            host_address("supporting-doc", &leaf)
        );
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
    fn a_leaf_path_keeps_its_dots_and_is_one_segment() {
        let parsed =
            parse_leaf_address("spec:20-surface/decision/authority/rejected.baked.because")
                .expect("a dotted collection path is one leaf path");
        assert_eq!(parsed.child_path, "rejected.baked.because");

        // A child path joins its own layers with `.`, never `/` (`crate::extract`'s
        // `addressed_leaves`), so a further `/` is a further *segment* and the count reads
        // the whole: five segments is a member address, not a leaf path carrying a slash.
        let five = "spec:20-surface/decision/authority/a/b";
        assert!(
            parse_leaf_address(five).is_none(),
            "an odd count is no leaf"
        );
        assert_eq!(
            parse_nested_address(five).map(|member| (member.host, member.kind, member.key)),
            Some(("spec:20-surface/decision/authority", "a", "b"))
        );
    }

    #[test]
    fn the_segment_count_decides_the_grain_at_any_depth() {
        // Every name, key and leaf is one segment, so the count is the whole
        // discrimination: odd is a member address, even a leaf's
        // (`specs/model/representation.md`, "member"). Any member's address may host, so
        // the reader reads what the model nests instead of capping at one layer.
        let one = host_address("area", "a");
        let three = nested_address(&one, "page", "b");
        let five = nested_address(&three, "section", "c");
        let seven = nested_address(&five, "note", "d");

        for (address, host, kind, key) in [
            (&three, one.as_str(), "page", "b"),
            (&five, three.as_str(), "section", "c"),
            (&seven, five.as_str(), "note", "d"),
        ] {
            let member = parse_nested_address(address).expect("an odd count is member grain");
            assert_eq!((member.host, member.kind, member.key), (host, kind, key));
            assert!(
                parse_leaf_address(address).is_none(),
                "`{address}` is member grain, never a leaf's"
            );
        }

        // The leaf grain at those same depths, one segment further down each time: the
        // leaf is the final segment, and the member it hangs under is read at its own
        // grain ahead of it.
        for (address, member, kind, key) in [
            (format!("{three}/purpose"), one.as_str(), "page", "b"),
            (format!("{five}/purpose"), three.as_str(), "section", "c"),
        ] {
            let leaf = parse_leaf_address(&address).expect("an even count is leaf grain");
            assert_eq!(
                (leaf.member, leaf.kind, leaf.key, leaf.child_path),
                (member, kind, key, "purpose")
            );
            assert!(
                parse_nested_address(&address).is_none(),
                "`{address}` is leaf grain, never a member's"
            );
        }
    }

    #[test]
    fn a_nested_hosts_identity_is_its_whole_address_and_never_a_first_colon_split() {
        // The host of a deep member address is read at the *host's* own grain: its kind is
        // its own (`page`, not `area`) and its identity is the whole address, which is
        // exactly what `address_of` round-trips. A first-colon split would name the kind
        // `area` and a member `a/page/b` that nothing bears.
        let three = "area:a/page/b";
        let five = "area:a/page/b/section/c";

        assert_eq!(
            embedded_source_host(three),
            Some((("page", "b"), ("area", "a"))),
            "a top-level host spells its kind and its name"
        );
        assert_eq!(
            embedded_source_host(five),
            Some((("section", "c"), ("page", three))),
            "a nested host spells its own kind and its whole address"
        );
        assert_eq!(
            address_of("page", three),
            three,
            "the host identity the pair carries is the address it came from"
        );

        // A host that is no member address at either depth names nothing: the bare short
        // form is a leaf-grain spelling, and it never climbs to member grain.
        assert!(parse_nested_address("note/requirement/my-req/hook/0").is_none());
        // Nor does a colon smuggled past the segmentation make a host of a `/`-bearing
        // head: a top-level host is exactly one segment.
        assert!(parse_nested_address("a/b:c/d/e").is_none());
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

        // Depth changes nothing: the hole is refused wherever the count puts it, so an
        // empty segment names nothing at either grain however deep the address runs.
        for address in [
            "area:a/page//section/c",
            "area:a//b/section/c",
            "area:a/page/b/section/",
            "area:a/page/b//c",
        ] {
            assert!(
                parse_nested_address(address).is_none(),
                "`{address}` is no member address"
            );
        }
        for address in [
            "area:a/page/b//c/purpose",
            "area:a/page/b/section/c/",
            "area:a//b/section/c/purpose",
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
