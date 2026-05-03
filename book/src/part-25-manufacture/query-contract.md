# Query Contract

A query is not a suggestion. It is a formal contract for the extraction of truth.

In the manufacturing loop, the Query Contract (expressed in SPARQL) defines exactly which semantic facts are needed to synthesize an artifact. It is the bridge between the high-level graph and the specific requirements of the template.

$$
\boxed{\textbf{The query contract bounds the inquiry to the admitted facts.}}
$$

## 25.1 The Selectivity of the Contract

A query must be specific. We do not use "Select All" (`SELECT *`). We select only the specific variables required by the template.

-   `?bit_name`
-   `?bit_position`
-   `?bit_description`

By naming these variables in the query, we define the "Interface" between the graph and the template. If the graph contains 50 properties for a bit but the query only selects 3, the template only ever sees those 3. This is **Information Hiding at the Semantic Layer**.

## 25.2 Deterministic Selection

The query contract must ensure a deterministic result set. If the results are used to generate a list of constants, the order of those constants must not change unless the graph changes.

We use the `ORDER BY` clause to guarantee stability:

```sparql
SELECT ?name ?pos
WHERE {
  ?bit a sb:SemanticBit ;
       sb:name ?name ;
       sb:position ?pos .
}
ORDER BY ?pos
```

Without an explicit order, the manufacturing tool might produce a different file every time it runs, even if the graph is the same. This violates the principle of **Deterministic Manufacture**.

## 25.3 Validation via Query

The query contract can also act as a validator. By using `FILTER` and `BIND`, we can ensure that the extracted data meets the structural requirements of the artifact.

If a query requires a bit position between 1 and 8, and the graph contains a bit at position 10, the query can be designed to exclude it or to emit a warning.

## 25.4 The Query as a Versioned Asset

Query contracts are stored as files in the repository. They are versioned alongside the source code and the graph. This allows us to see how the "Inquiry" has evolved over time.

If you change the query, you are changing the contract. This change will be reflected in the manufacturing receipt, alerting the system that the artifact's derivation has changed.

$$
\boxed{\textbf{The artifact is the product of the graph and its contract.}}
$$
