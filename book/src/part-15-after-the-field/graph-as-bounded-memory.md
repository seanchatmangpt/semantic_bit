# The Graph as Bounded Memory

A collection of triples is called a **Graph**. 

In the Semantic Bit philosophy, a graph is not just a "database" or a "knowledge base." A graph is a **Bounded Memory**. 

Just as a field (Book 1) is a bounded carrier of bitwise state, a graph is a bounded carrier of structural relations. If the boundary is missing, the memory is untrusted.

---

## 15.1 The Law of the Boundary

A graph without a boundary is just noise. 

You cannot make an operational decision based on "any triple found on the internet." You make decisions based on triples that have been **admitted** into a specific graph boundary.

24651
\boxed{\textbf{The boundary is the law of the graph.}}
24651

Every graph in the system must define its **Admission Rule**. This rule answers:

1. **Who** is allowed to add triples to this graph?
2. **What** shapes must these triples follow? (SHACL)
3. **When** was this graph versioned and signed?

---

## 15.2 Named Graphs as Cognitive Units

We do not use one giant "everything" graph. We use **Named Graphs** to represent specific cognitive units of the system.

- **The Policy Graph**: Contains the triples that define access rules.
- **The Schema Graph**: Contains the triples that define the field structures.
- **The Evidence Graph**: Contains the triples that represent receipts and proofs.

By naming the graphs, we create a **Scoped Memory**. When the machine performs an inquiry, it does not look everywhere; it looks within the specific graphs admitted for that task.

---

## 15.3 The Graph Receipt

The "Meaning Before Motion" philosophy requires that the state of the graph itself be captured in a receipt.

A **Graph Receipt** (or Graph Digest) is a cryptographic proof of the exact triples present in a graph at a specific moment. 

| Layer | Meaning | Proof |
| :--- | :--- | :--- |
| **State** | Bit position in a field | Field Value |
| **Relation** | Triple in a graph | Graph Digest |
| **Consequence** | Selected Condition | Operation Receipt |

When we manufacture a system from a graph, we include the Graph Receipt in the output. This ensures that the generated code is immutably linked to the exact version of the blueprint that created it.

---

## 15.4 Bounded Memory in Rust

In the Rust library, the concept of the bounded graph is reflected in how we handle data ingestion. We do not "import strings"; we "admit triples."

The graph is the surface where the machine "thinks" before it acts. By bounding that memory, we ensure that the machine's thoughts are always grounded in admitted evidence and lawful structure. 

The graph is where the "meaning" of the Semantic Bit is stored. The boundary is what makes that meaning operational.
