# Part XVI: Laws and Exercises

The principles of semantic manufacture are not suggestions; they are laws designed to prevent systemic failure.

## The Laws of Manufacture

1.  **The Law of the Single Source**: No structural truth shall exist solely in a manufactured artifact. If it is true, it must be in the graph.
2.  **The Law of Lawful Derivation**: All code generation must proceed deterministically from the graph via extraction and templates. Hand-edits to manufactured artifacts are strictly forbidden.
3.  **The Law of Invalidation**: If the graph changes, all derived artifacts are instantly invalid until remanufactured.
4.  **The Law of Semantic Adhesion**: The structure of the generated code must reflect the structure of the graph. Do not obfuscate the semantic intent during translation.

## Exercises

1.  **Extend the Vocabulary**: Add a `Company` class to the RDF graph from Part XIII. Give it a `name` and an `employee_count`. Rerun the `unrdf` pipeline to generate the updated Rust structs.
2.  **Shape Validation**: Write a SHACL shape to enforce that `employee_count` must be a positive integer.
3.  **New Target**: Create a new template to generate a SQL schema (CREATE TABLE statements) from the same RDF graph.

Mastery of these laws and exercises ensures that you are no longer merely writing code; you are manufacturing reality from truth.
