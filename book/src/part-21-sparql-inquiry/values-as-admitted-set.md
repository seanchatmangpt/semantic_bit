# VALUES as Admitted Set

An inquiry often begins with known constants. A badge ID, a room number, or a specific set of allowed status codes. 

The `VALUES` clause is the mechanism for **admission of external constants** into the inquiry boundary.

## 21.1 Injection of Meaning

We do not query "all badges" and then look for the one we have. We admit the badge we have into the inquiry.

```sparql
SELECT ?status
WHERE {
  VALUES ?badge { :Badge_101 }
  ?badge :hasStatus ?status .
}
```

In this example, the inquiry is immediately bounded to `:Badge_101`. The `VALUES` clause admits this identifier into the `?badge` position.

$$
\boxed{\textbf{Admission precedes inquiry.}}
$$

---

## 21.2 Admitting Sets

`VALUES` can admit multiple bindings at once. This is useful for checking a set of admitted conditions.

```sparql
VALUES ?allowedStatus { :Active :Probation :Contractor }
```

When used in a query, this acts as a structural guard. Only badges whose status is one of the `?allowedStatus` values will be admitted to the result.

---

## 21.3 VALUES vs. BIND

`BIND` assigns a value to a variable *during* the execution of the pattern match. `VALUES` admits the value *before* the pattern match begins.

*   Use `VALUES` for input parameters and known constants.
*   Use `BIND` for calculated values or renaming within the inquiry.

By using `VALUES`, we make the external dependencies of an inquiry explicit and verifiable.

---

## 21.4 The Empty Set as a Block

If a `VALUES` clause is empty, the entire inquiry is blocked.

```sparql
VALUES ?badge { }
```

This is a powerful safety feature. If a system fails to provide a badge ID, the `VALUES` clause will be empty, and the SPARQL engine will return no results. The inquiry boundary is preserved by the absence of admission.

$$
\boxed{\textbf{No admission, no match.}}
$$
