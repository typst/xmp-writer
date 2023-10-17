use std::{
    fmt::{Debug, Write},
    iter,
};

use crate::XmpWriter;

/// XML Namespaces for the XMP properties.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Namespace<'a> {
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
    Custom((&'a str, &'a str)),
}

impl Namespace<'_> {
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
pub struct Element<'a, 'n: 'a> {
    writer: &'a mut XmpWriter<'n>,
    name: &'a str,
    namespace: Namespace<'n>,
}

impl<'a, 'n: 'a> Element<'a, 'n> {
    pub(crate) fn start(
        writer: &'a mut XmpWriter<'n>,
        name: &'a str,
        namespace: Namespace<'n>,
    ) -> Self {
        Self::with_attrs(writer, name, namespace, iter::empty())
    }

    fn with_attrs<'b>(
        writer: &'a mut XmpWriter<'n>,
        name: &'a str,
        namespace: Namespace<'n>,
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
        self.writer.buf.push('>');
        val.write(&mut self.writer.buf);
        self.close();
    }

    /// Start writing a struct as the property value.
    pub fn obj(self) -> Struct<'a, 'n> {
        self.writer.namespaces.insert(Namespace::Rdf);
        self.writer.buf.push_str(" rdf:parseType=\"Resource\">");
        Struct::start(self.writer, self.name, self.namespace)
    }

    /// Start writing an array as the property value.
    pub fn array(self, kind: RdfCollectionType) -> Array<'a, 'n> {
        self.writer.buf.push('>');
        Array::start(self.writer, kind, self.name, self.namespace)
    }

    fn close(self) {
        write!(self.writer.buf, "</{}:{}>", self.namespace.prefix(), self.name).unwrap();
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
pub struct Array<'a, 'n: 'a> {
    writer: &'a mut XmpWriter<'n>,
    kind: RdfCollectionType,
    name: &'a str,
    namespace: Namespace<'a>,
}

impl<'a, 'n: 'a> Array<'a, 'n> {
    fn start(
        writer: &'a mut XmpWriter<'n>,
        kind: RdfCollectionType,
        name: &'a str,
        namespace: Namespace<'n>,
    ) -> Self {
        writer.namespaces.insert(Namespace::Rdf);
        write!(writer.buf, "<rdf:{}>", kind.rdf_type()).unwrap();
        Self { writer, kind, name, namespace }
    }

    /// Start writing an element in the array.
    pub fn element(&mut self) -> Element<'_, 'n> {
        self.element_with_attrs(iter::empty())
    }

    /// Start writing an element with attributes in the array.
    pub fn element_with_attrs(
        &mut self,
        attrs: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Element<'_, 'n> {
        Element::with_attrs(self.writer, "li", Namespace::Rdf, attrs)
    }
}

impl Drop for Array<'_, '_> {
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
pub struct Struct<'a, 'n: 'a> {
    writer: &'a mut XmpWriter<'n>,
    name: &'a str,
    namespace: Namespace<'a>,
}

impl<'a, 'n: 'a> Struct<'a, 'n> {
    fn start(
        writer: &'a mut XmpWriter<'n>,
        name: &'a str,
        namespace: Namespace<'n>,
    ) -> Self {
        Self { writer, name, namespace }
    }

    /// Start writing a property in the struct.
    pub fn element(
        &mut self,
        name: &'a str,
        namespace: Namespace<'n>,
    ) -> Element<'_, 'n> {
        self.element_with_attrs(name, namespace, iter::empty())
    }

    /// Start writing a property with attributes in the struct.
    pub fn element_with_attrs<'b>(
        &mut self,
        name: &'a str,
        namespace: Namespace<'n>,
        attrs: impl IntoIterator<Item = (&'b str, &'b str)>,
    ) -> Element<'_, 'n> {
        Element::with_attrs(self.writer, name, namespace, attrs)
    }
}

impl Drop for Struct<'_, '_> {
    fn drop(&mut self) {
        write!(self.writer.buf, "</{}:{}>", self.namespace.prefix(), self.name).unwrap();
    }
}

/// Primitive XMP types.
pub trait XmpType {
    /// Write the value to the buffer.
    fn write(&self, buf: &mut String);
}

impl XmpType for bool {
    fn write(&self, buf: &mut String) {
        if *self {
            buf.push_str("True");
        } else {
            buf.push_str("False");
        }
    }
}

impl XmpType for i32 {
    fn write(&self, buf: &mut String) {
        write!(buf, "{}", self).unwrap();
    }
}

impl XmpType for i64 {
    fn write(&self, buf: &mut String) {
        write!(buf, "{}", self).unwrap();
    }
}

impl XmpType for f32 {
    fn write(&self, buf: &mut String) {
        write!(buf, "{}", self).unwrap();
    }
}

impl XmpType for f64 {
    fn write(&self, buf: &mut String) {
        write!(buf, "{}", self).unwrap();
    }
}

impl XmpType for &str {
    fn write(&self, buf: &mut String) {
        for c in self.chars() {
            match c {
                '<' => buf.push_str("&lt;"),
                '>' => buf.push_str("&gt;"),
                '&' => buf.push_str("&amp;"),
                '\'' => buf.push_str("&apos;"),
                '"' => buf.push_str("&quot;"),
                _ => buf.push(c),
            }
        }
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
    fn write(&self, buf: &mut String) {
        buf.push_str(self.0);
    }
}

impl Default for LangId<'_> {
    fn default() -> Self {
        Self("x-default")
    }
}

/// A date and time.
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(missing_docs)]
pub struct DateTime {
    pub year: u16,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    /// The timezone of this date and time. No assumptions about the timezone or
    /// locale should be made if this is `None`.
    pub timezone: Option<Timezone>,
}

/// A timezone.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Timezone {
    /// UTC time. Use `Local` for British time.
    Utc,
    /// A local timezone offset.
    Local {
        /// Timezone offset in hours.
        hour: i8,
        /// Timezone offset in minutes.
        minute: i8,
    },
}

impl DateTime {
    /// Create a new date and time with all fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        timezone: Timezone,
    ) -> Self {
        Self {
            year,
            month: Some(month),
            day: Some(day),
            hour: Some(hour),
            minute: Some(minute),
            second: Some(second),
            timezone: Some(timezone),
        }
    }

    /// Create a new date with a year only.
    pub fn year(year: u16) -> Self {
        Self {
            year,
            month: None,
            day: None,
            hour: None,
            minute: None,
            second: None,
            timezone: None,
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
            timezone: None,
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
            timezone: None,
        }
    }
}

impl XmpType for DateTime {
    fn write(&self, buf: &mut String) {
        (|| {
            write!(buf, "{:04}", self.year).unwrap();
            write!(buf, "-{:02}", self.month?).unwrap();
            write!(buf, "-{:02}", self.day?).unwrap();
            write!(buf, "T{:02}:{:02}", self.hour?, self.minute?).unwrap();
            write!(buf, ":{:02}", self.second?).unwrap();
            match self.timezone? {
                Timezone::Utc => buf.push('Z'),
                Timezone::Local { hour, minute } => {
                    write!(buf, "{:+03}:{:02}", hour, minute).unwrap();
                }
            }
            Some(())
        })();
    }
}

/// The intended use of the resource.
#[derive(Debug, Clone, PartialEq)]
pub enum RenditionClass<'a> {
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
        format: Option<&'a str>,
        /// The size of the thumbnail.
        size: Option<(u32, u32)>,
        /// The color space of the thumbnail.
        color_space: Option<&'a str>,
    },
    /// A custom rendition class.
    Custom(&'a str),
}

impl XmpType for RenditionClass<'_> {
    fn write(&self, buf: &mut String) {
        match self {
            Self::Default => buf.push_str("default"),
            Self::Draft => buf.push_str("draft"),
            Self::LowResolution => buf.push_str("low-res"),
            Self::Proof => buf.push_str("proof"),
            Self::Screen => buf.push_str("screen"),
            Self::Thumbnail { format, size, color_space } => {
                buf.push_str("thumbnail");
                if let Some(format) = format {
                    buf.push(':');
                    buf.push_str(format);
                }
                if let Some((width, height)) = size {
                    buf.push(':');
                    buf.push_str(&width.to_string());
                    buf.push('x');
                    buf.push_str(&height.to_string());
                }
                if let Some(color_space) = color_space {
                    buf.push(':');
                    buf.push_str(color_space);
                }
            }
            Self::Custom(s) => buf.push_str(s),
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
            Some(stars) => {
                panic!("Invalid number of stars: {} (must be between 0 and 5)", stars)
            }
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
    fn write(&self, buf: &mut String) {
        match self {
            Self::All => buf.push_str("All"),
            Self::None => buf.push_str("None"),
        }
    }
}

/// The type of a resource event.
#[allow(missing_docs)]
pub enum ResourceEventAction<'a> {
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
    Custom(&'a str),
}

impl XmpType for ResourceEventAction<'_> {
    fn write(&self, buf: &mut String) {
        match self {
            Self::Converted => buf.push_str("converted"),
            Self::Copied => buf.push_str("copied"),
            Self::Created => buf.push_str("created"),
            Self::Cropped => buf.push_str("cropped"),
            Self::Edited => buf.push_str("edited"),
            Self::Filtered => buf.push_str("filtered"),
            Self::Formatted => buf.push_str("formatted"),
            Self::VersionUpdated => buf.push_str("version_updated"),
            Self::Printed => buf.push_str("printed"),
            Self::Published => buf.push_str("published"),
            Self::Managed => buf.push_str("managed"),
            Self::Produced => buf.push_str("produced"),
            Self::Resized => buf.push_str("resized"),
            Self::Saved => buf.push_str("saved"),
            Self::Custom(s) => buf.push_str(s),
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
    fn write(&self, buf: &mut String) {
        buf.push_str(match self {
            Self::CMYK => "CMYK",
            Self::RGB => "RGB",
            Self::Lab => "Lab",
        });
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
    fn write(&self, buf: &mut String) {
        buf.push_str(match self {
            Self::Process => "PROCESS",
            Self::Spot => "SPOT",
        });
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
    fn write(&self, buf: &mut String) {
        match self {
            Self::Inch => buf.push_str("inch"),
            Self::Mm => buf.push_str("mm"),
            Self::Pixel => buf.push_str("pixel"),
            Self::Pica => buf.push_str("pica"),
            Self::Point => buf.push_str("point"),
            Self::Custom(s) => buf.push_str(s),
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
    fn write(&self, buf: &mut String) {
        match self {
            Self::TrueType => buf.push_str("TrueType"),
            Self::OpenType => buf.push_str("OpenType"),
            Self::Type1 => buf.push_str("Type1"),
            Self::Bitmap => buf.push_str("Bitmap"),
            Self::Custom(s) => buf.push_str(s),
        }
    }
}
