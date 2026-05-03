# Part VII — SPARQL as Field Inquiry

With our graphs populated with triples and verified by SHACL shapes, we require a mechanism to interrogate them. For this, we employ SPARQL (SPARQL Protocol and RDF Query Language).

SPARQL is to the graph what bitwise masking is to the semantic field. It allows us to pinpoint exactly the relationships and nodes we care about. 

A SPARQL `SELECT` query operates by pattern matching. We define a subgraph pattern with variables (e.g., `?field`, `?length`), and the query engine returns all bindings in the graph that satisfy the pattern.

For example, we can query our architectural graph for all status fields that span more than 4 bits:
```sparql
SELECT ?field ?length
WHERE {
  ?field a sembit:StatusField ;
         sembit:hasLength ?length .
  FILTER(?length > 4)
}
```

This capability transforms our static specifications into a dynamic database. We can write queries to check for orphans, to list all dependencies of a specific module, or to extract the exact data needed by a code generation template. SPARQL elevates our specification from a static document to an active, verifiable inquiry system.
