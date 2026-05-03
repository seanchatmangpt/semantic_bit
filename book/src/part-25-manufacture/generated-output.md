# Generated Output

The manufacturing process must produce a predictable and structured output.

We do not scatter artifacts across the repository. Generated output is confined to specific directories, governed by the "Output Boundary." This boundary prevents generated files from polluting authorable source code and ensures that the manufacturing tool (like `unrdf`) can easily synchronize the state.

$$
\boxed{\textbf{Confine the output. Preserve the source.}}
$$

## 25.1 The Output Directory

Typical projects designate a specific folder for artifacts, such as `src/generated/` or `target/manufacture/`. Within this folder, the structure should reflect the semantic organization of the system, not the physical organization of the templates.

```text
src/
├── generated/
│   ├── mod.rs (Generated index)
│   ├── constants.rs
│   ├── fields.rs
│   └── records.rs
├── main.rs (Authored source)
└── lib.rs (Authored source)
```

## 25.2 The Generated Index

A manufactured output should include an index or a "Module Gateway" that provides a single point of entry for the rest of the system. In Rust, this is usually a `mod.rs` file that exports all the generated artifacts.

This index is itself a generated artifact. It is manufactured by querying the source graph for all active patterns and emitting the corresponding `pub mod` declarations.

## 25.3 Deterministic Output

Manufacture must be deterministic. Given the same source graph, query, and template, the generated output must be bit-for-bit identical every time it is run.

Deterministic output is required for:
1.  **Version Control**: Preventing "noise" commits where only timestamps or random IDs change.
2.  **Verification**: Ensuring that the hash of the output matches the receipt.
3.  **Distributed Building**: Allowing different machines to produce the same artifacts independently.

## 25.4 Handling Stale Artifacts

When a pattern is removed from the source graph, its corresponding artifact must be removed from the generated output.

A naive manufacturing tool only adds files. A semantic manufacturing tool **synchronizes** them. It identifies "Stale" artifacts—files in the output directory that are no longer supported by a declaration in the graph—and removes them.

$$
\boxed{\textbf{Truth in the graph requires the absence of stale artifacts in the field.}}
$$
