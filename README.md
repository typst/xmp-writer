# xmp-writer
[![Crates.io](https://img.shields.io/crates/v/xmp-writer.svg)](https://crates.io/crates/xmp-writer)
[![Documentation](https://docs.rs/xmp-writer/badge.svg)](https://docs.rs/xmp-writer)

Write XMP metadata, step by step.

```toml
[dependencies]
xmp-writer = "0.2"
```

XMP is a metadata format developed by Adobe. It is either embedded into
files (e.g. PDF, JPEG, TIFF) or stored in a separate "side-car" file.

This crate provides a simple API to write XMP metadata. Start by creating
a new `XmpWriter`, then add entries to it. Finally, call `XmpWriter::finish` to
get the XMP metadata as a byte vector. Some properties contain a complex data type like a
struct or an array. In this case, the writer returns a new struct that can be used to
write the data. The reference to the struct must be dropped before the writer can be used
again.

## Example

```rust
use xmp_writer::{LangId, DateTime, XmpWriter};

let mut writer = XmpWriter::new();
writer.creator(["Martin Haug"]);
writer.title([(Some(LangId("de")), "Titel"), (None, "Title")]);
writer.num_pages(3);
writer.pdf_keywords("Keyword1, Keyword2");
writer.description([(None, "Description")]);
writer.date([DateTime::date(2021, 11, 06)]);

let mut colors = writer.colorants();
colors.add_colorant().swatch_name("Red");
colors.add_colorant().swatch_name("Green");
drop(colors);

writer.creator_tool("xmp-writer 0.2.0");

println!("{}", std::str::from_utf8(&writer.finish(None)).unwrap());
```

## See also
- [XMP Specification, Part 1: Basics](https://github.com/adobe/XMP-Toolkit-SDK/blob/main/docs/XMPSpecificationPart1.pdf)
- [XMP Specification, Part 2: Additional Properties](https://github.com/adobe/XMP-Toolkit-SDK/blob/main/docs/XMPSpecificationPart2.pdf)
- [XMP Specification, Part 3: File Embedding and Interchange](https://github.com/adobe/XMP-Toolkit-SDK/blob/main/docs/XMPSpecificationPart3.pdf)

## Safety
This crate forbids unsafe code and has no dependencies.

## License
This crate is dual-licensed under the MIT and Apache 2.0 licenses.
