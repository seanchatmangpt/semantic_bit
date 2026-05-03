# Status as Field Truth

A field is a bounded location in memory, carrying meaning. **Status** is the metadata concerning that meaning's validity. 

A status does not describe *what* the data is. It describes *how* the data may be safely interacted with. It is an assertion of **Field Truth**.

If the status is unknown, the data is meaningless, regardless of the bytes contained within its boundary. A system must therefore establish status before it can admit data as meaning. By making status an explicit, 8-bit field, we separate the truth of the container from the contents of the payload.
