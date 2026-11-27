//! Pins the root's listing budget advisory: every model-invocable skill's and command's
//! `description` + `when_to_use`, each entry cut at 1,536 characters, judged as one sum
//! against the runtime's 8,000-character listing budget.

use temper::drift::Declarations;

use crate::common;

/// The label the listing budget advisory reports under.
const LISTING: &str = "root.listing.description+when_to_use";

/// One skill or command to list: its `description` and `when_to_use` lengths, and
/// whether it disables model invocation.
struct Listed {
    description: usize,
    when_to_use: usize,
    withheld: bool,
}

/// The frontmatter-led markdown one listed member carries, skill or command alike.
fn listed_md(name: &str, listed: &Listed) -> String {
    let withheld = if listed.withheld {
        "disable-model-invocation: true\n"
    } else {
        ""
    };
    format!(
        "---\nname: {name}\ndescription: {}\nwhen_to_use: {}\n{withheld}---\n# {name}\n",
        "d".repeat(listed.description),
        "w".repeat(listed.when_to_use),
    )
}

/// Check a harness carrying one `.claude/skills/<name>/SKILL.md` per listed skill.
fn listing_run(slug: &str, skills: &[Listed]) -> common::CheckRun {
    joint_listing_run(slug, skills, &[])
}

/// Check a harness carrying one `.claude/skills/<name>/SKILL.md` per listed skill and
/// one `.claude/commands/<name>.md` per listed command.
fn joint_listing_run(slug: &str, skills: &[Listed], commands: &[Listed]) -> common::CheckRun {
    let root = common::tmpdir(slug);
    for (index, skill) in skills.iter().enumerate() {
        let name = format!("listed-{index}");
        common::write_skill(&root, &name, &listed_md(&name, skill));
    }
    for (index, command) in commands.iter().enumerate() {
        let name = format!("command-{index}");
        common::write_sibling(
            &root,
            &format!(".claude/commands/{name}.md"),
            &listed_md(&name, command),
        );
    }
    common::write_lock(&root, Declarations::default());
    common::check_in(&root, &[], None)
}

/// `count` skills, each listing `description` + `when_to_use` characters.
fn listed(count: usize, description: usize, when_to_use: usize) -> Vec<Listed> {
    (0..count)
        .map(|_| Listed {
            description,
            when_to_use,
            withheld: false,
        })
        .collect()
}

/// Six entries of 1,524 characters list 9,144 — past the 8,000 budget — so the floor
/// draws one whole-grain advisory naming the total, and the run stays zero.
#[test]
fn a_skill_listing_over_the_listing_budget_draws_one_listing_budget_advisory() {
    let run = listing_run("listing-over", &listed(6, 1_024, 500));
    assert_eq!(
        run.output.matches(LISTING).count(),
        1,
        "an overrun listing draws one advisory over the whole selection, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("lists 9144 characters"),
        "the advisory names the summed listing, got:\n{}",
        run.output
    );
    assert!(
        run.ok,
        "the clause ships advisory ⇒ zero, got:\n{}",
        run.output
    );
}

/// The same six skills with one setting `disable-model-invocation: true` list five
/// entries, 7,620 characters, inside the budget: the withheld skill is not counted.
#[test]
fn a_skill_that_disables_model_invocation_is_not_counted_against_the_listing_budget() {
    let over = listing_run("listing-all-invocable", &listed(6, 1_024, 500));
    assert_eq!(
        over.output.matches(LISTING).count(),
        1,
        "six invocable skills overrun the budget, got:\n{}",
        over.output
    );

    let mut skills = listed(6, 1_024, 500);
    skills[0].withheld = true;
    let run = listing_run("listing-one-withheld", &skills);
    assert!(
        !run.output.contains(LISTING),
        "a skill that disables model invocation is not listed, so the rest fit, got:\n{}",
        run.output
    );
}

/// Entries of 3,024 raw characters each count 1,536, the cap the listing cuts them at:
/// five fit (7,680), six overrun by the capped sum (9,216), never the raw one.
#[test]
fn a_description_past_the_per_entry_cap_counts_only_its_first_1536_characters() {
    let five = listing_run("listing-capped-five", &listed(5, 1_024, 2_000));
    assert!(
        !five.output.contains(LISTING),
        "five capped entries fit the budget though their raw text does not, got:\n{}",
        five.output
    );

    let six = listing_run("listing-capped-six", &listed(6, 1_024, 2_000));
    assert!(
        six.output.contains("lists 9216 characters"),
        "six capped entries sum 6 × 1,536, got:\n{}",
        six.output
    );
}

/// Three skills and three commands of 1,524 characters each list 4,572 per kind — each
/// kind under the 8,000 budget alone — but 9,144 together, past it: the one listing spans
/// both kinds, so the root draws one advisory over the joint sum.
#[test]
fn skills_and_commands_that_overrun_the_listing_budget_only_together_draw_the_listing_advisory() {
    let skills_alone = listing_run("listing-skills-alone", &listed(3, 1_024, 500));
    assert!(
        !skills_alone.output.contains(LISTING),
        "three skills alone fit the budget, got:\n{}",
        skills_alone.output
    );
    let commands_alone = joint_listing_run("listing-commands-alone", &[], &listed(3, 1_024, 500));
    assert!(
        !commands_alone.output.contains(LISTING),
        "three commands alone fit the budget, got:\n{}",
        commands_alone.output
    );

    let joint = joint_listing_run(
        "listing-joint",
        &listed(3, 1_024, 500),
        &listed(3, 1_024, 500),
    );
    assert_eq!(
        joint.output.matches(LISTING).count(),
        1,
        "the joint overrun draws one advisory over the whole listing, got:\n{}",
        joint.output
    );
    assert!(
        joint.output.contains("lists 9144 characters"),
        "the advisory names the sum across both kinds, got:\n{}",
        joint.output
    );
    assert!(
        joint.ok,
        "the clause ships advisory ⇒ zero, got:\n{}",
        joint.output
    );
}

/// A file of `chars` rendered characters past its heading, in few long lines so the
/// 200-line size target stays silent and only the per-file limit can speak.
fn oversized(heading: &str, chars: usize) -> String {
    let line = format!("{}\n", "x".repeat(999));
    format!("# {heading}\n\n{}", line.repeat(chars / line.len() + 1))
}

/// Check a harness carrying one oversized file at `path`.
fn oversized_run(slug: &str, path: &str) -> common::CheckRun {
    let root = common::tmpdir(slug);
    common::write_sibling(&root, path, &oversized("Big", 40_001));
    common::write_lock(&root, Declarations::default());
    common::check_in(&root, &[], None)
}

/// A subdirectory's `CLAUDE.md` loads on demand, so the runtime cuts it past the
/// per-file limit: one advisory, and the run stays zero.
#[test]
fn a_subdirectory_memory_past_the_per_file_limit_draws_the_size_advisory() {
    let run = oversized_run("size-subdirectory-memory", "apps/web/CLAUDE.md");
    assert_eq!(
        run.output
            .matches("memory.when.loads=on-demand.extent")
            .count(),
        1,
        "the subdirectory memory draws one size advisory, got:\n{}",
        run.output
    );
    assert!(run.output.contains("truncated"), "{}", run.output);
    assert!(run.ok, "advisory ⇒ zero, got:\n{}", run.output);
}

/// A subdirectory's own rule loads on a read beneath it, so even without `paths` it is
/// on demand and judged against the per-file limit.
#[test]
fn a_nested_rule_without_paths_past_the_per_file_limit_draws_the_size_advisory() {
    let run = oversized_run("size-nested-rule", "apps/web/.claude/rules/x.md");
    assert_eq!(
        run.output
            .matches("rule.when.loads=on-demand.extent")
            .count(),
        1,
        "the nested rule draws one size advisory, got:\n{}",
        run.output
    );
    assert!(run.ok, "advisory ⇒ zero, got:\n{}", run.output);
}

/// The root `CLAUDE.md` loads at launch and arrives whole, so the same size draws no
/// per-file advisory.
#[test]
fn a_root_memory_past_the_per_file_limit_draws_no_size_advisory() {
    let run = oversized_run("size-root-memory", "CLAUDE.md");
    assert!(
        !run.output.contains("memory.when.loads=on-demand"),
        "a launch-loaded memory is not cut, got:\n{}",
        run.output
    );
    assert!(run.ok, "{}", run.output);
}

/// A subdirectory's `CLAUDE.local.md` loads on demand beside its `CLAUDE.md`, so the
/// runtime cuts it past the per-file limit: one advisory, and the run stays zero.
#[test]
fn a_subdirectory_claude_local_md_past_the_per_file_limit_draws_the_size_advisory() {
    let run = oversized_run("size-subdirectory-local-memory", "apps/web/CLAUDE.local.md");
    assert_eq!(
        run.output
            .matches("memory-local.when.loads=on-demand.extent")
            .count(),
        1,
        "the subdirectory local memory draws one size advisory, got:\n{}",
        run.output
    );
    assert!(run.ok, "advisory ⇒ zero, got:\n{}", run.output);
}

/// Check a harness whose memory at `importer` imports one oversized file no kind claims.
fn oversized_import_run(slug: &str, importer: &str) -> common::CheckRun {
    let root = common::tmpdir(slug);
    common::write_sibling(&root, importer, "# Memory\n\nSee @notes/big.md\n");
    let dir = std::path::Path::new(importer)
        .parent()
        .map_or_else(String::new, |dir| dir.to_string_lossy().into_owned());
    let target = if dir.is_empty() {
        "notes/big.md".to_string()
    } else {
        format!("{dir}/notes/big.md")
    };
    common::write_sibling(&root, &target, &oversized("Notes", 40_001));
    common::write_lock(&root, Declarations::default());
    common::check_in(&root, &[], None)
}

/// A file no kind claims, imported by a subdirectory's `CLAUDE.md`, loads with it on
/// demand, so the root sizes it against the per-file limit.
#[test]
fn an_unclaimed_import_target_of_a_subdirectory_memory_past_the_per_file_limit_draws_the_size_advisory()
 {
    let run = oversized_import_run("size-unclaimed-import", "apps/web/CLAUDE.md");
    assert_eq!(
        run.output
            .matches("root.when.loads=on-demand.extent")
            .count(),
        1,
        "the unclaimed import target draws one size advisory, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("apps/web/notes/big.md"),
        "the finding names the imported file, got:\n{}",
        run.output
    );
    assert!(run.ok, "advisory ⇒ zero, got:\n{}", run.output);
}

/// The root `CLAUDE.md` loads at launch, so the file it imports arrives whole and draws
/// no per-file advisory.
#[test]
fn an_unclaimed_import_target_of_the_root_memory_draws_no_size_advisory() {
    let run = oversized_import_run("size-root-import", "CLAUDE.md");
    assert!(
        !run.output.contains("root.when.loads=on-demand"),
        "a launch-loaded import is not cut, got:\n{}",
        run.output
    );
    assert!(run.ok, "{}", run.output);
}
