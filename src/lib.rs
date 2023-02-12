pub mod types;

use std::collections::BTreeSet;
use std::io::Write;

use types::{
    ArrayWriter, LangId, MaskMarkers, RenditionClass, ResourceEventAction, XmpDate, XmpElement,
    XmpNamespace, XmpStruct,
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

    #[inline]
    pub fn element<'a>(&'a mut self, name: &'a str, namespace: XmpNamespace) -> XmpElement<'a> {
        XmpElement::start(self, name, namespace)
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
                namespace.prefix(),
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
        self.element("contributor", XmpNamespace::DublinCore)
            .unordered_array(contributor);
        self
    }

    pub fn coverage(&mut self, coverage: &str) -> &mut Self {
        self.element("coverage", XmpNamespace::DublinCore)
            .value(coverage);
        self
    }

    pub fn creator<'a>(&mut self, creator: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.element("creator", XmpNamespace::DublinCore)
            .ordered_array(creator);
        self
    }

    pub fn date(&mut self, date: XmpDate) -> &mut Self {
        self.element("date", XmpNamespace::DublinCore).value(date);
        self
    }

    pub fn description<'a>(
        &mut self,
        description: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> &mut Self {
        self.element("description", XmpNamespace::DublinCore)
            .language_alternative(description);
        self
    }

    pub fn format(&mut self, mime: &str) -> &mut Self {
        self.element("format", XmpNamespace::DublinCore).value(mime);
        self
    }

    pub fn identifier(&mut self, id: &str) -> &mut Self {
        self.element("identifier", XmpNamespace::DublinCore)
            .value(id);
        self
    }

    pub fn language<'a>(&mut self, lang: impl IntoIterator<Item = LangId<'a>>) -> &mut Self {
        self.element("language", XmpNamespace::DublinCore)
            .unordered_array(lang);
        self
    }

    pub fn publisher<'a>(&mut self, publisher: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.element("publisher", XmpNamespace::DublinCore)
            .unordered_array(publisher);
        self
    }

    pub fn relation(&mut self, relation: &str) -> &mut Self {
        self.element("relation", XmpNamespace::DublinCore)
            .value(relation);
        self
    }

    pub fn rights<'a>(
        &mut self,
        rights: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> &mut Self {
        self.element("rights", XmpNamespace::DublinCore)
            .language_alternative(rights);
        self
    }

    pub fn source(&mut self, source: &str) -> &mut Self {
        self.element("source", XmpNamespace::DublinCore)
            .value(source);
        self
    }

    pub fn subject<'a>(&mut self, subject: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.element("subject", XmpNamespace::DublinCore)
            .unordered_array(subject);
        self
    }

    pub fn title<'a>(
        &mut self,
        title: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> &mut Self {
        self.element("title", XmpNamespace::DublinCore)
            .language_alternative(title);
        self
    }

    pub fn type_<'a>(&mut self, kind: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.element("type", XmpNamespace::DublinCore)
            .unordered_array(kind);
        self
    }
}

/// XMP Basic Schema
impl XmpWriter {
    pub fn base_url(&mut self, url: &str) -> &mut Self {
        self.element("BaseURL", XmpNamespace::Xmp).value(url);
        self
    }

    pub fn create_date(&mut self, date: XmpDate) -> &mut Self {
        self.element("CreateDate", XmpNamespace::Xmp).value(date);
        self
    }

    pub fn creator_tool(&mut self, tool: &str) -> &mut Self {
        self.element("CreatorTool", XmpNamespace::Xmp).value(tool);
        self
    }

    pub fn xmp_identifier<'a>(&mut self, id: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.element("Identifier", XmpNamespace::Xmp)
            .unordered_array(id);
        self
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.element("Label", XmpNamespace::Xmp).value(label);
        self
    }

    pub fn metadata_date(&mut self, date: XmpDate) -> &mut Self {
        self.element("MetadataDate", XmpNamespace::Xmp).value(date);
        self
    }

    pub fn modify_date(&mut self, date: XmpDate) -> &mut Self {
        self.element("ModifyDate", XmpNamespace::Xmp).value(date);
        self
    }

    pub fn nickname(&mut self, nickname: &str) -> &mut Self {
        self.element("Nickname", XmpNamespace::Xmp).value(nickname);
        self
    }

    pub fn rating(&mut self, rating: i64) -> &mut Self {
        self.element("Rating", XmpNamespace::Xmp).value(rating);
        self
    }

    pub fn thumbnail(&mut self) -> ThumbnailWriter<'_> {
        ThumbnailWriter::start(self.element("Thumbnail", XmpNamespace::Xmp).obj())
    }
}

pub struct ThumbnailWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a> ThumbnailWriter<'a> {
    pub fn start(stc: XmpStruct<'a>) -> Self {
        Self { stc }
    }

    pub fn format<'b>(&mut self, format: &'b str) -> &mut Self {
        self.stc
            .element("format", XmpNamespace::XmpImage)
            .value(format);
        self
    }

    pub fn format_jpeg(&mut self) -> &mut Self {
        self.format("JPEG")
    }

    pub fn width(&mut self, width: u64) -> &mut Self {
        self.stc
            .element("width", XmpNamespace::XmpImage)
            .value(width as i64);
        self
    }

    pub fn height(&mut self, height: u64) -> &mut Self {
        self.stc
            .element("height", XmpNamespace::XmpImage)
            .value(height as i64);
        self
    }

    pub fn image<'b>(&mut self, image: &'b str) -> &mut Self {
        self.stc
            .element("image", XmpNamespace::XmpImage)
            .value(image);
        self
    }
}

/// XMP Rights Management Schema
impl XmpWriter {
    pub fn certificate(&mut self, cert: &str) -> &mut Self {
        self.element("Certificate", XmpNamespace::XmpRights)
            .value(cert);
        self
    }

    pub fn marked(&mut self, marked: bool) -> &mut Self {
        self.element("Marked", XmpNamespace::XmpRights)
            .value(marked);
        self
    }

    pub fn owner<'a>(&mut self, owner: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.element("Owner", XmpNamespace::XmpRights)
            .unordered_array(owner);
        self
    }

    pub fn usage_terms<'a>(
        &mut self,
        terms: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> &mut Self {
        self.element("UsageTerms", XmpNamespace::XmpRights)
            .language_alternative(terms);
        self
    }

    pub fn web_statement(&mut self, statement: &str) -> &mut Self {
        self.element("WebStatement", XmpNamespace::XmpRights)
            .value(statement);
        self
    }
}

/// XMP Media Management Schema
impl XmpWriter {
    pub fn derived_from(&mut self) -> ResourceRefWriter<'_> {
        ResourceRefWriter::start(self.element("DerivedFrom", XmpNamespace::XmpMedia).obj())
    }

    pub fn document_id(&mut self, id: &str) -> &mut Self {
        self.element("DocumentID", XmpNamespace::XmpMedia).value(id);
        self
    }

    pub fn history<'a>(&mut self) -> ResourceEventsWriter<'_> {
        ResourceEventsWriter::start(
            self.element("History", XmpNamespace::XmpMedia)
                .array(types::RdfCollectionType::Seq),
        )
    }

    // TODO: Ingredients

    pub fn instance_id(&mut self, id: &str) -> &mut Self {
        self.element("InstanceID", XmpNamespace::XmpMedia).value(id);
        self
    }

    pub fn managed_from(&mut self) -> ResourceRefWriter<'_> {
        ResourceRefWriter::start(self.element("ManagedFrom", XmpNamespace::XmpMedia).obj())
    }

    pub fn manager<'a>(&mut self, manager: &'a str) -> &mut Self {
        self.element("Manager", XmpNamespace::XmpMedia)
            .value(manager);
        self
    }

    pub fn manage_to<'a>(&mut self, uri: &'a str) -> &mut Self {
        self.element("ManageTo", XmpNamespace::XmpMedia).value(uri);
        self
    }

    pub fn manage_ui<'a>(&mut self, uri: &'a str) -> &mut Self {
        self.element("ManageUI", XmpNamespace::XmpMedia).value(uri);
        self
    }

    pub fn manager_variant(&mut self, variant: &str) -> &mut Self {
        self.element("ManagerVariant", XmpNamespace::XmpMedia)
            .value(variant);
        self
    }

    pub fn original_doc_id(&mut self, id: &str) -> &mut Self {
        self.element("OriginalDocumentID", XmpNamespace::XmpMedia)
            .value(id);
        self
    }

    // TODO: Pantry

    pub fn rendition_class(&mut self, class: RenditionClass) -> &mut Self {
        self.element("RenditionClass", XmpNamespace::XmpMedia)
            .value(class);
        self
    }

    pub fn rendition_params(&mut self, params: &str) -> &mut Self {
        self.element("RenditionParams", XmpNamespace::XmpMedia)
            .value(params);
        self
    }

    pub fn version_id(&mut self, id: &str) -> &mut Self {
        self.element("VersionID", XmpNamespace::XmpMedia).value(id);
        self
    }

    // TODO: Versions
}

pub struct ResourceRefWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a> ResourceRefWriter<'a> {
    pub fn start(stc: XmpStruct<'a>) -> Self {
        Self { stc }
    }

    pub fn alternate_paths<'b>(&mut self, paths: impl IntoIterator<Item = &'b str>) -> &mut Self {
        self.stc
            .element("alternatePaths", XmpNamespace::XmpResourceRef)
            .ordered_array(paths);
        self
    }

    pub fn document_id<'b>(&mut self, id: &'b str) -> &mut Self {
        self.stc
            .element("documentID", XmpNamespace::XmpResourceRef)
            .value(id);
        self
    }

    pub fn file_path<'b>(&mut self, path: &'b str) -> &mut Self {
        self.stc
            .element("filePath", XmpNamespace::XmpResourceRef)
            .value(path);
        self
    }

    pub fn instance_id<'b>(&mut self, id: &'b str) -> &mut Self {
        self.stc
            .element("instanceID", XmpNamespace::XmpResourceRef)
            .value(id);
        self
    }

    pub fn last_modify_date(&mut self, date: XmpDate) -> &mut Self {
        self.stc
            .element("lastModifyDate", XmpNamespace::XmpResourceRef)
            .value(date);
        self
    }

    pub fn manager<'b>(&mut self, manager: &'b str) -> &mut Self {
        self.stc
            .element("manager", XmpNamespace::XmpResourceRef)
            .value(manager);
        self
    }

    pub fn manager_variant<'b>(&mut self, variant: &'b str) -> &mut Self {
        self.stc
            .element("managerVariant", XmpNamespace::XmpResourceRef)
            .value(variant);
        self
    }

    pub fn manage_to<'b>(&mut self, uri: &'b str) -> &mut Self {
        self.stc
            .element("manageTo", XmpNamespace::XmpResourceRef)
            .value(uri);
        self
    }

    pub fn manage_ui<'b>(&mut self, uri: &'b str) -> &mut Self {
        self.stc
            .element("manageTo", XmpNamespace::XmpResourceRef)
            .value(uri);
        self
    }

    pub fn mask_markers(&mut self, markers: MaskMarkers) -> &mut Self {
        self.stc
            .element("maskMarkers", XmpNamespace::XmpResourceRef)
            .value(markers);
        self
    }

    pub fn part_mapping<'b>(&mut self, mapping: &'b str) -> &mut Self {
        self.stc
            .element("partMapping", XmpNamespace::XmpResourceRef)
            .value(mapping);
        self
    }

    pub fn rendition_class(&mut self, rendition: RenditionClass) -> &mut Self {
        self.stc
            .element("renditionClass", XmpNamespace::XmpResourceRef)
            .value(rendition);
        self
    }

    pub fn rendition_params<'b>(&mut self, params: &'b str) -> &mut Self {
        self.stc
            .element("renditionParams", XmpNamespace::XmpResourceRef)
            .value(params);
        self
    }

    pub fn to_part<'b>(&mut self, part: &'b str) -> &mut Self {
        self.stc
            .element("toPart", XmpNamespace::XmpResourceRef)
            .value(part);
        self
    }

    pub fn version_id<'b>(&mut self, id: &'b str) -> &mut Self {
        self.stc
            .element("versionID", XmpNamespace::XmpResourceRef)
            .value(id);
        self
    }
}

pub struct ResourceEventWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a> ResourceEventWriter<'a> {
    pub fn start(stc: XmpStruct<'a>) -> Self {
        Self { stc }
    }

    pub fn action(&mut self, action: ResourceEventAction) -> &mut Self {
        self.stc
            .element("action", XmpNamespace::XmpResourceEvent)
            .value(action);
        self
    }

    pub fn instance_id<'b>(&mut self, id: &'b str) -> &mut Self {
        self.stc
            .element("instanceID", XmpNamespace::XmpResourceEvent)
            .value(id);
        self
    }

    pub fn parameters<'b>(&mut self, params: &'b str) -> &mut Self {
        self.stc
            .element("parameters", XmpNamespace::XmpResourceEvent)
            .value(params);
        self
    }

    pub fn software_agent<'b>(&mut self, agent: &'b str) -> &mut Self {
        self.stc
            .element("softwareAgent", XmpNamespace::XmpResourceEvent)
            .value(agent);
        self
    }

    pub fn when(&mut self, date: XmpDate) -> &mut Self {
        self.stc
            .element("when", XmpNamespace::XmpResourceEvent)
            .value(date);
        self
    }
}

pub struct ResourceEventsWriter<'a> {
    array: ArrayWriter<'a>,
}

impl<'a> ResourceEventsWriter<'a> {
    pub fn start(array: ArrayWriter<'a>) -> Self {
        Self { array }
    }

    pub fn add_event(&mut self) -> ResourceEventWriter<'_> {
        ResourceEventWriter::start(self.array.element().obj())
    }
}
