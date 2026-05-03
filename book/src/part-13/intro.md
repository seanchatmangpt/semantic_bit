# Part XIII: Rust from Triples

We now demonstrate the end-to-end manufacture of Rust structures from our semantic graph.

## The Goal

We want to define a `Person` entity with a `name` and an `age` in RDF, and automatically generate the corresponding Rust `struct`:

```rust
// This should be generated
pub struct Person {
    pub name: String,
    pub age: u32,
}
```

## The Workflow

1.  **The Vocabulary (Turtle)**:
    ```turtle
    @prefix ex: <http://example.org/> .
    @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
    @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

    ex:Person a rdfs:Class .
    ex:name a rdfs:Property ; rdfs:domain ex:Person ; rdfs:range xsd:string .
    ex:age a rdfs:Property ; rdfs:domain ex:Person ; rdfs:range xsd:integer .
    ```

2.  **The Extraction (SPARQL)**:
    We write a query to find all classes and their properties.
    ```sparql
    SELECT ?className ?propName ?propType
    WHERE {
      ?class a rdfs:Class .
      BIND(REPLACE(STR(?class), "^.*//[^/]+/(.*)$", "$1") AS ?className)
      
      ?prop rdfs:domain ?class ;
            rdfs:range ?range .
      BIND(REPLACE(STR(?prop), "^.*//[^/]+/(.*)$", "$1") AS ?propName)
      BIND(REPLACE(STR(?range), "^.*#", "") AS ?propType)
    }
    ```

3.  **The Manufacture (unrdf + Tera)**:
    `unrdf` executes the query and passes the nested results to a Tera template:
    ```tera
    {% for class in classes %}
    pub struct {{ class.name }} {
    {% for prop in class.properties %}
        pub {{ prop.name }}: {{ map_type(prop.type) }},
    {% endfor %}
    }
    {% endfor %}
    ```

The result is perfectly formed Rust code, derived directly from the semantic truth.
