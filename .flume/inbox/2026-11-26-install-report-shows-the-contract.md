# Install's discovery report shows what the built-in default contract says about the members

`specs/model/adoption.md`, "Install": the report opens "findings first … what the walk finds, members by kind, and what the built-in default contract says about them". Today `render_discovery` (`src/install.rs:812-831`) prints only a count per kind, so an adopter decides whether to represent the harness without seeing a finding. The census marked the claim *(pending)*. The report runs the built-in contract over the discovered members, as a bare `check` does, and leads with those findings before the represent question. The *(pending)* mark comes off at the cut that ships this. Acceptance:
- **new:** an install over a harness with a skill that has no description shows that finding in the report, before the question;
- **kept:** the per-kind counts still print.
