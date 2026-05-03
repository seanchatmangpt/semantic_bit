# Part VIII — CONSTRUCT as Lawful Change

While SPARQL `SELECT` allows us to extract tabular data from the graph, SPARQL `CONSTRUCT` provides a far more powerful capability: the lawful transformation of the graph itself.

A `CONSTRUCT` query takes an input graph, matches patterns against it, and generates an entirely *new* graph based on a specified template. This is not a mutation of the original data in place, but a functional transformation—taking one state and lawfully deriving the next.

This is critical for manufacture. Often, the graph defining our high-level requirements (the "intent") differs structurally from the graph required by our code generators (the "implementation"). 

Instead of writing custom scripts to bridge this gap, we write a `CONSTRUCT` query. The query formally declares how a requirement node translates into a set of implementation nodes. 

Because `CONSTRUCT` operates strictly within the bounds of RDF, its outputs can immediately be validated against a new set of SHACL shapes. This creates a pipeline of lawful changes: from abstract specification, through structural transformation, to concrete implementation details—every step fully typed and verifiable.
