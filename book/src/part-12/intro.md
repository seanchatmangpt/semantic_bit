# Part XII: unrdf

To bridge the gap between the semantic graph and the manufacturing pipeline, we introduce `unrdf`.

`unrdf` is not a compiler; it is an extraction engine. It takes an RDF graph and a set of SPARQL templates and yields structured data (typically JSON or YAML) ready for consumption by standard templating engines.

## The unrdf Philosophy

The core philosophy of `unrdf` is separation of concerns in the manufacturing process:

*   **The Graph** knows what is true.
*   **SPARQL** knows how to ask for it.
*   **unrdf** handles the mechanics of the query and the serialization of the result.
*   **Templates** (e.g., Handlebars, Tera, Jinja) know how to format the result into syntax.

By isolating the extraction phase, `unrdf` allows us to decouple our knowledge representation from our target languages. A single RDF graph can feed a Rust generator, a Python generator, and a Markdown generator simultaneously, simply by varying the SPARQL queries and templates applied by `unrdf`.

## Core Mechanisms

1.  **Ingestion**: `unrdf` parses RDF serializations (Turtle, N-Triples, JSON-LD) into an in-memory graph.
2.  **Query Execution**: It applies parametrized SPARQL `SELECT` or `CONSTRUCT` queries.
3.  **Shaping**: It processes the tabular SPARQL result sets into nested JSON structures, applying grouping and mapping defined in its configuration.

`unrdf` turns the graph into a predictable data source for our manufacturing tools.
