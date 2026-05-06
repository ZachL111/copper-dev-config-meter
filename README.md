# copper-dev-config-meter

`copper-dev-config-meter` is a Rust project for Developer tools. It turns build a Rust toolkit that studies config behavior through windowed input fixtures, with late-data behavior checks and bounded memory input sets into a small local model with readable fixtures and a direct verification command.

## Reading Copper Dev Config Meter

Start with the README, then open `metadata/project.json` to check the constants behind the examples. After that, `fixtures/cases.csv` shows the compact path and `examples/extended_cases.csv` gives a wider look at the same rule.

## Purpose

I use this kind of project to make a rule visible before adding more machinery around it. The important part here is not the size of the codebase. It is that the input signals, scoring rule, fixture data, and expected output can all be checked in one sitting.

## Files Worth Reading

- `src`: primary implementation
- `tests`: verification harness
- `fixtures`: compact golden scenarios
- `examples`: expanded scenario set
- `metadata`: project constants and verification metadata
- `docs`: operations and extension notes
- `scripts`: local verification and audit commands
- `Cargo.toml`: Rust package metadata

## What It Does

- Includes extended examples for safe defaults, including `surge` and `degraded`.
- Documents repeatable output tradeoffs in `docs/operations.md`.
- Runs locally with a single verification command and no external credentials.
- Stores project constants and verification metadata in `metadata/project.json`.
- Adds a repository audit script that checks structure before running the language verifier.

## Design Sketch

The design is intentionally direct: parse or construct a signal, score it, classify it, and verify the expected branch. This makes the repository useful for studying developer tools behavior without needing a service or database unless the language project itself is SQL. The Rust code keeps ownership and data movement plain, which makes the tests useful for checking both behavior and API shape.

## Setup

Install Rust and run the commands from the repository root. The project does not need credentials or a hosted service.

## Fixture Notes

`boundary` is the first example I would inspect because it lands on the `review` path with a score of 117. The broader file also keeps `degraded` at -34 and `surge` at 231, which gives the model a useful low-to-high spread.

## Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

This runs the language-level build or test path against the compact fixture set.

## Verification

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/audit.ps1
```

The audit command checks repository structure and README constraints before it delegates to the verifier.

## Next Directions

- Add a loader for `examples/extended_cases.csv` and promote selected cases into the language test suite.
- Add a short report command that prints the score breakdown for a single scenario.
- Add malformed input fixtures so the failure path is as visible as the happy path.
- Add one more developer tools fixture that focuses on a malformed or borderline input.

## Limits

The scoring model is simple by design. More domain-specific behavior should be added through explicit adapters or extra fixture classes rather than hidden constants.
