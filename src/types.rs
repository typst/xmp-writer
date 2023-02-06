use std::{
    fmt::Debug,
    io::{Error, Write},
};

use crate::XmpWriter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmpNamespace {
    Rdf,
    DublinCore,
    Xmp,
    XmpRights,
    XmpResourceRef,
    XmpMedia,
    XmpJob,
    XmpPaged,
    XmpDynamicMedia,
    XmpImage,
    AdobePdf,
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
            Self::XmpMedia => "http://ns.adobe.com/xap/1.0/mm/",
            Self::XmpJob => "http://ns.adobe.com/xap/1.0/bj/",
            Self::XmpPaged => "http://ns.adobe.com/xap/1.0/t/pg/",
            Self::XmpDynamicMedia => "http://ns.adobe.com/xap/1.0/DynamicMedia/",
            Self::XmpImage => "http://ns.adobe.com/xap/1.0/g/img/",
            Self::AdobePdf => "http://ns.adobe.com/pdf/1.3/",
            Self::Custom((_, url)) => url,
        }
    }

    pub fn namespace(&self) -> &str {
        match self {
            Self::Rdf => "rdf",
            Self::DublinCore => "dc",
            Self::Xmp => "xmp",
            Self::XmpRights => "xmpRights",
            Self::XmpResourceRef => "stRef",
            Self::XmpMedia => "xmpMM",
            Self::XmpJob => "xmpBJ",
            Self::XmpPaged => "xmpTPg",
            Self::XmpDynamicMedia => "xmpDM",
            Self::XmpImage => "xmpGImg",
            Self::AdobePdf => "pdf",
            Self::Custom((namespace, _)) => namespace,
        }
    }
}

pub struct XmpElement<'a> {
    namespace: XmpNamespace,
    name: &'a str,
    value: XmpValue<'a>,
    attrs: Vec<(&'a str, &'a str)>,
}

impl<'a> XmpElement<'a> {
    pub fn new(namespace: XmpNamespace, name: &'a str, value: XmpValue<'a>) -> XmpElement<'a> {
        XmpElement {
            namespace,
            name,
            value,
            attrs: Vec::new(),
        }
    }

    pub fn with_attrs(
        namespace: XmpNamespace,
        name: &'a str,
        value: XmpValue<'a>,
        attrs: Vec<(&'a str, &'a str)>,
    ) -> XmpElement<'a> {
        XmpElement {
            namespace,
            name,
            value,
            attrs,
        }
    }

    pub(crate) fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        write!(buf, "<{}:{}", self.namespace.namespace(), self.name)?;

        for (key, value) in self.attrs.iter() {
            write!(buf, " {}=\"{}\"", key, value)?;
        }

        if let XmpValue::Struct(_) = self.value {
            write!(buf, " rdf:parseType=\"Resource\">")?;
        } else {
            write!(buf, ">")?;
        }

        self.value.write(buf)?;

        write!(buf, "</{}:{}>", self.namespace.namespace(), self.name)?;

        Ok(())
    }

    pub(crate) fn namespaces(&self, f: &mut impl FnMut(XmpNamespace)) {
        f(self.namespace.clone());
        self.value.namespaces(f);
    }
}

pub enum XmpValue<'a> {
    Boolean(bool),
    Date(XmpDate),
    String(&'a str),
    Integer(i64),
    Real(f64),
    Array(RdfCollection<'a>),
    Struct(XmpStruct<'a>),
    DynValue(Box<dyn XmpType>),
}

pub struct XmpStruct<'a> {
    writer: &'a mut XmpWriter,
    namespace: XmpNamespace,
    name: &'a str,
}

impl<'a> XmpStruct<'a> {
    pub fn new(
        writer: &'a mut XmpWriter,
        name: &'a str,
        namespace: XmpNamespace,
    ) -> Result<XmpStruct<'a>, Error> {
        write!(
            writer.buf,
            "<{}:{} rdf:parseType=\"Resource\">",
            namespace.namespace(),
            name,
        )?;
        writer.add_namespace(namespace.clone());

        Ok(Self {
            writer,
            namespace,
            name,
        })
    }

    pub fn add_element(&mut self, element: XmpElement<'a>) {
        self.writer.add_element(element);
    }
}

impl Drop for XmpStruct<'_> {
    fn drop(&mut self) {
        write!(
            self.writer.buf,
            "</{}:{}>",
            self.namespace.namespace(),
            self.name
        )
        .unwrap();
    }
}

pub trait XmpType {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error>;
}

impl<'a> XmpValue<'a> {
    pub fn from_str(value: &'a str) -> XmpValue<'a> {
        XmpValue::String(value)
    }

    pub fn language_alternative(
        iter: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> XmpValue<'a> {
        XmpValue::Array(RdfCollection::Alt(
            iter.into_iter()
                .map(|(lang, value)| {
                    XmpElement::with_attrs(
                        XmpNamespace::Rdf,
                        "li",
                        XmpValue::String(value),
                        vec![("xml:lang", lang.map(|l| l.0).unwrap_or_else(|| "x-default"))],
                    )
                })
                .collect(),
        ))
    }

    pub fn unordered_array<'b>(iter: impl IntoIterator<Item = XmpValue<'a>>) -> Self {
        XmpValue::Array(RdfCollection::Bag(
            iter.into_iter()
                .map(|value| XmpElement::new(XmpNamespace::Rdf, "li", value))
                .collect(),
        ))
    }

    pub fn ordered_array<'b>(iter: impl IntoIterator<Item = XmpValue<'a>>) -> Self {
        XmpValue::Array(RdfCollection::Seq(
            iter.into_iter()
                .map(|value| XmpElement::new(XmpNamespace::Rdf, "li", value))
                .collect(),
        ))
    }

    pub fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            XmpValue::Boolean(b) => write!(buf, "{}", if *b { "True" } else { "False" }),
            XmpValue::Date(d) => d.write(buf),
            XmpValue::String(s) => write!(buf, "{}", {
                let mut res = String::new();
                for c in s.chars() {
                    match c {
                        '<' => res.push_str("&lt;"),
                        '>' => res.push_str("&gt;"),
                        '&' => res.push_str("&amp;"),
                        '\'' => res.push_str("&apos;"),
                        '"' => res.push_str("&quot;"),
                        _ => res.push(c),
                    }
                }
                res
            }),
            XmpValue::Integer(i) => write!(buf, "{}", i),
            XmpValue::Real(r) => write!(buf, "{}", r),
            XmpValue::Array(a) => a.write(buf),
            XmpValue::Struct(_) => Ok(()),
            XmpValue::DynValue(d) => d.write(buf),
        }
    }

    pub(crate) fn namespaces(&self, f: &mut impl FnMut(XmpNamespace)) {
        match self {
            XmpValue::Boolean(_) => (),
            XmpValue::Date(_) => (),
            XmpValue::String(_) => (),
            XmpValue::Integer(_) => (),
            XmpValue::Real(_) => (),
            XmpValue::Array(a) => a.namespaces(f),
            XmpValue::Struct(_) => (),
            XmpValue::DynValue(_) => (),
        }
    }
}

pub enum RdfCollection<'a> {
    Seq(Vec<XmpElement<'a>>),
    Bag(Vec<XmpElement<'a>>),
    Alt(Vec<XmpElement<'a>>),
}

impl<'a> RdfCollection<'a> {
    fn rdf_type(&self) -> &'static str {
        match self {
            RdfCollection::Seq(_) => "Seq",
            RdfCollection::Bag(_) => "Bag",
            RdfCollection::Alt(_) => "Alt",
        }
    }

    fn iter(&self) -> impl Iterator<Item = &XmpElement> {
        match self {
            RdfCollection::Seq(v) => v.iter(),
            RdfCollection::Bag(v) => v.iter(),
            RdfCollection::Alt(v) => v.iter(),
        }
    }

    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        let kind = self.rdf_type();
        write!(buf, "<rdf:{}>", kind)?;
        for elem in self.iter() {
            elem.write(buf)?;
        }
        write!(buf, "</rdf:{}>", kind)?;
        Ok(())
    }

    fn namespaces(&self, f: &mut impl FnMut(XmpNamespace)) {
        for elem in self.iter() {
            elem.namespaces(f);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LangId<'a>(&'a str);

impl<'a> From<LangId<'a>> for XmpValue<'a> {
    fn from(value: LangId<'a>) -> Self {
        XmpValue::String(value.0)
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
    pub(crate) fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
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
}

impl From<XmpRating> for XmpValue<'static> {
    fn from(rating: XmpRating) -> Self {
        match rating {
            XmpRating::Rejected => XmpValue::Real(-1.0),
            XmpRating::Unknown => XmpValue::Real(0.0),
            XmpRating::OneStar => XmpValue::Real(1.0),
            XmpRating::TwoStars => XmpValue::Real(2.0),
            XmpRating::ThreeStars => XmpValue::Real(3.0),
            XmpRating::FourStars => XmpValue::Real(4.0),
            XmpRating::FiveStars => XmpValue::Real(5.0),
        }
    }
}
