# Part XV: Worked Manufacturing Examples

This section provides complete, executable examples of the manufacturing workflows discussed in the preceding chapters.

## Example 1: The Status Field Generator

In Book 1, we laboriously defined status fields bit by bit. Now, we will define a status field in RDF and manufacture its implementation.

1.  **The RDF Definition**: We define a `ConnectionStatus` field with states for `Connecting`, `Connected`, and `Disconnected`.
2.  **The Validation**: We write SHACL shapes to ensure that `ConnectionStatus` never has overlapping states.
3.  **The Query**: We use SPARQL to extract the states and their bitmasks.
4.  **The Target**: We use `unrdf` and templates to generate both the Rust implementation (using bitwise operators) and the Markdown documentation explaining the field.

*(Detailed code listings and Makefile targets for running this example are provided in the source repository.)*

## Example 2: The API Contract

We extend the manufacturing pipeline to interface boundaries.

1.  **The RDF Definition**: We define a set of request and response payloads using RDF and SHACL.
2.  **The Manufacture**: We generate the Rust DTOs, the OpenAPI specification, and the client-side TypeScript definitions simultaneously.

These examples demonstrate that semantic manufacture scales from the single bit to the entire system boundary.
