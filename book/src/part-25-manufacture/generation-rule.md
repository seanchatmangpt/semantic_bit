# Generation Rule

A generation rule is the formal link between a semantic inquiry and a physical synthesis.

It is the "recipe" for manufacture. A rule declares which query contract to execute against the source graph and which template contract to apply to the results. It also defines where the resulting artifact should be placed.

$$
\boxed{\textbf{The generation rule binds the 'What' of the query to the 'How' of the template.}}
$$

## 25.1 Anatomy of a Rule

A generation rule consists of four mandatory components:

1.  **Name**: A unique identifier for the rule (e.g., `manufacture-rust-fields`).
2.  **Query Path**: The location of the SPARQL query file.
3.  **Template Path**: The location of the template file.
4.  **Output Path**: The target location for the generated artifact.

In a configuration file for a manufacturing tool (like `unrdf.yml`), a rule might look like this:

```yaml
rules:
  - name: rust-constants
    query: queries/all-constants.sparql
    template: templates/rust-constants.tera
    output: src/generated/constants.rs
```

## 25.2 Rule Scope

A rule can be **Singular** or **Multiplexed**.

-   **Singular Rule**: Produces one artifact from one query result (e.g., a single file containing all constants).
-   **Multiplexed Rule**: Produces many artifacts from a set of query results (e.g., one file for every pattern cell found in the graph).

Multiplexing is powerful but requires a stable naming convention for the output files to avoid collisions.

## 25.3 Conditional Generation

A rule may include guards. If the query returns no results, the rule may be skipped, or it may produce a "Null Artifact" (an empty file or a file with a specific "Empty" warning).

This ensures that the output directory only contains artifacts that are actually supported by evidence in the graph.

## 25.4 The Rule as Law

A generation rule is not just configuration. It is a declaration of how the law of the system should be manifested in the physical world. If you change a rule, you change the manifest form of the system.

Any change to a generation rule MUST be recorded in the manufacturing receipt.

$$
\boxed{\textbf{The rule is the bridge. The receipt is the proof that the bridge was crossed.}}
$$
