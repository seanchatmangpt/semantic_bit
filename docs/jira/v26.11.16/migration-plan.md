# semantic_bit migration plan v26.11.16

Standing: UNVERIFIED

## Identity
- Path: /Users/sac/semantic_bit
- Remote: https://github.com/seanchatmangpt/semantic_bit.git (upstream origin/main)
- Branch: main@cc6af50, last commit 2026-07-02, 21 commits, 7 MB
- Languages/build: Rust (Cargo.toml), Justfile, mdBook (book.toml), unrdf.toml, ontology/
- Nested repos: none

## Observed state
- Dirty 0; untracked 3: Justfile, docs/ (contains docs/jira/v26.9.19)
- Ahead 0 / behind 0; stash 0; no unpushed branches; no build dirs; no submodules

## What it is
"Semantic8 Spine" — an mdBook-style authored text plus a unrdf manufacturing pipeline over an
ontology; Rust crate with publication metadata. No README; purpose from commit titles.

## Classification
WARM (census agrees).

## Migration risk
LOW — only the untracked Justfile and docs/jira folder are local-only.

## Recommended action
COMMIT-PUSH
1. `cd /Users/sac/semantic_bit && git add Justfile docs && git commit -m "docs: jira + Justfile"`
2. `git push origin main`
3. Re-clone on new machine (no build dirs to clean).

## First commands on the new machine
`cargo check` (and `just` list if Justfile targets exist)

## See Also
- ../v26.9.19/
- /Users/sac/docs/jira/v26.11.16/README.md
