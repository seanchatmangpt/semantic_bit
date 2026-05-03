# Authorship and Drift

Authorship is the declaration of intent. Drift is the decay of that intent over time.

In traditional software engineering, authorship is applied to everything: logic, formatting, boilerplate, and documentation. When a human authors every layer, they become the bottleneck for consistency.

$$
\boxed{\textbf{Human authorship is the source of all drift.}}
$$

## 25.1 The Nature of Drift

Drift occurs when two representations of the same meaning diverge.

-   **Logic vs. Documentation**: The code says 8 bits, but the README says 16.
-   **Structure vs. Validation**: The database allows NULL, but the Rust struct requires a value.
-   **Intent vs. Implementation**: The designer intended a strict selection rule, but the developer implemented a loose `if/else` chain.

Drift is not a sign of incompetence. It is a mathematical certainty in any system where the same meaning is authored multiple times in different languages.

## 25.2 Restricting Authorship

The Semantic Bit solves drift by restricting human authorship to the **Source Graph**.

The human authors the *meaning* (the bits, the relations, the rules). The machine authors the *representation* (the Rust code, the Markdown, the SQL).

By removing the human from the representation layer, we eliminate the possibility of the representation diverging from the intent.

## 25.3 Authorship of the Law

When we speak of "authoring" in this book, we mean the authorship of the law.

-   Declaring that `BADGE_PRESENT` is Position 1.
-   Declaring that `GRANT` requires `HOLDER_ACTIVE`.
-   Declaring the shape of an `AccessReceipt`.

These are acts of authorship. They require judgment, domain expertise, and a sense of "Meaning Before Motion."

## 25.4 The End of Manual Maintenance

Manual maintenance is a symptom of failed manufacture. If you find yourself manually updating a constant in three different files, you have admitted drift into your system.

$$
\boxed{\textbf{If it can be manufactured, it must not be authored.}}
$$

The goal of the manufacturing process is to reach a state where the only files in the repository that are not generated are the source graphs, the queries, and the templates. Everything else is a temporary artifact of the manufacturing loop.
