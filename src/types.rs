use std::{
    fmt::Debug,
    io::{Error, Write},
    iter,
};

use crate::XmpWriter;

/// XML Namespaces for the XMP properties.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Namespace {
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

impl Namespace {
    /// Returns the URL for the namespace.
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

    /// Returns the prefix for the namespace.
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

/// A XMP property.
///
/// Created by [`XmpWriter::element`], [`Array::element`],
/// [`Array::element_with_attrs`], [`Struct::element`],
/// [`Struct::element_with_attrs`].
pub struct Element<'a> {
    writer: &'a mut XmpWriter,
    name: &'a str,
    namespace: Namespace,
}

impl<'a> Element<'a> {
    pub(crate) fn start(
        writer: &'a mut XmpWriter,
        name: &'a str,
        namespace: Namespace,
    ) -> Self {
        Self::with_attrs(writer, name, namespace, iter::empty())
    }

    fn with_attrs<'b>(
        writer: &'a mut XmpWriter,
        name: &'a str,
        namespace: Namespace,
        attrs: impl IntoIterator<Item = (&'b str, &'b str)>,
    ) -> Self {
        write!(writer.buf, "<{}:{}", namespace.prefix(), name).unwrap();

        for (key, value) in attrs {
            write!(writer.buf, " {}=\"{}\"", key, value).unwrap();
        }

        writer.namespaces.insert(namespace.clone());
        Element { writer, name, namespace }
    }

    /// Sets the property to a primitive value.
    pub fn value(self, val: impl XmpType) {
        write!(self.writer.buf, ">").unwrap();
        val.write(&mut self.writer.buf).unwrap();
        self.close();
    }

    /// Start writing a struct as the property value.
    pub fn obj(self) -> Struct<'a> {
        self.writer.namespaces.insert(Namespace::Rdf);
        write!(self.writer.buf, " rdf:parseType=\"Resource\">").unwrap();
        Struct::start(self.writer, self.name, self.namespace)
    }

    /// Start writing an array as the property value.
    pub fn array(self, kind: RdfCollectionType) -> Array<'a> {
        write!(self.writer.buf, ">").unwrap();
        Array::start(self.writer, kind, self.name, self.namespace)
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

    /// Set a language alternative of primitive values as the property value.
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

    /// Start writing an unordered array (`rdf:Bag`) as the property value.
    pub fn unordered_array<'b>(self, items: impl IntoIterator<Item = impl XmpType>) {
        let mut array = self.array(RdfCollectionType::Bag);
        for item in items {
            array.element().value(item);
        }
    }

    /// Start writing an ordered array (`rdf:Seq`) as the property value.
    pub fn ordered_array<'b>(self, items: impl IntoIterator<Item = impl XmpType>) {
        let mut array = self.array(RdfCollectionType::Seq);
        for item in items {
            array.element().value(item);
        }
    }

    /// Start writing an alternative array (`rdf:Alt`) as the property value.
    pub fn alternative_array<'b>(self, items: impl IntoIterator<Item = impl XmpType>) {
        let mut array = self.array(RdfCollectionType::Alt);
        for item in items {
            array.element().value(item);
        }
    }
}

/// An XMP array value.
///
/// Created by [`Element::array`].
pub struct Array<'a> {
    writer: &'a mut XmpWriter,
    kind: RdfCollectionType,
    name: &'a str,
    namespace: Namespace,
}

impl<'a> Array<'a> {
    fn start(
        writer: &'a mut XmpWriter,
        kind: RdfCollectionType,
        name: &'a str,
        namespace: Namespace,
    ) -> Self {
        writer.namespaces.insert(Namespace::Rdf);
        write!(writer.buf, "<rdf:{}>", kind.rdf_type()).unwrap();
        Self { writer, kind, name, namespace }
    }

    /// Start writing an element in the array.
    pub fn element(&mut self) -> Element<'_> {
        self.element_with_attrs(iter::empty())
    }

    /// Start writing an element with attributes in the array.
    pub fn element_with_attrs(
        &mut self,
        attrs: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Element<'_> {
        Element::with_attrs(self.writer, "li", Namespace::Rdf, attrs)
    }
}

impl Drop for Array<'_> {
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

/// An XMP struct value.
///
/// Created by [`Element::obj`].
pub struct Struct<'a> {
    writer: &'a mut XmpWriter,
    name: &'a str,
    namespace: Namespace,
}

impl<'a> Struct<'a> {
    fn start(writer: &'a mut XmpWriter, name: &'a str, namespace: Namespace) -> Self {
        Self { writer, name, namespace }
    }

    /// Start writing a property in the struct.
    pub fn element(&mut self, name: &'a str, namespace: Namespace) -> Element<'_> {
        self.element_with_attrs(name, namespace, iter::empty())
    }

    /// Start writing a property with attributes in the struct.
    pub fn element_with_attrs<'b>(
        &mut self,
        name: &'a str,
        namespace: Namespace,
        attrs: impl IntoIterator<Item = (&'b str, &'b str)>,
    ) -> Element<'_> {
        Element::with_attrs(self.writer, name, namespace, attrs)
    }
}

impl Drop for Struct<'_> {
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

/// Primitive XMP types.
pub trait XmpType {
    /// Write the value to the buffer.
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

/// Types of RDF collections.
pub enum RdfCollectionType {
    /// An ordered array / sequence.
    Seq,
    /// An unordered array / bag.
    Bag,
    /// An alternative array.
    Alt,
}

impl RdfCollectionType {
    /// The RDF type name for this collection type.
    pub fn rdf_type(&self) -> &'static str {
        match self {
            RdfCollectionType::Seq => "Seq",
            RdfCollectionType::Bag => "Bag",
            RdfCollectionType::Alt => "Alt",
        }
    }
}

/// A language specifier as defined in RFC 3066. Can also be `x-default` if the
/// language is not known.
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

/// A date and time.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DateTime {
    year: u16,
    month: Option<u8>,
    day: Option<u8>,
    hour: Option<u8>,
    minute: Option<u8>,
    second: Option<u8>,
    tz_hour: Option<i8>,
    tz_minute: Option<i8>,
}

impl DateTime {
    /// Create a new date and time with all fields.
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

    /// Create a new date and time without a timezone.
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

    /// Create a new date and time without a timezone.
    pub fn local_time(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Self {
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
}

impl XmpType for DateTime {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            DateTime {
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
            DateTime {
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
            DateTime {
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
            DateTime {
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
            DateTime {
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
            DateTime {
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

/// The intended use of the resource.
#[derive(Debug, Clone, PartialEq)]
pub enum RenditionClass {
    /// The master resource.
    Default,
    /// A review copy.
    Draft,
    /// A low-resolution stand-in.
    LowResolution,
    /// A proof copy.
    Proof,
    /// A copy at screen resolution.
    Screen,
    /// A thumbnail.
    Thumbnail {
        /// The format of the thumbnail.
        format: Option<String>,
        /// The size of the thumbnail.
        size: Option<(u32, u32)>,
        /// The color space of the thumbnail.
        color_space: Option<String>,
    },
    /// A custom rendition class.
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
            Self::Thumbnail { format, size, color_space } => {
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

/// A user-assigned rating.
pub enum Rating {
    /// The resource has been rejected.
    Rejected,
    /// The resource has not been rated.
    Unknown,
    /// The resource has been rated 1 star.
    OneStar,
    /// The resource has been rated 2 stars.
    TwoStars,
    /// The resource has been rated 3 stars.
    ThreeStars,
    /// The resource has been rated 4 stars.
    FourStars,
    /// The resource has been rated 5 stars.
    FiveStars,
}

impl Rating {
    /// Creates a new `Rating` from the number of stars.
    pub fn from_stars(stars: Option<u32>) -> Self {
        match stars {
            Some(0) => Self::Unknown,
            Some(1) => Self::OneStar,
            Some(2) => Self::TwoStars,
            Some(3) => Self::ThreeStars,
            Some(4) => Self::FourStars,
            Some(5) => Self::FiveStars,
            Some(stars) => panic!(
                "Invalid number of stars: {} (must be between 0 and 5)",
                stars
            ),
            None => Self::Unknown,
        }
    }

    /// Convert the rating to an XMP primitive.
    pub fn to_xmp(self) -> f32 {
        match self {
            Self::Rejected => -1.0,
            Self::Unknown => 0.0,
            Self::OneStar => 1.0,
            Self::TwoStars => 2.0,
            Self::ThreeStars => 3.0,
            Self::FourStars => 4.0,
            Self::FiveStars => 5.0,
        }
    }
}

/// Whether to ignore the markers of an [ingredient.](crate::ResourceRefWriter)
pub enum MaskMarkers {
    /// Ignore all markers and those of the children.
    All,
    /// Process all markers.
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

/// The type of a resource event.
#[allow(missing_docs)]
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

/// The color space in which a colorant is defined.
#[allow(missing_docs)]
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

/// The type of a colorant.
pub enum ColorantType {
    /// Colors inherent to the printing process.
    Process,
    /// Special colors.
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

/// The unit of a physical dimension.
#[allow(missing_docs)]
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

/// The font file type.
#[allow(missing_docs)]
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
