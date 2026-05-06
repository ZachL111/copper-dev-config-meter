# Copper Dev Config Meter Walkthrough

The fixture is intentionally compact, so the review starts with the cases that pull farthest apart.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 190 | ship |
| stress | diagnostic quality | 253 | ship |
| edge | review cost | 166 | ship |
| recovery | safe rewrite | 221 | ship |
| stale | change width | 187 | ship |

Start with `stress` and `edge`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

`stress` is the optimistic case; use it to make sure the scoring path still rewards strong signal.
