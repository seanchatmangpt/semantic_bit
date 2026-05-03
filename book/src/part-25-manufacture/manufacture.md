# Manufacture

Code is not written by hand when the law is already declared.

Manufacture is the process of transforming semantic declarations into executable artifacts. In the Semantic Bit, we do not "write" the critical path; we "manufacture" it from the evidence graph.

$$
\boxed{\textbf{Authorship is for the law. Manufacture is for the machine.}}
$$

## 25.1 The Manufacturing Loop

The manufacturing loop consists of three distinct phases:

1.  **Declaration**: Defining the patterns, fields, and relations in an RDF graph.
2.  **Inquiry**: Querying the graph to extract the specific requirements for an artifact.
3.  **Synthesis**: Applying templates to the query results to generate source code.

This loop ensures that the executable code is always a faithful reflection of the declared semantic law.

## 25.2 Why We Manufacture

Hand-written code is prone to "Drift." A developer might update a struct definition but forget to update the corresponding documentation or the database schema.

Manufacture eliminates drift. When the source graph is updated, all dependent artifacts—Rust structs, mdBook chapters, SQL schemas—are regenerated simultaneously.

## 25.3 The Inputs of Manufacture

Manufacture requires three formal inputs:

| Input              | Purpose                                           |
| :----------------- | :------------------------------------------------ |
| **Source Graph**   | The admitted truth of the system (RDF).           |
| **Query Contract** | The precise extraction of truth (SPARQL).         |
| **Template**       | The physical form of the output (Tera/Jinaj2).    |

If any of these inputs are missing or unverified, manufacture cannot proceed.

## 25.4 The Output is an Artifact

The result of manufacture is not a "file." It is an **Artifact**. An artifact is a generated object that carries its own manufacturing receipt. This receipt allows the system to verify that the artifact was produced according to the declared rules and has not been tampered with.

$$
\boxed{\textbf{We do not trust the file. We trust the receipt of its manufacture.}}
$$
