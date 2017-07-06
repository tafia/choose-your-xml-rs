# Choose your XML

## Crates

| | |
| --- | --- |
| [xml-rs](https://crates.io/crates/xml-rs) | ![Version][xml-rs-version] ![CI][xml-rs-travis] |
| [quick-xml](https://crates.io/crates/quick-xml) | ![Version][quick-xml-version] ![CI][quick-xml-travis] |
| [xml5ever](https://crates.io/crates/xml5ever) | ![Version][xml5ever-version] ![CI][xml5ever-travis] |
| [sxd-document](https://crates.io/crates/sxd-document) | ![Version][sxd-document-version] ![CI][sxd-document-travis] |

[xml-rs-version]: https://img.shields.io/crates/v/xml-rs.svg
[quick-xml-version]: https://img.shields.io/crates/v/quick-xml.svg
[xml5ever-version]: https://img.shields.io/crates/v/xml5ever.svg
[sxd-document-version]: https://img.shields.io/crates/v/sxd-document.svg

[xml-rs-travis]: https://img.shields.io/travis/netvl/xml-rs.svg
[quick-xml-travis]: https://img.shields.io/travis/tafia/quick-xml.svg
[xml5ever-travis]: https://img.shields.io/travis/servo/html5ever.svg
[sxd-document-travis]: https://img.shields.io/travis/shepmaster/sxd-document.svg

## Features

| Feature/Crate                 | xml-rs           | quick-xml        | xml5ever         | sxd-document     |
| ----------------------------- | ---------------- | ---------------- | ---------------- | ---------------- |
| [Document type definition]    | No               | No               | [Partial][1:3]   | Partial          |
| Namespaces                    | Yes              | Yes              | ?                | Yes              |
| `xml:space`                   | No               | No               | No               | No               |
| [Numeric character reference] | Yes              | Yes              | Yes              | Yes              |
| [XML entity reference]        | Yes              | Yes              | Yes              | Yes              |
| [HTML entity reference]       | No               | No               | Yes              | No               |
| [DTD entity reference]        | No               | No               | No               | No               |
| Non UTF-8 input               | No               | No               | No               | No               |
| [XPath]                       | No               | No               | No               | [Yes][7:4]       |
| [XQuery]                      | No               | No               | No               | No               |
| Event parser                  | Yes              | Yes              | Yes              | No               |
| Push parsing                  | No               | No               | Yes              | No               |
| Error recovery                | No               | No               | Partial          | No               |
| DOM                           | No               | No               | [Yes][11:3]      | Yes              |
| Writing                       | Yes              | Yes              | Yes              | Yes              |

[Document type definition]: https://en.wikipedia.org/wiki/Document_type_definition
[Numeric character reference]: https://en.wikipedia.org/wiki/Numeric_character_reference
[XPath]: https://en.wikipedia.org/wiki/XPath
[XQuery]: https://en.wikipedia.org/wiki/XQuery
[XML entity reference]: https://en.wikipedia.org/wiki/List_of_XML_and_HTML_character_entity_references#Predefined_entities_in_XML
[HTML entity reference]: https://en.wikipedia.org/wiki/List_of_XML_and_HTML_character_entity_references#Character_entity_references_in_HTML
[DTD entity reference]: https://en.wikipedia.org/wiki/List_of_XML_and_HTML_character_entity_references#Character_reference_overview

[1:3]: https://github.com/servo/html5ever/blob/master/xml5ever/README.md#when-you-shouldnt-use-it
[11:3]: https://github.com/servo/html5ever/blob/master/markup5ever/rcdom.rs
[7:4]: https://crates.io/crates/sxd-xpath

You can test features by yourself using corresponding example:

```bash
cargo run --example crate_name file_path

# example
cargo run --example quick_xml data/small.xml
```

## Performance

```
test quick_xml_large     ... bench:   1,899,961 ns/iter (+/- 19,762)
test quick_xml_medium    ... bench:     606,169 ns/iter (+/- 12,840)
test quick_xml_small     ... bench:       6,861 ns/iter (+/- 337)

test sxd_document_large  ... bench:      failed
test sxd_document_medium ... bench:   2,556,833 ns/iter (+/- 53,307)
test sxd_document_small  ... bench:      39,422 ns/iter (+/- 1,562)

test xml5ever_large      ... bench:   8,318,061 ns/iter (+/- 38,335)
test xml5ever_medium     ... bench:   7,403,966 ns/iter (+/- 39,070)
test xml5ever_small      ... bench:      44,433 ns/iter (+/- 528)

test xmlrs_large         ... bench:  26,283,106 ns/iter (+/- 155,005)
test xmlrs_medium        ... bench:  13,513,665 ns/iter (+/- 15,710)
test xmlrs_small         ... bench:      92,391 ns/iter (+/- 149)
```

You can run benchmarks by yourself using `cargo bench`.

\* Note that `sxd-document` generates a whole DOM.

## Contributing

Any suggestions and patches are welcome. Especially from crate authors.

## License

This repo is licensed under MIT license.
