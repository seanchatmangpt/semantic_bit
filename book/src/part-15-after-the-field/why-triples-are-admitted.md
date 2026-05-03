# Why Triples Are Admitted

We admit triples because they provide the smallest unit of verifiable relation. 

A single value (a bit) is too small to carry a relation. A collection of values (a record) is too large to be a universal primitive. The triple—Subject, Predicate, Object—is the "Semantic Atom".

$$
\boxed{\textbf{The triple is the minimum bounded record required to connect two identities.}}
$$

## 15.6 The Failure of the String

In traditional programming, relations are often hidden in prose:
`"User 123 has permission to read File 456"`
This string is a narrative. It requires parsing. It is fragile.

## 15.7 The Admission of the Triple

A triple admits the relation by breaking it into three numeric or named identities:
- `123` (Subject)
- `READ_PERMISSION` (Predicate)
- `456` (Object)

By admitting the relation as a triple, we make it machine-readable, indexable, and verifiable. We no longer guess the meaning of the string; we select the behavior associated with the predicate.
