use xmp_writer::{
    types::{LangId, XmpDate},
    XmpWriter,
};

fn main() {
    let mut writer = XmpWriter::new();
    writer.creator(["Martin Haug"]);
    writer.title([(Some(LangId("de")), "Titel"), (None, "Title")]);
    writer.num_pages(3);
    writer.pdf_keywords("Keyword1, Keyword2");
    writer.description([(None, "Description")]);
    writer.creator_tool("xmp-writer 0.1.0");
    writer.date([XmpDate::date(2021, 11, 06)]);
    let mut colors = writer.colorants();
    colors.add_colorant().swatch_name("Red");
    colors.add_colorant().swatch_name("Green");
    drop(colors);
    println!("{}", std::str::from_utf8(&writer.finalize(None)).unwrap());
}
