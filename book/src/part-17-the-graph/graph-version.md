# Graph Version

A graph is not static. It changes as facts are added or removed. 

The **Graph Version** is the identity of the graph at a specific point in time.

$$
\boxed{\textbf{A version is a snapshot of the graph boundary.}}
$$

## 17.13 Immutability and Versioning

In a semantic system, we often treat graphs as immutable. Instead of "updating" a graph, we create a new version (a new named graph) that contains the delta. 

## 17.14 Version Receipts

Every version change must be accompanied by a receipt. This receipt links the old version to the new version and preserves the triples that were added or removed. This creates a **Version Chain** that allows us to audit the entire history of the system's memory.
