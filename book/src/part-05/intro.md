# Part V — Vocabulary and Shape

With RDF as our grammar, we must define our vocabulary—the specific IRIs and terms we use to describe our domain. A vocabulary is the agreed-upon set of predicates and classes that give meaning to our architecture.

For example, we might define `sembit:Channel` or `sembit:multiplexes`. These terms must be strictly managed to prevent semantic drift. 

However, vocabulary alone is insufficient. We must also enforce the **shape** of our data. A node claiming to be a `sembit:StatusField` must possess certain mandatory properties—it must have a bit offset, a length, and a reference to its containing record. 

Shapes constrain the graph, ensuring that it is not merely a collection of valid triples, but a collection of triples that form a valid *system*. Without shapes, our graphs could easily become internally inconsistent, rendering them useless for automated manufacture. In the next section, we will explore how we enforce these shapes, ensuring our specifications remain as disciplined as the semantic fields they describe.
