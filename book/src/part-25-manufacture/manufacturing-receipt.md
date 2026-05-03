# Manufacturing Receipt

A system that manufactures its own code must be able to prove that it did so correctly.

The manufacturing receipt is the definitive evidence of a successful manufacturing cycle. It is an immutable record that captures the state of all inputs and the resulting artifacts.

$$
\boxed{\textbf{The receipt is the proof of the loop.}}
$$

## 25.1 The Content of the Receipt

A manufacturing receipt is a JSON or RDF record containing:

1.  **Context**: The version of the manufacturing tool (e.g., `unrdf 0.5.0`) and the timestamp of the run.
2.  **Input Evidence**: The cryptographic hashes (e.g., SHA-256) of:
    -   The entire Source Graph.
    -   All Query Contracts used.
    -   All Template Contracts used.
3.  **Output Evidence**: A map of generated artifact paths to their cryptographic hashes.
4.  **Verification Signature**: (Optional) A digital signature from the manufacturing tool or the CI/CD environment.

## 25.2 The Role of the Receipt in Trust

We do not trust the generated code because we can read it. We trust it because the receipt proves it was generated from a source graph that we have already admitted.

If an artifact is found in the repository without a corresponding entry in the latest manufacturing receipt, it is an **Unauthenticated Artifact**. It must be rejected by the build system.

## 25.3 Receipt-Driven Verification

The build system (or a "Doctor" gate) uses the receipt to verify the integrity of the workspace:

1.  Calculate the hash of every file in the `generated/` directory.
2.  Compare these hashes to the hashes stored in the receipt.
3.  If any hash differs, the artifact has been tampered with or is out of sync.
4.  Fail the build.

## 25.4 Replaying the Manufacture

A receipt allows for a **Replay Audit**. Given a receipt, an external auditor can gather the exact same source graph, queries, and templates, and run the manufacturing tool again. If the resulting artifacts do not match the hashes in the receipt, the manufacturer has committed **Manufacturing Fraud**.

$$
\boxed{\textbf{Auditability is the consequence of receipt-driven manufacture.}}
$$

The receipt ensures that "Meaning Before Motion" is preserved not just in the logic of the system, but in the very process of its creation.
