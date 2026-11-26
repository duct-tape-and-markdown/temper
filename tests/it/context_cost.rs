//! Pins the skill floor's listing budget advisory: the model-invocable skills'
//! `description` + `when_to_use`, each entry cut at 1,536 characters, judged as one sum
//! against the runtime's 8,000-character listing budget.

use temper::drift::Declarations;

use crate::common;

/// The label the listing budget advisory reports under.
const LISTING: &str = "skill.listing.description+when_to_use";

/// One skill to list: its `description` and `when_to_use` lengths, and whether it
/// disables model invocation.
struct Listed {
    description: usize,
    when_to_use: usize,
    withheld: bool,
}

/// Check a harness carrying one `.claude/skills/<name>/SKILL.md` per listed skill.
fn listing_run(slug: &str, skills: &[Listed]) -> common::CheckRun {
    let root = common::tmpdir(slug);
    for (index, skill) in skills.iter().enumerate() {
        let name = format!("listed-{index}");
        let withheld = if skill.withheld {
            "disable-model-invocation: true\n"
        } else {
            ""
        };
        common::write_skill(
            &root,
            &name,
            &format!(
                "---\nname: {name}\ndescription: {}\nwhen_to_use: {}\n{withheld}---\n# {name}\n",
                "d".repeat(skill.description),
                "w".repeat(skill.when_to_use),
            ),
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
