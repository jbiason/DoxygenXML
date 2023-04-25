# DoxygenXML

Parse Doxygen XML files

## What does it do?

The idea behind DoxygenXML is to parse the directory with the XML produced by
Doxygen and give an iterator over the objects.

## Example

```rust
for element in DoxygenXML(path_to_xml) {
    match element {
        Namespace(name) => println!("Found a namespace named {name}"),
        Class(class) => {
            let name = class.name;
            println!("Found a classed named {name}");
        }
        ...
    }
}
```
