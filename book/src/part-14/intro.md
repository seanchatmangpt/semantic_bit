# Part XIV: Book from Triples

Manufacture is not limited to executable code. Documentation itself must be a derived artifact. When documentation is written by hand, it drifts from the code. When documentation is manufactured from the same semantic graph as the code, drift is impossible.

## Documentation as a Manufactured Artifact

The same `unrdf` pipeline used to generate Rust structs can be aimed at generating Markdown.

Consider the vocabulary defined in the previous section. Instead of a Tera template targeting Rust syntax, we provide a template targeting Markdown:

```markdown
# Vocabulary Reference

{% for class in classes %}
## Class: {{ class.name }}

Properties:
{% for prop in class.properties %}
*   `{{ prop.name }}` (Type: `{{ prop.type }}`)
{% endfor %}

{% endfor %}
```

## The Single Source of Truth

This approach elevates documentation from an afterthought to a first-class citizen of the manufacturing process.

If we add a new property to the `Person` class in our RDF graph:
1.  The Rust manufacturing pipeline updates the struct.
2.  The Book manufacturing pipeline updates the mdBook chapter.
3.  The two artifacts remain perfectly synchronized because they share the same source of truth.

This is the ultimate promise of the semantic graph: a unified foundation from which all expressions of the system are lawfully derived.
