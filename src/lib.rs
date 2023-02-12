pub mod types;

use std::collections::BTreeSet;
use std::io::Write;

use types::{
    ArrayWriter, ColorantMode, ColorantType, DimensionUnit, FontType, LangId, MaskMarkers,
    RdfCollectionType, RenditionClass, ResourceEventAction, XmpDate, XmpElement, XmpNamespace,
    XmpStruct,
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

    pub fn date(&mut self, date: impl IntoIterator<Item = XmpDate>) -> &mut Self {
        self.element("date", XmpNamespace::DublinCore)
            .ordered_array(date);
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

    pub fn relation<'a>(&mut self, relation: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.element("relation", XmpNamespace::DublinCore)
            .unordered_array(relation);
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
                .array(RdfCollectionType::Seq),
        )
    }

    pub fn ingredients<'a>(&mut self) -> ResourceRefsWriter<'_> {
        ResourceRefsWriter::start(
            self.element("Ingredients", XmpNamespace::XmpMedia)
                .array(RdfCollectionType::Bag),
        )
    }

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

    pub fn pantry(&mut self) -> PantryWriter<'_> {
        PantryWriter::start(
            self.element("Pantry", XmpNamespace::XmpMedia)
                .array(RdfCollectionType::Bag),
        )
    }

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

    pub fn version_ref(&mut self) -> VersionsWriter<'_> {
        VersionsWriter::start(
            self.element("Versions", XmpNamespace::XmpMedia)
                .array(RdfCollectionType::Seq),
        )
    }
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

/// Basic Job Management.
impl XmpWriter {
    pub fn jobs(&mut self) -> JobWriter<'_> {
        JobWriter::start(self.element("Job", XmpNamespace::XmpJobManagement).obj())
    }
}

/// Paged-text.
impl XmpWriter {
    pub fn colorants(&mut self) -> ColorantsWriter<'_> {
        ColorantsWriter::start(
            self.element("Colorants", XmpNamespace::XmpPaged)
                .array(RdfCollectionType::Seq),
        )
    }

    pub fn fonts(&mut self) -> FontsWriter<'_> {
        FontsWriter::start(
            self.element("Fonts", XmpNamespace::XmpPaged)
                .array(RdfCollectionType::Bag),
        )
    }

    pub fn max_page_size(&mut self) -> DimensionsWriter<'_> {
        DimensionsWriter::start(self.element("MaxPageSize", XmpNamespace::XmpPaged).obj())
    }

    pub fn num_pages(&mut self, num: u32) -> &mut Self {
        self.element("NPages", XmpNamespace::XmpPaged)
            .value(num as i64);
        self
    }

    pub fn plate_names<'a>(&mut self, names: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.element("PlateNames", XmpNamespace::XmpPaged)
            .ordered_array(names);
        self
    }
}

// TODO: Dynamic Media

/// XMPIDQ.
impl XmpWriter {
    pub fn idq_scheme<'a>(&mut self, scheme: &'a str) -> &mut Self {
        self.element("Scheme", XmpNamespace::XmpIdq).value(scheme);
        self
    }
}

/// Adobe PDF.
impl XmpWriter {
    pub fn pdf_keywords<'a>(&mut self, keywords: &'a str) -> &mut Self {
        self.element("Keywords", XmpNamespace::AdobePdf)
            .value(keywords);
        self
    }

    pub fn pdf_version<'a>(&mut self, version: &'a str) -> &mut Self {
        self.element("PDFVersion", XmpNamespace::AdobePdf)
            .value(version);
        self
    }

    pub fn producer<'a>(&mut self, producer: &'a str) -> &mut Self {
        self.element("Producer", XmpNamespace::AdobePdf)
            .value(producer);
        self
    }

    pub fn trapped(&mut self, trapped: bool) -> &mut Self {
        self.element("Trapped", XmpNamespace::AdobePdf)
            .value(trapped);
        self
    }
}

/// PDF/A and PDF/X.
impl XmpWriter {
    pub fn pdfa_part<'a>(&mut self, part: &'a str) -> &mut Self {
        self.element("part", XmpNamespace::PdfAId).value(part);
        self
    }

    pub fn pdfa_conformance<'a>(&mut self, conformance: &'a str) -> &mut Self {
        self.element("conformance", XmpNamespace::PdfAId)
            .value(conformance);
        self
    }

    pub fn pdfx_version<'a>(&mut self, version: &'a str) -> &mut Self {
        self.element("GTS_PDFXVersion", XmpNamespace::PdfXId)
            .value(version);
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

pub struct ResourceRefsWriter<'a> {
    array: ArrayWriter<'a>,
}

impl<'a> ResourceRefsWriter<'a> {
    pub fn start(array: ArrayWriter<'a>) -> Self {
        Self { array }
    }

    pub fn add_ref(&mut self) -> ResourceRefWriter<'_> {
        ResourceRefWriter::start(self.array.element().obj())
    }
}

pub struct PantryItemWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a> PantryItemWriter<'a> {
    pub fn start(stc: XmpStruct<'a>) -> Self {
        Self { stc }
    }

    pub fn instance_id<'b>(&mut self, id: &'b str) -> &mut Self {
        self.stc
            .element("instanceID", XmpNamespace::XmpMedia)
            .value(id);
        self
    }

    pub fn element<'c: 'a>(&mut self, name: &'c str, namespace: XmpNamespace) -> XmpElement<'_> {
        self.stc.element(name, namespace)
    }

    pub fn element_with_attrs<'c: 'a, 'd>(
        &mut self,
        name: &'c str,
        namespace: XmpNamespace,
        attrs: impl IntoIterator<Item = (&'d str, &'d str)>,
    ) -> XmpElement<'_> {
        self.stc.element_with_attrs(name, namespace, attrs)
    }
}

pub struct PantryWriter<'a> {
    array: ArrayWriter<'a>,
}

impl<'a> PantryWriter<'a> {
    pub fn start(array: ArrayWriter<'a>) -> Self {
        Self { array }
    }

    pub fn add_item(&mut self) -> PantryItemWriter<'_> {
        PantryItemWriter::start(self.array.element().obj())
    }
}

pub struct VersionWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a> VersionWriter<'a> {
    pub fn start(stc: XmpStruct<'a>) -> Self {
        Self { stc }
    }

    pub fn comments<'b>(&mut self, comments: &'b str) -> &mut Self {
        self.stc
            .element("comments", XmpNamespace::XmpVersion)
            .value(comments);
        self
    }

    pub fn event(&mut self) -> ResourceEventWriter<'_> {
        ResourceEventWriter::start(self.stc.element("event", XmpNamespace::XmpVersion).obj())
    }

    pub fn modifier<'b>(&mut self, modifier: &'b str) -> &mut Self {
        self.stc
            .element("modifier", XmpNamespace::XmpVersion)
            .value(modifier);
        self
    }

    pub fn modify_date(&mut self, date: XmpDate) -> &mut Self {
        self.stc
            .element("modifyDate", XmpNamespace::XmpVersion)
            .value(date);
        self
    }

    pub fn version<'b>(&mut self, version: &'b str) -> &mut Self {
        self.stc
            .element("version", XmpNamespace::XmpVersion)
            .value(version);
        self
    }
}

pub struct VersionsWriter<'a> {
    array: ArrayWriter<'a>,
}

impl<'a> VersionsWriter<'a> {
    pub fn start(array: ArrayWriter<'a>) -> Self {
        Self { array }
    }

    pub fn add_version(&mut self) -> VersionWriter<'_> {
        VersionWriter::start(self.array.element().obj())
    }
}

pub struct JobWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a> JobWriter<'a> {
    pub fn start(stc: XmpStruct<'a>) -> Self {
        Self { stc }
    }

    pub fn id<'b>(&mut self, id: &'b str) -> &mut Self {
        self.stc.element("id", XmpNamespace::XmpJob).value(id);
        self
    }

    pub fn name<'b>(&mut self, name: &'b str) -> &mut Self {
        self.stc.element("name", XmpNamespace::XmpJob).value(name);
        self
    }

    pub fn url<'b>(&mut self, url: &'b str) -> &mut Self {
        self.stc.element("url", XmpNamespace::XmpJob).value(url);
        self
    }
}

pub struct ColorantWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a> ColorantWriter<'a> {
    pub fn start(stc: XmpStruct<'a>) -> Self {
        Self { stc }
    }

    pub fn type_(&mut self, kind: ColorantType) -> &mut Self {
        self.stc
            .element("type", XmpNamespace::XmpColorant)
            .value(kind);
        self
    }

    pub fn swatch_name<'b>(&mut self, name: &'b str) -> &mut Self {
        self.stc
            .element("swatchName", XmpNamespace::XmpColorant)
            .value(name);
        self
    }

    pub fn colorant_mode(&mut self, mode: ColorantMode) -> &mut Self {
        self.stc
            .element("colorantMode", XmpNamespace::XmpColorant)
            .value(mode);
        self
    }

    pub fn l(&mut self, l: f64) -> &mut Self {
        self.stc.element("L", XmpNamespace::XmpColorant).value(l);
        self
    }

    pub fn a(&mut self, a: i32) -> &mut Self {
        self.stc.element("a", XmpNamespace::XmpColorant).value(a);
        self
    }

    pub fn b(&mut self, b: i32) -> &mut Self {
        self.stc.element("b", XmpNamespace::XmpColorant).value(b);
        self
    }

    pub fn black(&mut self, black: f64) -> &mut Self {
        self.stc
            .element("black", XmpNamespace::XmpColorant)
            .value(black);
        self
    }

    pub fn cyan(&mut self, cyan: f64) -> &mut Self {
        self.stc
            .element("cyan", XmpNamespace::XmpColorant)
            .value(cyan);
        self
    }

    pub fn magenta(&mut self, magenta: f64) -> &mut Self {
        self.stc
            .element("magenta", XmpNamespace::XmpColorant)
            .value(magenta);
        self
    }

    pub fn yellow(&mut self, yellow: f64) -> &mut Self {
        self.stc
            .element("yellow", XmpNamespace::XmpColorant)
            .value(yellow);
        self
    }

    pub fn red(&mut self, red: i32) -> &mut Self {
        self.stc
            .element("red", XmpNamespace::XmpColorant)
            .value(red);
        self
    }

    pub fn green(&mut self, green: i32) -> &mut Self {
        self.stc
            .element("green", XmpNamespace::XmpColorant)
            .value(green);
        self
    }

    pub fn blue(&mut self, blue: i32) -> &mut Self {
        self.stc
            .element("blue", XmpNamespace::XmpColorant)
            .value(blue);
        self
    }
}

pub struct ColorantsWriter<'a> {
    array: ArrayWriter<'a>,
}

impl<'a> ColorantsWriter<'a> {
    pub fn start(array: ArrayWriter<'a>) -> Self {
        Self { array }
    }

    pub fn add_colorant(&mut self) -> ColorantWriter<'_> {
        ColorantWriter::start(self.array.element().obj())
    }
}

pub struct DimensionsWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a> DimensionsWriter<'a> {
    pub fn start(stc: XmpStruct<'a>) -> Self {
        Self { stc }
    }

    pub fn width(&mut self, width: f64) -> &mut Self {
        self.stc
            .element("width", XmpNamespace::XmpDimensions)
            .value(width);
        self
    }

    pub fn height(&mut self, height: f64) -> &mut Self {
        self.stc
            .element("height", XmpNamespace::XmpDimensions)
            .value(height);
        self
    }

    pub fn unit(&mut self, unit: DimensionUnit) -> &mut Self {
        self.stc
            .element("unit", XmpNamespace::XmpDimensions)
            .value(unit);
        self
    }
}

pub struct FontWriter<'a> {
    stc: XmpStruct<'a>,
}

impl<'a> FontWriter<'a> {
    pub fn start(stc: XmpStruct<'a>) -> Self {
        Self { stc }
    }

    pub fn child_font_files<'b>(&mut self, files: impl IntoIterator<Item = &'b str>) -> &mut Self {
        self.stc
            .element("childFontFiles", XmpNamespace::XmpFont)
            .ordered_array(files);
        self
    }

    pub fn composite(&mut self, composite: bool) -> &mut Self {
        self.stc
            .element("composite", XmpNamespace::XmpFont)
            .value(composite);
        self
    }

    pub fn font_face<'b>(&mut self, face: &'b str) -> &mut Self {
        self.stc
            .element("fontFace", XmpNamespace::XmpFont)
            .value(face);
        self
    }

    pub fn font_family<'b>(&mut self, family: &'b str) -> &mut Self {
        self.stc
            .element("fontFamily", XmpNamespace::XmpFont)
            .value(family);
        self
    }

    pub fn font_file<'b>(&mut self, file_name: &'b str) -> &mut Self {
        self.stc
            .element("fontFileName", XmpNamespace::XmpFont)
            .value(file_name);
        self
    }

    pub fn font_name<'b>(&mut self, name: &'b str) -> &mut Self {
        self.stc
            .element("fontName", XmpNamespace::XmpFont)
            .value(name);
        self
    }

    pub fn font_type(&mut self, font_type: FontType) -> &mut Self {
        self.stc
            .element("fontType", XmpNamespace::XmpFont)
            .value(font_type);
        self
    }

    pub fn version_string<'b>(&mut self, version: &'b str) -> &mut Self {
        self.stc
            .element("versionString", XmpNamespace::XmpFont)
            .value(version);
        self
    }
}

pub struct FontsWriter<'a> {
    array: ArrayWriter<'a>,
}

impl<'a> FontsWriter<'a> {
    pub fn start(array: ArrayWriter<'a>) -> Self {
        Self { array }
    }

    pub fn add_font(&mut self) -> FontWriter<'_> {
        FontWriter::start(self.array.element().obj())
    }
}
