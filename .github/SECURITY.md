# Security policy

## Reporting a vulnerability

Please report suspected vulnerabilities **privately**, not through a public
issue. Use GitHub's private vulnerability reporting: on the repository's
**Security** tab, choose **Report a vulnerability**. This opens a private
advisory visible only to you and the maintainer.

Do not open a public issue, and do not disclose the details anywhere public,
until a fix has shipped and you have been told it is safe to do so.

## The evidence bar: demonstrate, don't speculate

A report must **demonstrate** the vulnerability, not describe a way one might
exist. Unverified, assistant-generated vulnerability reports have become the new
spam: plausible-sounding prose about a flaw that was never reproduced. Reports
that cannot clear this bar will be closed without further investigation.

A report we can act on includes:

- **A concrete reproduction.** The exact input, files, or command sequence that
  triggers the issue, runnable against a stated version.
- **The observed impact.** What actually happens (crash, data exposure,
  arbitrary write, etc.), not what could theoretically follow.
- **The version and environment.** The `temper` version or commit, and the OS.

"An AI flagged this pattern as potentially unsafe" is not a report. A minimal
case that shows the pattern being exploited is.

## Scope

`temper` is an I/O-bound tool that reads and rewrites local harness
configuration files. The security surface that matters is what it does with
untrusted *input files*: parsing, path handling, and write-back. Findings there
are in scope. General dependency-advisory noise without a demonstrated path to
impact in `temper` is not.

## Handling

We will acknowledge a report that clears the evidence bar, confirm the issue,
and work a fix on a private advisory before any public disclosure. Because this
is a single-maintainer project, please allow reasonable time before considering
coordinated public disclosure.
