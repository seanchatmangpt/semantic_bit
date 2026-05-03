# Generated Artifact

An artifact is the concrete output of a manufacturing process.

In the Semantic Bit, we do not call generated files "code." We call them artifacts to remind the engineer that they are products of a manufacturing line. They are not to be edited by hand.

$$
\boxed{\textbf{An artifact is a frozen reflection of a semantic law.}}
$$

## 25.1 The Protection of the Artifact

Every artifact must carry a warning that it is generated. This is not a courtesy; it is a boundary of trust.

If an engineer modifies a generated artifact, they have broken the link between the semantic law and the executable notation. This is known as **Manual Intervention Drift**, and it is a critical failure of the manufacturing loop.

Typical protection includes:
-   **Header Warnings**: Comments at the top of the file (e.g., `// GENERATED - DO NOT EDIT`).
-   **Read-Only Flags**: Files system attributes that prevent writing.
-   **Checkstep Gates**: CI/CD tools that verify the hash of the artifact against the manufacturing receipt.

## 25.2 Artifact Identity

An artifact carries the identity of its source. In a Rust artifact, this might be expressed as an attribute:

```rust
#[semantic_artifact(source = "sb:pattern/access-field", version = "1.2.0")]
pub struct AccessField(u8);
```

This identity allows for **Back-Tracing**. If a bug is found in the behavior of the `AccessField`, we do not fix the artifact. We trace back to the source graph or the manufacturing template and fix the law.

## 25.3 Ephemeral vs. Persistent Artifacts

Some artifacts are ephemeral. They exist only during the build process and are never committed to version control.

Other artifacts are persistent. They are committed to the repository to allow for auditing, code review, and easier distribution. Both types are equally "generated" and must be treated with the same adversarial trust.

## 25.4 The Artifact Receipt

The most important part of an artifact is its receipt. The receipt captures:
1.  The hash of the source graph.
2.  The hash of the query contract.
3.  The hash of the template.
4.  The hash of the resulting artifact.

If these four hashes do not align, the artifact is invalid. It is not an admitted part of the system.

$$
\boxed{\textbf{Without a receipt, an artifact is merely a file.}}
$$
