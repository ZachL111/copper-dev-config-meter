# copper-dev-config-meter

`copper-dev-config-meter` explores developer tools with a small Rust codebase and local fixtures. The technical goal is to build a Rust toolkit that studies config behavior through windowed input fixtures, with late-data behavior checks and bounded memory input sets.

## Use Case

The point is to make a small domain rule concrete enough that a reader can change it and immediately see what broke.

## Copper Dev Config Meter Review Notes

For a quick review, compare `diagnostic quality` with `review cost` before reading the middle cases.

## Highlights

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/copper-dev-config-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `diagnostic quality` and `review cost`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Code Layout

The repository has two validation layers: the original compact policy fixture and the domain review fixture. They are separate so one can change without hiding failures in the other.

The added Rust path is deliberately direct, with fixtures doing most of the explaining.

## Run The Check

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Regression Path

The check exercises the source code and the review fixture. `stress` is the high score at 253; `edge` is the low score at 166.

## Future Work

The repository is intentionally scoped to local checks. I would expand it by adding adversarial fixtures before adding features.
