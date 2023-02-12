pub mod types;

use std::collections::BTreeSet;
use std::io::Write;

use quick_xml::name;
use types::{
    LangId, MaskMarkers, RenditionClass, ResourceEventAction, XmpDate, XmpElement, XmpNamespace,
    XmpStruct, XmpValue,
};

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
            XmpValue::String(coverage),
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
            XmpValue::String(mime),
        ))
    }

    pub fn identifier(&mut self, id: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::DublinCore,
            "identifier",
            XmpValue::String(id),
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
            XmpValue::String(relation),
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
            XmpValue::String(source),
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
        ThumbnailWriter::start(self, XmpNamespace::Xmp)
    }
}

pub struct ThumbnailWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a, 'b: 'a> ThumbnailWriter<'a> {
    pub fn start(writer: &'a mut XmpWriter, namespace: XmpNamespace) -> Self {
        writer.namespaces.insert(namespace.clone());
        Self {
            stc: XmpStruct::new(writer, "Thumbnail", namespace).unwrap(),
        }
    }

    pub fn format(&mut self, format: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpImage,
            "format",
            XmpValue::String(format),
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
            XmpValue::String(image),
        ));
        self
    }
}

/// XMP Rights Management Schema
impl XmpWriter {
    pub fn certificate(&mut self, cert: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpRights,
            "Certificate",
            XmpValue::String(cert),
        ))
    }

    pub fn marked(&mut self, marked: bool) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpRights,
            "Marked",
            XmpValue::Boolean(marked),
        ))
    }

    pub fn owner<'a>(&mut self, owner: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpRights,
            "Owner",
            XmpValue::unordered_array(owner.into_iter().map(XmpValue::from_str)),
        ))
    }

    pub fn usage_terms<'a>(
        &mut self,
        terms: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpRights,
            "UsageTerms",
            XmpValue::language_alternative(terms),
        ))
    }

    pub fn web_statement(&mut self, statement: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpRights,
            "WebStatement",
            XmpValue::String(statement),
        ))
    }
}

/// XMP Media Management Schema
impl XmpWriter {
    pub fn derived_from(&mut self) -> ResourceRefWriter<'_> {
        ResourceRefWriter::start(self, "DerivedFrom", XmpNamespace::XmpMedia)
    }

    pub fn document_id(&mut self, id: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpMedia,
            "DocumentID",
            XmpValue::String(id),
        ))
    }

    pub fn history<'a>(&mut self) -> ResourceEventsWriter<'_> {
        ResourceEventsWriter::start(self, "History", XmpNamespace::XmpMedia)
    }

    // TODO: Ingredients

    pub fn instance_id(&mut self, id: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpMedia,
            "InstanceID",
            XmpValue::String(id),
        ))
    }

    pub fn managed_from(&mut self) -> ResourceRefWriter<'_> {
        ResourceRefWriter::start(self, "ManagedFrom", XmpNamespace::XmpMedia)
    }

    pub fn manager<'a>(&mut self, manager: &'a str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpMedia,
            "Manager",
            XmpValue::String(manager),
        ))
    }

    pub fn manage_to<'a>(&mut self, uri: &'a str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpMedia,
            "ManageTo",
            XmpValue::String(uri),
        ))
    }

    pub fn manage_ui<'a>(&mut self, uri: &'a str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpMedia,
            "ManageUI",
            XmpValue::String(uri),
        ))
    }

    pub fn manager_variant(&mut self, variant: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpMedia,
            "ManagerVariant",
            XmpValue::String(variant),
        ))
    }

    pub fn original_doc_id(&mut self, id: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpMedia,
            "OriginalDocumentID",
            XmpValue::String(id),
        ))
    }

    // TODO: Pantry

    pub fn rendition_class(&mut self, class: RenditionClass) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpMedia,
            "RenditionClass",
            XmpValue::DynValue(Box::new(class)),
        ))
    }

    pub fn rendition_params(&mut self, params: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpMedia,
            "RenditionParams",
            XmpValue::String(params),
        ))
    }

    pub fn version_id(&mut self, id: &str) -> &mut Self {
        self.add_element(XmpElement::new(
            XmpNamespace::XmpMedia,
            "VersionID",
            XmpValue::String(id),
        ))
    }

    // TODO: Versions
}

pub struct ResourceRefWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a, 'b: 'a> ResourceRefWriter<'a> {
    pub fn start(writer: &'a mut XmpWriter, name: &'a str, namespace: XmpNamespace) -> Self {
        writer.namespaces.insert(namespace.clone());
        Self {
            stc: XmpStruct::new(writer, name, namespace).unwrap(),
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
            XmpValue::String(id),
        ));
        self
    }

    pub fn file_path(&mut self, path: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "filePath",
            XmpValue::String(path),
        ));
        self
    }

    pub fn instance_id(&mut self, id: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "instanceID",
            XmpValue::String(id),
        ));
        self
    }

    pub fn last_modify_date(&mut self, date: XmpDate) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "lastModifyDate",
            XmpValue::Date(date),
        ));
        self
    }

    pub fn manager(&mut self, manager: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "manager",
            XmpValue::String(manager),
        ));
        self
    }

    pub fn manager_variant(&mut self, variant: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "managerVariant",
            XmpValue::String(variant),
        ));
        self
    }

    pub fn manage_to(&mut self, uri: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "manageTo",
            XmpValue::String(uri),
        ));
        self
    }

    pub fn manage_ui(&mut self, uri: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "manageTo",
            XmpValue::String(uri),
        ));
        self
    }

    pub fn mask_markers(&mut self, markers: MaskMarkers) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "maskMarkers",
            XmpValue::DynValue(Box::new(markers)),
        ));
        self
    }

    pub fn part_mapping(&mut self, mapping: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "partMapping",
            XmpValue::String(mapping),
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
            XmpValue::String(params),
        ));
        self
    }

    pub fn to_part(&mut self, part: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "toPart",
            XmpValue::String(part),
        ));
        self
    }

    pub fn version_id(&mut self, id: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceRef,
            "versionID",
            XmpValue::String(id),
        ));
        self
    }
}

pub struct ResourceEventWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a, 'b: 'a> ResourceEventWriter<'a> {
    pub fn start(writer: &'a mut XmpWriter, name: &'a str, namespace: XmpNamespace) -> Self {
        writer.namespaces.insert(namespace.clone());
        Self {
            stc: XmpStruct::new(writer, name, namespace).unwrap(),
        }
    }

    pub fn action(&mut self, action: ResourceEventAction) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceEvent,
            "action",
            XmpValue::DynValue(Box::new(action)),
        ));
        self
    }

    pub fn instance_id(&mut self, id: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceEvent,
            "instanceID",
            XmpValue::String(id),
        ));
        self
    }

    pub fn parameters(&mut self, params: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceEvent,
            "parameters",
            XmpValue::String(params),
        ));
        self
    }

    pub fn software_agent(&mut self, agent: &'b str) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceEvent,
            "softwareAgent",
            XmpValue::String(agent),
        ));
        self
    }

    pub fn when(&mut self, date: XmpDate) -> &mut Self {
        self.stc.add_element(XmpElement::new(
            XmpNamespace::XmpResourceEvent,
            "when",
            XmpValue::Date(date),
        ));
        self
    }
}

pub struct ResourceEventsWriter<'a> {
    writer: &'a mut XmpWriter,
    name: &'a str,
    namespace: XmpNamespace,
}

impl<'a, 'b: 'a> ResourceEventsWriter<'a> {
    pub fn start(writer: &'a mut XmpWriter, name: &'a str, namespace: XmpNamespace) -> Self {
        write!(
            &mut writer.buf,
            "<{}:{}><rdf:Seq>",
            namespace.namespace(),
            name
        )
        .unwrap();
        writer.namespaces.insert(namespace.clone());
        writer.namespaces.insert(XmpNamespace::Rdf);
        Self {
            writer,
            name,
            namespace,
        }
    }

    pub fn add_event(&mut self) -> ResourceEventWriter<'_> {
        ResourceEventWriter::start(self.writer, "li", XmpNamespace::Rdf)
    }
}

impl Drop for ResourceEventsWriter<'_> {
    fn drop(&mut self) {
        write!(
            &mut self.writer.buf,
            "</rdf:Seq></{}:{}>",
            self.namespace.namespace(),
            self.name
        )
        .unwrap();
    }
}
