use xmp_writer::XmpWriter;

fn main() {
    let mut writer = XmpWriter::new();
    writer.creator(["Martin Haug"]);
    println!("{}", std::str::from_utf8(&writer.finalize(None)).unwrap());
}
