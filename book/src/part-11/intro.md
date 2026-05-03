# Part XI: Manufacture

The transition from the semantic field to the graph representation is only halfway to our goal. We now have a rigorous, machine-readable declaration of intent—the Triple—but intent must become action. It must become code.

Manufacture is the process of translating semantic assertions into functional software artifacts. It is not mere code generation; it is the lawful derivation of structure from a unified source of truth.

When we write Rust structs, TypeScript interfaces, or database schemas by hand, we introduce a translation gap. The mental model resides in our heads, and the code is a lossy transcription. The semantic bit demands that the model itself *be* the source. Manufacture closes the gap.

## The Manufacturing Pipeline

The pipeline is a deterministic function: `f(Graph) -> Code`.

1.  **Assertion (RDF)**: The domain expert defines the entities, properties, and relationships.
2.  **Constraint (SHACL)**: The system architect enforces the rules and boundaries of those assertions.
3.  **Extraction (SPARQL)**: We query the graph to isolate the specific shapes needed for a target artifact.
4.  **Emission (Templates)**: We map the extracted shapes into the syntax of a target language.

This pipeline ensures that every line of generated code has a direct lineage back to the semantic graph. If the graph changes, the code is remanufactured. The code is a byproduct of the truth, not the truth itself.
