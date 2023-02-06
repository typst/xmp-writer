pub mod types;

use std::collections::BTreeSet;
use std::io::Write;

use types::{LangId, RenditionClass, XmpDate, XmpElement, XmpNamespace, XmpStruct, XmpValue};

pub struct XmpWriter {
    pub(crate) buf: Vec<u8>,
    namespaces: BTreeSet<XmpNamespace>,
}

impl XmpWriter {
    pub fn new() -> XmpWriter {
        Self {
            buf: vec![],
            namespaces: BTreeSet::new(),
        }
    }

    pub(crate) fn add_namespace(&mut self, ns: XmpNamespace) {
        self.namespaces.insert(ns);
    }

    #[inline]
    pub fn add_element(&mut self, element: XmpElement) -> &mut Self {
        element.write(&mut self.buf).unwrap();
        element.namespaces(&mut |ns| {
            self.namespaces.insert(ns);
        });
        self
    }

    pub fn finalize(self, about: Option<&str>) -> Vec<u8> {
        let mut buf = vec![];
        write!(
            &mut buf,
            "<?xpacket begin=\"\u{feff}\" id=\"W5M0MpCehiHzreSzNTczkc9d\"?>"
        )
        .unwrap();
        write!(
            &mut buf,
            "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"xmp-writer\"><rdf:RDF xmlns:rdf=\"{}\"><rdf:Description rdf:about=\"{}\"",
            XmpNamespace::Rdf.url(),
            about.unwrap_or(""),
        )
        .unwrap();

        for namespace in self
            .namespaces
            .into_iter()
            .filter(|ns| &XmpNamespace::Rdf != ns)
        {
            write!(
                &mut buf,
                " xmlns:{}=\"{}\" ",
                namespace.namespace(),
                namespace.url()
            )
            .unwrap();
        }

        buf.extend_from_slice(b">");
        buf.extend_from_slice(&self.buf);
        write!(
            &mut buf,
            "</rdf:Description></rdf:RDF></x:xmpmeta><?xpacket end=\"r\"?>"
        )
        .unwrap();
        buf
    }
}

/// XMP Dublin Core Schema
impl XmpWriter {
    pub fn contributor<'a>(&mut self, contributor: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "contributor",
            XmpValue::unordered_array(contributor.into_iter().map(XmpValue::from_str)),
        ))
    }

    pub fn coverage(&mut self, coverage: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "coverage",
            XmpValue::String(coverage.into()),
        ))
    }

    pub fn creator<'a>(&mut self, creator: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "creator",
            XmpValue::ordered_array(creator.into_iter().map(XmpValue::from_str)),
        ))
    }

    pub fn date(&mut self, date: XmpDate) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "date",
            XmpValue::Date(date),
        ))
    }

    pub fn description<'a>(
        &mut self,
        description: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "description",
            XmpValue::language_alternative(description),
        ))
    }

    pub fn format(&mut self, mime: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "format",
            XmpValue::String(mime.into()),
        ))
    }

    pub fn identifier(&mut self, id: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "identifier",
            XmpValue::String(id.into()),
        ))
    }

    pub fn language<'a>(&mut self, lang: impl IntoIterator<Item = LangId<'a>>) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "language",
            XmpValue::unordered_array(lang.into_iter().map(XmpValue::from)),
        ))
    }

    pub fn publisher<'a>(&mut self, publisher: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "publisher",
            XmpValue::unordered_array(publisher.into_iter().map(XmpValue::from_str)),
        ))
    }

    pub fn relation(&mut self, relation: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "relation",
            XmpValue::String(relation.into()),
        ))
    }

    pub fn rights<'a>(
        &mut self,
        rights: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "rights",
            XmpValue::language_alternative(rights),
        ))
    }

    pub fn source(&mut self, source: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "source",
            XmpValue::String(source.into()),
        ))
    }

    pub fn subject<'a>(&mut self, subject: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "subject",
            XmpValue::unordered_array(subject.into_iter().map(XmpValue::from_str)),
        ))
    }

    pub fn title<'a>(
        &mut self,
        title: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "title",
            XmpValue::language_alternative(title),
        ))
    }

    pub fn type_<'a>(&mut self, kind: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "type",
            XmpValue::unordered_array(kind.into_iter().map(XmpValue::from_str)),
        ))
    }
}

/// XMP Basic Schema
impl XmpWriter {
    pub fn base_url(&mut self, url: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::Xmp,
            "BaseURL",
            XmpValue::String(url),
        ))
    }

    pub fn create_date(&mut self, date: XmpDate) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::Xmp,
            "CreateDate",
            XmpValue::Date(date),
        ))
    }

    pub fn creator_tool(&mut self, tool: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::Xmp,
            "CreatorTool",
            XmpValue::String(tool),
        ))
    }

    pub fn xmp_identifier<'a>(&mut self, id: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::Xmp,
            "Identifier",
            XmpValue::unordered_array(id.into_iter().map(XmpValue::from_str)),
        ))
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::Xmp,
            "Label",
            XmpValue::String(label),
        ))
    }

    pub fn metadata_date(&mut self, date: XmpDate) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::Xmp,
            "MetadataDate",
            XmpValue::Date(date),
        ))
    }

    pub fn modify_date(&mut self, date: XmpDate) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::Xmp,
            "ModifyDate",
            XmpValue::Date(date),
        ))
    }

    pub fn nickname(&mut self, nickname: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::Xmp,
            "Nickname",
            XmpValue::String(nickname),
        ))
    }

    pub fn rating(&mut self, rating: i64) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::Xmp,
            "Rating",
            XmpValue::Integer(rating),
        ))
    }

    pub fn thumbnail(&mut self) -> ThumbnailWriter<'_> {
        ThumbnailWriter::start(self)
    }
}

pub struct ThumbnailWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a, 'b: 'a> ThumbnailWriter<'a> {
    pub fn start(writer: &'a mut XmpWriter) -> Self {
        Self {
            stc: XmpStruct::new(writer, "Thumbnail", XmpNamespace::XmpImage).unwrap(),
        }
    }

    pub fn format(&mut self, format: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpImage,
            "format",
            XmpValue::String(format.into()),
        ));
        self
    }

    pub fn format_jpeg(&mut self) -> &mut Self {
        self.format("JPEG")
    }

    pub fn width(&mut self, width: u64) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpImage,
            "width",
            XmpValue::Integer(width as i64),
        ));
        self
    }

    pub fn height(&mut self, height: u64) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpImage,
            "height",
            XmpValue::Integer(height as i64),
        ));
        self
    }

    pub fn image(&mut self, image: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpImage,
            "image",
            XmpValue::String(image.into()),
        ));
        self
    }
}

/// XMP Rights Management Schema
impl XmpWriter {}

pub struct ResourceRefWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a, 'b: 'a> ResourceRefWriter<'a> {
    pub fn start(writer: &'a mut XmpWriter, name: &'a str) -> Self {
        Self {
            stc: XmpStruct::new(writer, name, XmpNamespace::XmpResourceRef).unwrap(),
        }
    }

    pub fn alternate_paths(&mut self, paths: impl IntoIterator<Item = &'b str>) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "alternatePaths",
            XmpValue::ordered_array(paths.into_iter().map(XmpValue::from_str)),
        ));
        self
    }

    pub fn document_id(&mut self, id: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "documentID",
            XmpValue::String(id.into()),
        ));
        self
    }

    pub fn file_path(&mut self, path: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "filePath",
            XmpValue::String(path.into()),
        ));
        self
    }

    pub fn instance_id(&mut self, id: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "instanceID",
            XmpValue::String(id.into()),
        ));
        self
    }

    pub fn rendition_class(&mut self, rendition: RenditionClass) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "renditionClass",
            XmpValue::DynValue(Box::new(rendition)),
        ));
        self
    }

    pub fn rendition_params(&mut self, params: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "renditionParams",
            XmpValue::String(params.into()),
        ));
        self
    }
}
