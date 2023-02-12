use std::{
    fmt::Debug,
    io::{Error, Write},
    iter,
};

use crate::XmpWriter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmpNamespace {
    Rdf,
    DublinCore,
    Xmp,
    XmpRights,
    XmpResourceRef,
    XmpResourceEvent,
    XmpVersion,
    XmpJob,
    XmpJobManagement,
    XmpColorant,
    XmpFont,
    XmpDimensions,
    XmpMedia,
    XmpPaged,
    XmpDynamicMedia,
    XmpImage,
    XmpIdq,
    AdobePdf,
    PdfAId,
    PdfXId,
    Custom((String, String)),
}

impl XmpNamespace {
    pub fn url(&self) -> &str {
        match self {
            Self::Rdf => "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
            Self::DublinCore => "http://purl.org/dc/elements/1.1/",
            Self::Xmp => "http://ns.adobe.com/xap/1.0/",
            Self::XmpRights => "http://ns.adobe.com/xap/1.0/rights/",
            Self::XmpResourceRef => "http://ns.adobe.com/xap/1.0/sType/ResourceRef#",
            Self::XmpResourceEvent => "http://ns.adobe.com/xap/1.0/sType/ResourceEvent#",
            Self::XmpVersion => "http://ns.adobe.com/xap/1.0/sType/Version#",
            Self::XmpJob => "http://ns.adobe.com/xap/1.0/sType/Job#",
            Self::XmpColorant => "http://ns.adobe.com/xap/1.0/g/",
            Self::XmpFont => "http://ns.adobe.com/xap/1.0/sType/Font#",
            Self::XmpDimensions => "http://ns.adobe.com/xap/1.0/sType/Dimensions#",
            Self::XmpMedia => "http://ns.adobe.com/xap/1.0/mm/",
            Self::XmpJobManagement => "http://ns.adobe.com/xap/1.0/bj/",
            Self::XmpPaged => "http://ns.adobe.com/xap/1.0/t/pg/",
            Self::XmpDynamicMedia => "http://ns.adobe.com/xap/1.0/DynamicMedia/",
            Self::XmpImage => "http://ns.adobe.com/xap/1.0/g/img/",
            Self::AdobePdf => "http://ns.adobe.com/pdf/1.3/",
            Self::XmpIdq => "http://ns.adobe.com/xmp/Identifier/qual/1.0/",
            Self::PdfAId => "http://www.aiim.org/pdfa/ns/id/",
            Self::PdfXId => "http://www.npes.org/pdfx/ns/id/",
            Self::Custom((_, url)) => url,
        }
    }

    pub fn prefix(&self) -> &str {
        match self {
            Self::Rdf => "rdf",
            Self::DublinCore => "dc",
            Self::Xmp => "xmp",
            Self::XmpRights => "xmpRights",
            Self::XmpResourceRef => "stRef",
            Self::XmpResourceEvent => "stEvt",
            Self::XmpVersion => "stVer",
            Self::XmpJob => "stJob",
            Self::XmpColorant => "xmpG",
            Self::XmpFont => "stFnt",
            Self::XmpDimensions => "stDim",
            Self::XmpMedia => "xmpMM",
            Self::XmpJobManagement => "xmpBJ",
            Self::XmpPaged => "xmpTPg",
            Self::XmpDynamicMedia => "xmpDM",
            Self::XmpImage => "xmpGImg",
            Self::AdobePdf => "pdf",
            Self::XmpIdq => "xmpidq",
            Self::PdfAId => "pdfaid",
            Self::PdfXId => "pdfxid",
            Self::Custom((namespace, _)) => namespace,
        }
    }
}

pub struct XmpElement<'a> {
    writer: &'a mut XmpWriter,
    name: &'a str,
    namespace: XmpNamespace,
}

impl<'a> XmpElement<'a> {
    pub fn start(writer: &'a mut XmpWriter, name: &'a str, namespace: XmpNamespace) -> Self {
        Self::with_attrs(writer, name, namespace, iter::empty())
    }

    pub fn with_attrs<'b>(
        writer: &'a mut XmpWriter,
        name: &'a str,
        namespace: XmpNamespace,
        attrs: impl IntoIterator<Item = (&'b str, &'b str)>,
    ) -> Self {
        write!(writer.buf, "<{}:{}", namespace.prefix(), name).unwrap();

        for (key, value) in attrs {
            write!(writer.buf, " {}=\"{}\"", key, value).unwrap();
        }

        writer.namespaces.insert(namespace.clone());
        XmpElement {
            writer,
            name,
            namespace,
        }
    }

    pub fn value(self, val: impl XmpType) {
        write!(self.writer.buf, ">").unwrap();
        val.write(&mut self.writer.buf).unwrap();
        self.close();
    }

    pub fn obj(self) -> XmpStruct<'a> {
        write!(self.writer.buf, " rdf:parseType=\"Resource\">").unwrap();
        XmpStruct::start(self.writer, self.name, self.namespace)
    }

    pub fn array(self, kind: RdfCollectionType) -> ArrayWriter<'a> {
        write!(self.writer.buf, ">").unwrap();
        ArrayWriter::start(self.writer, kind, self.name, self.namespace)
    }

    fn close(self) {
        write!(
            self.writer.buf,
            "</{}:{}>",
            self.namespace.prefix(),
            self.name
        )
        .unwrap();
    }

    pub fn language_alternative<'b>(
        self,
        items: impl IntoIterator<Item = (Option<LangId<'b>>, &'b str)>,
    ) {
        let mut array = self.array(RdfCollectionType::Alt);
        for (lang, value) in items {
            array
                .element_with_attrs(iter::once(("xml:lang", lang.unwrap_or_default().0)))
                .value(value);
        }
        drop(array);
    }

    pub fn unordered_array<'b>(self, items: impl IntoIterator<Item = impl XmpType>) {
        let mut array = self.array(RdfCollectionType::Bag);
        for item in items {
            array.element().value(item);
        }
    }

    pub fn ordered_array<'b>(self, items: impl IntoIterator<Item = impl XmpType>) {
        let mut array = self.array(RdfCollectionType::Seq);
        for item in items {
            array.element().value(item);
        }
    }

    pub fn alternative_array<'b>(self, items: impl IntoIterator<Item = impl XmpType>) {
        let mut array = self.array(RdfCollectionType::Alt);
        for item in items {
            array.element().value(item);
        }
    }
}

pub struct ArrayWriter<'a> {
    writer: &'a mut XmpWriter,
    kind: RdfCollectionType,
    name: &'a str,
    namespace: XmpNamespace,
}

impl<'a> ArrayWriter<'a> {
    pub fn start(
        writer: &'a mut XmpWriter,
        kind: RdfCollectionType,
        name: &'a str,
        namespace: XmpNamespace,
    ) -> Self {
        writer.namespaces.insert(XmpNamespace::Rdf);
        write!(writer.buf, "<rdf:{}>", kind.rdf_type()).unwrap();
        Self {
            writer,
            kind,
            name,
            namespace,
        }
    }

    pub fn element(&mut self) -> XmpElement<'_> {
        self.element_with_attrs(iter::empty())
    }

    pub fn element_with_attrs(
        &mut self,
        attrs: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> XmpElement<'_> {
        XmpElement::with_attrs(self.writer, "li", XmpNamespace::Rdf, attrs)
    }
}

impl Drop for ArrayWriter<'_> {
    fn drop(&mut self) {
        write!(
            self.writer.buf,
            "</rdf:{}></{}:{}>",
            self.kind.rdf_type(),
            self.namespace.prefix(),
            self.name
        )
        .unwrap();
    }
}

pub struct XmpStruct<'a> {
    writer: &'a mut XmpWriter,
    name: &'a str,
    namespace: XmpNamespace,
}

impl<'a> XmpStruct<'a> {
    pub fn start(writer: &'a mut XmpWriter, name: &'a str, namespace: XmpNamespace) -> Self {
        Self {
            writer,
            name,
            namespace,
        }
    }

    pub fn element(&mut self, name: &'a str, namespace: XmpNamespace) -> XmpElement<'_> {
        self.element_with_attrs(name, namespace, iter::empty())
    }

    pub fn element_with_attrs<'b>(
        &mut self,
        name: &'a str,
        namespace: XmpNamespace,
        attrs: impl IntoIterator<Item = (&'b str, &'b str)>,
    ) -> XmpElement<'_> {
        XmpElement::with_attrs(self.writer, name, namespace, attrs)
    }
}

impl Drop for XmpStruct<'_> {
    fn drop(&mut self) {
        write!(
            self.writer.buf,
            "</{}:{}>",
            self.namespace.prefix(),
            self.name
        )
        .unwrap();
    }
}

pub trait XmpType {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error>;
}

impl XmpType for bool {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        if *self {
            buf.extend_from_slice(b"True");
        } else {
            buf.extend_from_slice(b"False");
        }
        Ok(())
    }
}

impl XmpType for i32 {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        write!(buf, "{}", self)
    }
}

impl XmpType for i64 {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        write!(buf, "{}", self)
    }
}

impl XmpType for f32 {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        write!(buf, "{}", self)
    }
}

impl XmpType for f64 {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        write!(buf, "{}", self)
    }
}

impl XmpType for &str {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        let mut res = String::new();
        for c in self.chars() {
            match c {
                '<' => res.push_str("&lt;"),
                '>' => res.push_str("&gt;"),
                '&' => res.push_str("&amp;"),
                '\'' => res.push_str("&apos;"),
                '"' => res.push_str("&quot;"),
                _ => res.push(c),
            }
        }
        write!(buf, "{}", res)
    }
}

pub enum RdfCollectionType {
    Seq,
    Bag,
    Alt,
}

impl RdfCollectionType {
    fn rdf_type(&self) -> &'static str {
        match self {
            RdfCollectionType::Seq => "Seq",
            RdfCollectionType::Bag => "Bag",
            RdfCollectionType::Alt => "Alt",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LangId<'a>(pub &'a str);

impl XmpType for LangId<'_> {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        write!(buf, "{}", self.0)
    }
}

impl Default for LangId<'_> {
    fn default() -> Self {
        Self("x-default")
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct XmpDate {
    year: u16,
    month: Option<u8>,
    day: Option<u8>,
    hour: Option<u8>,
    minute: Option<u8>,
    second: Option<u8>,
    tz_hour: Option<i8>,
    tz_minute: Option<i8>,
}

impl XmpDate {
    pub fn date(year: u16, month: u8, day: u8) -> Self {
        Self {
            year,
            month: Some(month),
            day: Some(day),
            hour: None,
            minute: None,
            second: None,
            tz_hour: None,
            tz_minute: None,
        }
    }

    pub fn local_time(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            year,
            month: Some(month),
            day: Some(day),
            hour: Some(hour),
            minute: Some(minute),
            second: Some(second),
            tz_hour: None,
            tz_minute: None,
        }
    }

    pub fn new(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        tz_hour: i8,
        tz_minute: i8,
    ) -> Self {
        Self {
            year,
            month: Some(month),
            day: Some(day),
            hour: Some(hour),
            minute: Some(minute),
            second: Some(second),
            tz_hour: Some(tz_hour),
            tz_minute: Some(tz_minute),
        }
    }
}

impl XmpType for XmpDate {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            XmpDate {
                year,
                month: None,
                day: None,
                hour: None,
                minute: None,
                second: None,
                tz_hour: None,
                tz_minute: None,
            } => {
                write!(buf, "{:04}", year)
            }
            XmpDate {
                year,
                month: Some(month),
                day: None,
                hour: None,
                minute: None,
                second: None,
                tz_hour: None,
                tz_minute: None,
            } => {
                write!(buf, "{:04}-{:02}", year, month)
            }
            XmpDate {
                year,
                month: Some(month),
                day: Some(day),
                hour: None,
                minute: None,
                second: None,
                tz_hour: None,
                tz_minute: None,
            } => {
                write!(buf, "{:04}-{:02}-{:02}", year, month, day)
            }
            XmpDate {
                year,
                month: Some(month),
                day: Some(day),
                hour: Some(hour),
                minute,
                second: None,
                tz_hour: None,
                tz_minute: None,
            } => {
                write!(
                    buf,
                    "{:04}-{:02}-{:02}T{:02}:{:02}",
                    year,
                    month,
                    day,
                    hour,
                    minute.unwrap_or_default()
                )
            }
            XmpDate {
                year,
                month: Some(month),
                day: Some(day),
                hour: Some(hour),
                minute: Some(minute),
                second: Some(second),
                tz_hour: None,
                tz_minute: None,
            } => {
                write!(
                    buf,
                    "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
                    year, month, day, hour, minute, second
                )
            }
            XmpDate {
                year,
                month: Some(month),
                day: Some(day),
                hour: Some(hour),
                minute: Some(minute),
                second: Some(second),
                tz_hour: Some(tz_hour),
                tz_minute,
            } => {
                write!(
                    buf,
                    "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{:03}:{:02}",
                    year,
                    month,
                    day,
                    hour,
                    minute,
                    second,
                    tz_hour,
                    tz_minute.unwrap_or_default()
                )
            }
            _ => {
                panic!("Invalid XMP date");
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RenditionClass {
    Default,
    Draft,
    LowResolution,
    Proof,
    Screen,
    Thumbnail {
        format: Option<String>,
        size: Option<(u32, u32)>,
        color_space: Option<String>,
    },
    Custom(String),
}

impl XmpType for RenditionClass {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            Self::Default => write!(buf, "default"),
            Self::Draft => write!(buf, "draft"),
            Self::LowResolution => write!(buf, "low-res"),
            Self::Proof => write!(buf, "proof"),
            Self::Screen => write!(buf, "screen"),
            Self::Thumbnail {
                format,
                size,
                color_space,
            } => {
                write!(buf, "thumbnail")?;
                if let Some(format) = format {
                    write!(buf, ":{}", format)?;
                }
                if let Some((width, height)) = size {
                    write!(buf, ":{}x{}", width, height)?;
                }
                if let Some(color_space) = color_space {
                    write!(buf, ":{}", color_space)?;
                }
                Ok(())
            }
            Self::Custom(s) => write!(buf, "{}", s),
        }
    }
}

pub enum XmpRating {
    Rejected,
    Unknown,
    OneStar,
    TwoStars,
    ThreeStars,
    FourStars,
    FiveStars,
}

impl XmpRating {
    pub fn from_stars(stars: Option<u32>) -> Self {
        match stars {
            Some(0) => XmpRating::Unknown,
            Some(1) => XmpRating::OneStar,
            Some(2) => XmpRating::TwoStars,
            Some(3) => XmpRating::ThreeStars,
            Some(4) => XmpRating::FourStars,
            Some(5) => XmpRating::FiveStars,
            Some(stars) => panic!(
                "Invalid number of stars: {} (must be between 0 and 5)",
                stars
            ),
            None => XmpRating::Unknown,
        }
    }

    pub fn to_xmp(self) -> f32 {
        match self {
            XmpRating::Rejected => -1.0,
            XmpRating::Unknown => 0.0,
            XmpRating::OneStar => 1.0,
            XmpRating::TwoStars => 2.0,
            XmpRating::ThreeStars => 3.0,
            XmpRating::FourStars => 4.0,
            XmpRating::FiveStars => 5.0,
        }
    }
}

pub enum MaskMarkers {
    All,
    None,
}

impl XmpType for MaskMarkers {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            Self::All => write!(buf, "All"),
            Self::None => write!(buf, "None"),
        }
    }
}

pub enum ResourceEventAction {
    Converted,
    Copied,
    Created,
    Cropped,
    Edited,
    Filtered,
    Formatted,
    VersionUpdated,
    Printed,
    Published,
    Managed,
    Produced,
    Resized,
    Saved,
    Custom(String),
}

impl XmpType for ResourceEventAction {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            Self::Converted => write!(buf, "converted"),
            Self::Copied => write!(buf, "copied"),
            Self::Created => write!(buf, "created"),
            Self::Cropped => write!(buf, "cropped"),
            Self::Edited => write!(buf, "edited"),
            Self::Filtered => write!(buf, "filtered"),
            Self::Formatted => write!(buf, "formatted"),
            Self::VersionUpdated => write!(buf, "version_updated"),
            Self::Printed => write!(buf, "printed"),
            Self::Published => write!(buf, "published"),
            Self::Managed => write!(buf, "managed"),
            Self::Produced => write!(buf, "produced"),
            Self::Resized => write!(buf, "resized"),
            Self::Saved => write!(buf, "saved"),
            Self::Custom(s) => write!(buf, "{}", s),
        }
    }
}

pub enum ColorantMode {
    CMYK,
    RGB,
    Lab,
}

impl XmpType for ColorantMode {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        buf.extend_from_slice(match self {
            Self::CMYK => b"CMYK",
            Self::RGB => b"RGB",
            Self::Lab => b"Lab",
        });
        Ok(())
    }
}

pub enum ColorantType {
    Process,
    Spot,
}

impl XmpType for ColorantType {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        buf.extend_from_slice(match self {
            Self::Process => b"PROCESS",
            Self::Spot => b"SPOT",
        });
        Ok(())
    }
}

pub enum DimensionUnit<'a> {
    Inch,
    Mm,
    Pixel,
    Pica,
    Point,
    Custom(&'a str),
}

impl<'a> XmpType for DimensionUnit<'a> {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            Self::Inch => write!(buf, "inch"),
            Self::Mm => write!(buf, "mm"),
            Self::Pixel => write!(buf, "pixel"),
            Self::Pica => write!(buf, "pica"),
            Self::Point => write!(buf, "point"),
            Self::Custom(s) => write!(buf, "{}", s),
        }
    }
}

pub enum FontType<'a> {
    TrueType,
    OpenType,
    Type1,
    Bitmap,
    Custom(&'a str),
}

impl<'a> XmpType for FontType<'a> {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            Self::TrueType => write!(buf, "TrueType"),
            Self::OpenType => write!(buf, "OpenType"),
            Self::Type1 => write!(buf, "Type1"),
            Self::Bitmap => write!(buf, "Bitmap"),
            Self::Custom(s) => write!(buf, "{}", s),
        }
    }
}
