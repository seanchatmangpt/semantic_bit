# SPARQL Exercises

Apply the laws of inquiry to the following problems.

## Exercise 21.1: The Bounded Inquiry

Write a SPARQL inquiry that selects the `:hasStatus` relation for a specific `:badgeId` admitted via a `VALUES` clause. Ensure that only `AccessBadge` types are matched.

## Exercise 21.2: The Security Guard

Extend the inquiry from Exercise 21.1. Only project the status if the badge is assigned to a person whose role is `:SecurityGuard`. Use structural patterns in the `WHERE` clause.

## Exercise 21.3: The Expiry Guard

Add a `FILTER` guard to the inquiry. Only admit the result if the `:expiryDate` of the badge is greater than the current time (represented by a variable `?now`).

## Exercise 21.4: Deterministic Logs

You are tasked with projecting a list of `:AuditLog` entries for a specific `:room`.
1. Admit the `:room` IRI using `VALUES`.
2. Match all log entries related to that room.
3. Order the results by timestamp (descending) and then by the log entry's own IRI.

Explain why ordering by IRI is necessary even if you are already ordering by timestamp.

---

## Solutions

### Solution 21.1
```sparql
SELECT ?status
WHERE {
  VALUES ?badge { :Badge_101 }
  ?badge a :AccessBadge ;
         :hasStatus ?status .
}
```

### Solution 21.2
```sparql
SELECT ?status
WHERE {
  VALUES ?badge { :Badge_101 }
  ?badge a :AccessBadge ;
         :hasStatus ?status ;
         :assignedTo ?person .
  ?person :hasRole :SecurityGuard .
}
```

### Solution 21.3
```sparql
SELECT ?status
WHERE {
  VALUES ?badge { :Badge_101 }
  VALUES ?now { "2024-05-20T10:00:00Z"^^xsd:dateTime }
  ?badge a :AccessBadge ;
         :hasStatus ?status ;
         :expiryDate ?expiry .
  FILTER(?expiry > ?now)
}
```

### Solution 21.4
```sparql
SELECT ?logEntry
WHERE {
  VALUES ?room { :Room_404 }
  ?logEntry a :AuditLog ;
            :locatedIn ?room ;
            :timestamp ?ts .
}
ORDER BY DESC(?ts) ?logEntry
```
*Reasoning*: Multiple log entries might occur at the exact same timestamp (especially in high-frequency systems). Without ordering by the `?logEntry` IRI as a tie-breaker, the relative order of these entries would be non-deterministic.
