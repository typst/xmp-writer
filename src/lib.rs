/*!
*Write XMP metadata, step by step.*

XMP is a metadata format developed by Adobe. It is either embedded into
files (e.g. PDF, JPEG, TIFF) or stored in a separate "side-car" file.

This crate provides a simple API to write XMP metadata. Start by creating
a new [`XmpWriter`], then add entries to it. Finally, call [`XmpWriter::finish`] to
get the XMP metadata as a byte vector. Some properties contain a complex data type like a
struct or an array. In this case, the writer returns a new struct that can be used to
write the data. The reference to the struct must be dropped before the writer can be used
again.

## Example

```rust
use xmp_writer::{LangId, DateTime, XmpWriter};

let mut writer = XmpWriter::new();
writer.creator(["Martin Haug"]);
writer.title([(Some(LangId("de")), "Titel"), (None, "Title")]);
writer.num_pages(3);
writer.pdf_keywords("Keyword1, Keyword2");
writer.description([(None, "Description")]);
writer.date([DateTime::date(2021, 11, 06)]);

let mut colors = writer.colorants();
colors.add_colorant().swatch_name("Red");
colors.add_colorant().swatch_name("Green");
drop(colors);

writer.creator_tool("xmp-writer 0.2.0");

println!("{}", writer.finish(None));
```

## See also
- [XMP Specification, Part 1: Basics](https://github.com/adobe/XMP-Toolkit-SDK/blob/main/docs/XMPSpecificationPart1.pdf)
- [XMP Specification, Part 2: Additional Properties](https://github.com/adobe/XMP-Toolkit-SDK/blob/main/docs/XMPSpecificationPart2.pdf)
- [XMP Specification, Part 3: File Embedding and Interchange](https://github.com/adobe/XMP-Toolkit-SDK/blob/main/docs/XMPSpecificationPart3.pdf)
*/

#![deny(missing_docs)]

mod types;

use std::collections::BTreeSet;
use std::fmt::Write;

pub use types::*;

/// Implements `Deref` and `DerefMut` by delegating to a field of a struct.
macro_rules! deref {
    ($a:lifetime, $b:lifetime, $from:ty => $to:ty, $field:ident) => {
        impl<$a, $b> std::ops::Deref for $from {
            type Target = $to;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl<$a, $b> std::ops::DerefMut for $from {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}

/// The main writer struct.
///
/// Use [`XmpWriter::new`] to create a new instance and get the resulting XMP
/// metadata by calling [`XmpWriter::finish`].
pub struct XmpWriter<'a> {
    pub(crate) buf: String,
    namespaces: BTreeSet<Namespace<'a>>,
}

impl<'n> XmpWriter<'n> {
    /// Create a new XMP writer.
    pub fn new() -> XmpWriter<'n> {
        Self { buf: String::new(), namespaces: BTreeSet::new() }
    }

    /// Add a custom element to the XMP metadata.
    #[inline]
    pub fn element<'a>(
        &'a mut self,
        name: &'a str,
        namespace: Namespace<'n>,
    ) -> Element<'a, 'n> {
        Element::start(self, name, namespace)
    }

    /// Finish the XMP metadata and return it as a byte vector.
    pub fn finish(self, about: Option<&str>) -> String {
        let mut buf = String::with_capacity(280 + self.buf.len());
        buf.push_str("<?xpacket begin=\"\u{feff}\" id=\"W5M0MpCehiHzreSzNTczkc9d\"?>");

        write!(
            &mut buf,
            "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"xmp-writer\"><rdf:RDF xmlns:rdf=\"{}\"><rdf:Description rdf:about=\"{}\"",
            Namespace::Rdf.url(),
            about.unwrap_or(""),
        )
        .unwrap();

        for namespace in self.namespaces.into_iter().filter(|ns| &Namespace::Rdf != ns) {
            write!(&mut buf, " xmlns:{}=\"{}\" ", namespace.prefix(), namespace.url())
                .unwrap();
        }

        buf.push('>');
        buf.push_str(&self.buf);
        buf.push_str("</rdf:Description></rdf:RDF></x:xmpmeta><?xpacket end=\"r\"?>");
        buf
    }
}

/// XMP Dublin Core Schema.
impl XmpWriter<'_> {
    /// Write the `dc:contributor` property.
    ///
    /// All entities responsible for making contributions to the resource not
    /// listed in [`XmpWriter::creator`].
    pub fn contributor<'a>(
        &mut self,
        contributor: impl IntoIterator<Item = &'a str>,
    ) -> &mut Self {
        self.element("contributor", Namespace::DublinCore)
            .unordered_array(contributor);
        self
    }

    /// Write the `dc:coverage` property.
    ///
    /// The scope of the resource.
    pub fn coverage(&mut self, coverage: &str) -> &mut Self {
        self.element("coverage", Namespace::DublinCore).value(coverage);
        self
    }

    /// Write the `dc:creator` property.
    ///
    /// An entity primarily responsible for making the resource.
    pub fn creator<'a>(
        &mut self,
        creator: impl IntoIterator<Item = &'a str>,
    ) -> &mut Self {
        self.element("creator", Namespace::DublinCore).ordered_array(creator);
        self
    }

    /// Write the `dc:date` property.
    ///
    /// Date(s) that something happened to the resource.
    pub fn date(&mut self, date: impl IntoIterator<Item = DateTime>) -> &mut Self {
        self.element("date", Namespace::DublinCore).ordered_array(date);
        self
    }

    /// Write the `dc:description` property.
    ///
    /// An account of the resource, possibly in multiple languages.
    pub fn description<'a>(
        &mut self,
        description: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> &mut Self {
        self.element("description", Namespace::DublinCore)
            .language_alternative(description);
        self
    }

    /// Write the `dc:format` property.
    ///
    /// The mime type of the resource.
    pub fn format(&mut self, mime: &str) -> &mut Self {
        self.element("format", Namespace::DublinCore).value(mime);
        self
    }

    /// Write the `dc:identifier` property.
    ///
    /// An unambiguous reference to the resource within a given context.
    pub fn identifier(&mut self, id: &str) -> &mut Self {
        self.element("identifier", Namespace::DublinCore).value(id);
        self
    }

    /// Write the `dc:language` property.
    ///
    /// Languges used in the resource.
    pub fn language<'a>(
        &mut self,
        lang: impl IntoIterator<Item = LangId<'a>>,
    ) -> &mut Self {
        self.element("language", Namespace::DublinCore).unordered_array(lang);
        self
    }

    /// Write the `dc:publisher` property.
    ///
    /// Publishers of the resource.
    pub fn publisher<'a>(
        &mut self,
        publisher: impl IntoIterator<Item = &'a str>,
    ) -> &mut Self {
        self.element("publisher", Namespace::DublinCore)
            .unordered_array(publisher);
        self
    }

    /// Write the `dc:relation` property.
    ///
    /// List of related resources.
    pub fn relation<'a>(
        &mut self,
        relation: impl IntoIterator<Item = &'a str>,
    ) -> &mut Self {
        self.element("relation", Namespace::DublinCore)
            .unordered_array(relation);
        self
    }

    /// Write the `dc:rights` property.
    ///
    /// Informal rights statements, possibly in multiple languages.
    pub fn rights<'a>(
        &mut self,
        rights: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> &mut Self {
        self.element("rights", Namespace::DublinCore)
            .language_alternative(rights);
        self
    }

    /// Write the `dc:source` property.
    ///
    /// A related resource from which the described resource is derived.
    pub fn source(&mut self, source: &str) -> &mut Self {
        self.element("source", Namespace::DublinCore).value(source);
        self
    }

    /// Write the `dc:subject` property.
    ///    
    /// A list of phrases or keywords that specify the topic of the resource.
    pub fn subject<'a>(
        &mut self,
        subject: impl IntoIterator<Item = &'a str>,
    ) -> &mut Self {
        self.element("subject", Namespace::DublinCore)
            .unordered_array(subject);
        self
    }

    /// Write the `dc:title` property.
    ///
    /// A name given to the resource, possibly in multiple languages.
    pub fn title<'a>(
        &mut self,
        title: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> &mut Self {
        self.element("title", Namespace::DublinCore)
            .language_alternative(title);
        self
    }

    /// Write the `dc:type` property.
    ///
    /// The nature or genre of the resource. Please use [`XmpWriter::format`] to
    /// specify the mime type.
    pub fn type_<'a>(&mut self, kind: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.element("type", Namespace::DublinCore).unordered_array(kind);
        self
    }
}

/// XMP Basic Schema.
impl<'n> XmpWriter<'n> {
    /// Write the `xmp:BaseURL` property.
    ///
    /// The base URL for relative URLs in the document.
    pub fn base_url(&mut self, url: &str) -> &mut Self {
        self.element("BaseURL", Namespace::Xmp).value(url);
        self
    }

    /// Write the `xmp:CreateDate` property.
    ///
    /// The date and time the resource was created.
    pub fn create_date(&mut self, date: DateTime) -> &mut Self {
        self.element("CreateDate", Namespace::Xmp).value(date);
        self
    }

    /// Write the `xmp:CreatorTool` property.
    ///
    /// The name of the application used to create the resource.
    pub fn creator_tool(&mut self, tool: &str) -> &mut Self {
        self.element("CreatorTool", Namespace::Xmp).value(tool);
        self
    }

    /// Write the `xmp:Identifier` property.
    ///
    /// Unordered array of text strings that identify the resource. The
    /// [`XmpWriter::idq_scheme`] method can be used to specify the scheme.
    pub fn xmp_identifier<'a>(
        &mut self,
        id: impl IntoIterator<Item = &'a str>,
    ) -> &mut Self {
        self.element("Identifier", Namespace::Xmp).unordered_array(id);
        self
    }

    /// Write the `xmp:Label` property.
    ///
    /// A user-defined label for the resource.
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.element("Label", Namespace::Xmp).value(label);
        self
    }

    /// Write the `xmp:MetadataDate` property.
    ///
    /// The date and time the metadata for the resource was last changed.
    pub fn metadata_date(&mut self, date: DateTime) -> &mut Self {
        self.element("MetadataDate", Namespace::Xmp).value(date);
        self
    }

    /// Write the `xmp:ModifyDate` property.
    ///
    /// The date and time the resource was last modified.
    pub fn modify_date(&mut self, date: DateTime) -> &mut Self {
        self.element("ModifyDate", Namespace::Xmp).value(date);
        self
    }

    /// Write the `xmp:Nickname` property.
    ///
    /// A short informal name for the resource.
    pub fn nickname(&mut self, nickname: &str) -> &mut Self {
        self.element("Nickname", Namespace::Xmp).value(nickname);
        self
    }

    /// Write the `xmp:Rating` property.
    ///
    /// A user-assigned rating of the resource.
    pub fn rating(&mut self, rating: i64) -> &mut Self {
        self.element("Rating", Namespace::Xmp).value(rating);
        self
    }

    /// Start writing the `xmp:Thumbnails` property.
    ///
    /// A thumbnail image of the resource.
    pub fn thumbnails(&mut self) -> ThumbnailsWriter<'_, 'n> {
        ThumbnailsWriter::start(
            self.element("Thumbnails", Namespace::Xmp)
                .array(RdfCollectionType::Alt),
        )
    }
}

/// XMP Rights Management Schema.
impl XmpWriter<'_> {
    /// Write the `xmpRights:Certificate` property.
    ///
    /// A URL with a rights management certificate.
    pub fn certificate(&mut self, cert: &str) -> &mut Self {
        self.element("Certificate", Namespace::XmpRights).value(cert);
        self
    }

    /// Write the `xmpRights:Marked` property.
    ///
    /// Whether the resource has been marked as rights managed. If false, the
    /// resource is in the public domain.
    pub fn marked(&mut self, marked: bool) -> &mut Self {
        self.element("Marked", Namespace::XmpRights).value(marked);
        self
    }

    /// Write the `xmpRights:Owner` property.
    ///
    /// A list of people or organizations owning the resource.
    pub fn owner<'a>(&mut self, owner: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.element("Owner", Namespace::XmpRights).unordered_array(owner);
        self
    }

    /// Write the `xmpRights:UsageTerms` property.
    ///
    /// Under what conditions the resource may be used.
    pub fn usage_terms<'a>(
        &mut self,
        terms: impl IntoIterator<Item = (Option<LangId<'a>>, &'a str)>,
    ) -> &mut Self {
        self.element("UsageTerms", Namespace::XmpRights)
            .language_alternative(terms);
        self
    }

    /// Write the `xmpRights:WebStatement` property.
    ///
    /// A URL with a rights management statement.
    pub fn web_statement(&mut self, statement: &str) -> &mut Self {
        self.element("WebStatement", Namespace::XmpRights).value(statement);
        self
    }
}

/// XMP Media Management Schema.
impl<'n> XmpWriter<'n> {
    /// Start writing the `xmpMM:DerivedFrom` property.
    ///
    /// The document from which this document is derived.
    pub fn derived_from(&mut self) -> ResourceRefWriter<'_, 'n> {
        ResourceRefWriter::start(self.element("DerivedFrom", Namespace::XmpMedia).obj())
    }

    /// Write the `xmpMM:DocumentID` property.
    ///
    /// A common identifier for the document and all of its versions /
    /// renditions.
    pub fn document_id(&mut self, id: &str) -> &mut Self {
        self.element("DocumentID", Namespace::XmpMedia).value(id);
        self
    }

    /// Start writing the `xmpMM:History` property.
    ///
    /// A list of actions taken on the document.
    pub fn history<'a>(&mut self) -> ResourceEventsWriter<'_, 'n> {
        ResourceEventsWriter::start(
            self.element("History", Namespace::XmpMedia)
                .array(RdfCollectionType::Seq),
        )
    }

    /// Write the `xmpMM:Ingredients` property.
    ///
    /// A list of resources that were used to create the document.
    pub fn ingredients<'a>(&mut self) -> ResourceRefsWriter<'_, 'n> {
        ResourceRefsWriter::start(
            self.element("Ingredients", Namespace::XmpMedia)
                .array(RdfCollectionType::Bag),
        )
    }

    /// Write the `xmpMM:InstanceID` property.
    ///
    /// A unique identifier for the rendition of the document, updated each
    /// time the document is saved.
    pub fn instance_id(&mut self, id: &str) -> &mut Self {
        self.element("InstanceID", Namespace::XmpMedia).value(id);
        self
    }

    /// Start writing the `xmpMM:ManagedFrom` property.
    ///
    /// A reference to the document before it was managed.
    pub fn managed_from(&mut self) -> ResourceRefWriter<'_, 'n> {
        ResourceRefWriter::start(self.element("ManagedFrom", Namespace::XmpMedia).obj())
    }

    /// Write the `xmpMM:Manager` property.
    ///
    /// The name of the application that manages the document.
    pub fn manager(&mut self, manager: &str) -> &mut Self {
        self.element("Manager", Namespace::XmpMedia).value(manager);
        self
    }

    /// Write the `xmpMM:ManageTo` property.
    ///
    /// The URI of the document in the management system.
    pub fn manage_to(&mut self, uri: &str) -> &mut Self {
        self.element("ManageTo", Namespace::XmpMedia).value(uri);
        self
    }

    /// Write the `xmpMM:ManageUI` property.
    ///
    /// A web page that allows the user to manage the document.
    pub fn manage_ui(&mut self, uri: &str) -> &mut Self {
        self.element("ManageUI", Namespace::XmpMedia).value(uri);
        self
    }

    /// Write the `xmpMM:ManagerVariant` property.
    ///
    /// The name of the variant of the application that manages the document.
    pub fn manager_variant(&mut self, variant: &str) -> &mut Self {
        self.element("ManagerVariant", Namespace::XmpMedia).value(variant);
        self
    }

    /// Write the `xmpMM:OriginalDocumentID` property.
    ///
    /// The ID of the resource from which this document was derived.
    pub fn original_doc_id(&mut self, id: &str) -> &mut Self {
        self.element("OriginalDocumentID", Namespace::XmpMedia).value(id);
        self
    }

    /// Start writing the `xmpMM:Pantry` property.
    ///
    /// An unordered array of structs with custom properties, each of which must
    /// have an `xmpMM:InstanceID` property.
    pub fn pantry(&mut self) -> PantryWriter<'_, 'n> {
        PantryWriter::start(
            self.element("Pantry", Namespace::XmpMedia)
                .array(RdfCollectionType::Bag),
        )
    }

    /// Write the `xmpMM:RenditionClass` property.
    ///
    /// The type of the rendition. Shall be absent or [`RenditionClass::Default`]
    /// if this is not a derived document.
    pub fn rendition_class(&mut self, class: RenditionClass) -> &mut Self {
        self.element("RenditionClass", Namespace::XmpMedia).value(class);
        self
    }

    /// Write the `xmpMM:RenditionParams` property.
    ///
    /// The parameters used to create the rendition.
    pub fn rendition_params(&mut self, params: &str) -> &mut Self {
        self.element("RenditionParams", Namespace::XmpMedia).value(params);
        self
    }

    /// Write the `xmpMM:VersionID` property.
    ///
    /// A unique identifier for the version of the document.
    pub fn version_id(&mut self, id: &str) -> &mut Self {
        self.element("VersionID", Namespace::XmpMedia).value(id);
        self
    }

    /// Start writing the `xmpMM:Versions` property.
    ///
    /// The list of versions of the document, starting with the oldest version.
    pub fn version_ref(&mut self) -> VersionsWriter<'_, 'n> {
        VersionsWriter::start(
            self.element("Versions", Namespace::XmpMedia)
                .array(RdfCollectionType::Seq),
        )
    }
}

/// Basic Job Management.
impl<'n> XmpWriter<'n> {
    /// Start writing the `xmpBJ:JobRef` property.
    ///
    /// A reference to jobs in a system that involves this resource.
    pub fn jobs(&mut self) -> JobsWriter<'_, 'n> {
        JobsWriter::start(
            self.element("Job", Namespace::XmpJobManagement)
                .array(RdfCollectionType::Bag),
        )
    }
}

/// Paged-text.
impl<'n> XmpWriter<'n> {
    /// Start writing the `xmpTPg:NPages` property.
    ///
    /// Colorants used in the document.
    pub fn colorants(&mut self) -> ColorantsWriter<'_, 'n> {
        ColorantsWriter::start(
            self.element("Colorants", Namespace::XmpPaged)
                .array(RdfCollectionType::Seq),
        )
    }

    /// Start writing the `xmpTPg:Fonts` property.
    ///
    /// Fonts used in the document.
    pub fn fonts(&mut self) -> FontsWriter<'_, 'n> {
        FontsWriter::start(
            self.element("Fonts", Namespace::XmpPaged)
                .array(RdfCollectionType::Bag),
        )
    }

    /// Start writing the `xmpTPg:MaxPageSize` property.
    ///
    /// The maximum page size in the document.
    pub fn max_page_size(&mut self) -> DimensionsWriter<'_, 'n> {
        DimensionsWriter::start(self.element("MaxPageSize", Namespace::XmpPaged).obj())
    }

    /// Write the `xmpTPg:NPages` property.
    ///
    /// The number of pages in the document.
    pub fn num_pages(&mut self, num: u32) -> &mut Self {
        self.element("NPages", Namespace::XmpPaged).value(num as i64);
        self
    }

    /// Write the `xmpTPg:PlateNames` property.
    ///
    /// The names of the plates needed to print the document.
    pub fn plate_names<'a>(
        &mut self,
        names: impl IntoIterator<Item = &'a str>,
    ) -> &mut Self {
        self.element("PlateNames", Namespace::XmpPaged).ordered_array(names);
        self
    }
}

// TODO: Dynamic Media

/// XMPIDQ.
impl XmpWriter<'_> {
    /// Write the `xmpidq:GImg` property.
    ///
    /// Identifies the scheme of the [`XmpWriter::xmp_identifier`] property.
    pub fn idq_scheme(&mut self, scheme: &str) -> &mut Self {
        self.element("Scheme", Namespace::XmpIdq).value(scheme);
        self
    }
}

/// Adobe PDF.
impl XmpWriter<'_> {
    /// Write the `pdf:Keywords` property.
    ///
    /// The document's keywords.
    pub fn pdf_keywords(&mut self, keywords: &str) -> &mut Self {
        self.element("Keywords", Namespace::AdobePdf).value(keywords);
        self
    }

    /// Write the `pdf:PDFVersion` property.
    ///
    /// The version of the PDF specification to which the document conforms
    /// (e.g. `"1.0", "1.7"`)
    pub fn pdf_version(&mut self, version: &str) -> &mut Self {
        self.element("PDFVersion", Namespace::AdobePdf).value(version);
        self
    }

    /// Write the `pdf:Producer` property.
    ///
    /// The name of the application that created the PDF document.
    pub fn producer(&mut self, producer: &str) -> &mut Self {
        self.element("Producer", Namespace::AdobePdf).value(producer);
        self
    }

    /// Write the `pdf:Trapped` property.
    ///
    /// Whether the document has been trapped.
    pub fn trapped(&mut self, trapped: bool) -> &mut Self {
        self.element("Trapped", Namespace::AdobePdf).value(trapped);
        self
    }
}

/// PDF/A and PDF/X.
impl XmpWriter<'_> {
    /// Write the `pdfaid:part` property.
    ///
    /// The part of the PDF/A standard to which the document conforms (e.g.
    /// `"1", "4"`)
    pub fn pdfa_part(&mut self, part: &str) -> &mut Self {
        self.element("part", Namespace::PdfAId).value(part);
        self
    }

    /// Write the `pdfaid:conformance` property.
    ///
    /// The conformance level of the PDF/A standard to which the document
    /// conforms (e.g. `"A", "B"`)
    pub fn pdfa_conformance(&mut self, conformance: &str) -> &mut Self {
        self.element("conformance", Namespace::PdfAId).value(conformance);
        self
    }

    /// Write the `pdfxid:GTS_PDFXVersion` property.
    ///
    /// The version of the PDF/X standard to which the document conforms (e.g.
    /// `"PDF/X-3:2003"`)
    pub fn pdfx_version(&mut self, version: &str) -> &mut Self {
        self.element("GTS_PDFXVersion", Namespace::PdfXId).value(version);
        self
    }
}

/// A self-contained thumbnail image.
///
/// Created by [`ThumbnailsWriter::add_thumbnail`].
pub struct ThumbnailWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n: 'a> ThumbnailWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `xmpGImg:format` property with a custom format of the
    /// thumbnail image. Must be "JPEG" for now.
    pub fn format(&mut self, format: &str) -> &mut Self {
        self.stc.element("format", Namespace::XmpImage).value(format);
        self
    }

    /// Write the `xmpGImg:format` property with the value "JPEG".
    pub fn format_jpeg(&mut self) -> &mut Self {
        self.format("JPEG")
    }

    /// Write the `xmpGImg:width` property.
    pub fn width(&mut self, width: u64) -> &mut Self {
        self.stc.element("width", Namespace::XmpImage).value(width as i64);
        self
    }

    /// Write the `xmpGImg:height` property.
    pub fn height(&mut self, height: u64) -> &mut Self {
        self.stc.element("height", Namespace::XmpImage).value(height as i64);
        self
    }

    /// Write the `xmpGImg:image` property.
    ///
    /// The image must be a base64-encoded JPEG.
    pub fn image(&mut self, image: &str) -> &mut Self {
        self.stc.element("image", Namespace::XmpImage).value(image);
        self
    }
}

deref!('a, 'n, ThumbnailWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Write a set of thumbnails.
///
/// Created by [`XmpWriter::thumbnails`].
pub struct ThumbnailsWriter<'a, 'n: 'a> {
    array: Array<'a, 'n>,
}

impl<'a, 'n: 'a> ThumbnailsWriter<'a, 'n> {
    fn start(array: Array<'a, 'n>) -> Self {
        Self { array }
    }

    /// Add a thumbnail.
    pub fn add_thumbnail(&mut self) -> ThumbnailWriter<'_, 'n> {
        ThumbnailWriter::start(self.array.element().obj())
    }
}

deref!('a, 'n, ThumbnailsWriter<'a, 'n> => Array<'a, 'n>, array);

/// Writer for a reference to a resource.
///
/// Created by [`XmpWriter::derived_from`], [`XmpWriter::managed_from`], or [`ResourceRefsWriter::add_ref`].
pub struct ResourceRefWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n: 'a> ResourceRefWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `stRef:alternatePaths` property.
    ///
    /// Fallback paths to the resource.
    pub fn alternate_paths<'b>(
        &mut self,
        paths: impl IntoIterator<Item = &'b str>,
    ) -> &mut Self {
        self.stc
            .element("alternatePaths", Namespace::XmpResourceRef)
            .ordered_array(paths);
        self
    }

    /// Write the `stRef:documentID` property.
    ///
    /// The [`XmpWriter::document_id`] of the referenced resource.
    pub fn document_id(&mut self, id: &str) -> &mut Self {
        self.stc.element("documentID", Namespace::XmpResourceRef).value(id);
        self
    }

    /// Write the `stRef:filePath` property.
    ///
    /// The path or URL to the resource.
    pub fn file_path(&mut self, path: &str) -> &mut Self {
        self.stc.element("filePath", Namespace::XmpResourceRef).value(path);
        self
    }

    /// Write the `stRef:instanceID` property.
    ///
    /// The [`XmpWriter::instance_id`] of the referenced resource.
    pub fn instance_id(&mut self, id: &str) -> &mut Self {
        self.stc.element("instanceID", Namespace::XmpResourceRef).value(id);
        self
    }

    /// Write the `stRef:lastModifyDate` property.
    ///
    /// The last modification date of the resource. See [`ResourceEventWriter::when`].
    pub fn last_modify_date(&mut self, date: DateTime) -> &mut Self {
        self.stc
            .element("lastModifyDate", Namespace::XmpResourceRef)
            .value(date);
        self
    }

    /// Write the `stRef:manager` property.
    ///
    /// The name of the application that manages the resource. See [`XmpWriter::manager`].
    pub fn manager(&mut self, manager: &str) -> &mut Self {
        self.stc.element("manager", Namespace::XmpResourceRef).value(manager);
        self
    }

    /// Write the `stRef:managerVariant` property.
    ///
    /// The variant of the application that manages the resource. See [`XmpWriter::manager_variant`].
    pub fn manager_variant(&mut self, variant: &str) -> &mut Self {
        self.stc
            .element("managerVariant", Namespace::XmpResourceRef)
            .value(variant);
        self
    }

    /// Write the `stRef:manageTo` property.
    ///
    /// The URI of the resource prior to being managed. See [`XmpWriter::manage_to`].
    pub fn manage_to(&mut self, uri: &str) -> &mut Self {
        self.stc.element("manageTo", Namespace::XmpResourceRef).value(uri);
        self
    }

    /// Write the `stRef:manageUI` property.
    ///
    /// An URI to the user interface of the application that manages the resource. See [`XmpWriter::manage_ui`].
    pub fn manage_ui(&mut self, uri: &str) -> &mut Self {
        self.stc.element("manageTo", Namespace::XmpResourceRef).value(uri);
        self
    }

    /// Write the `stRef:maskMarkers` property.
    ///
    /// Whether to process markers for resources in the [`XmpWriter::ingredients`] array.
    pub fn mask_markers(&mut self, markers: MaskMarkers) -> &mut Self {
        self.stc
            .element("maskMarkers", Namespace::XmpResourceRef)
            .value(markers);
        self
    }

    /// Write the `stRef:partMapping` property.
    ///
    /// The name or URI of a mapping function to map `fromPart` to `toPart`.
    pub fn part_mapping(&mut self, mapping: &str) -> &mut Self {
        self.stc
            .element("partMapping", Namespace::XmpResourceRef)
            .value(mapping);
        self
    }

    /// Write the `stRef:renditionClass` property.
    ///
    /// The rendition class of the referenced resource. See
    /// [`XmpWriter::rendition_class`].
    pub fn rendition_class(&mut self, rendition: RenditionClass) -> &mut Self {
        self.stc
            .element("renditionClass", Namespace::XmpResourceRef)
            .value(rendition);
        self
    }

    /// Write the `stRef:renditionParams` property.
    ///
    /// The rendition parameters of the referenced resource. See
    /// [`XmpWriter::rendition_params`].
    pub fn rendition_params(&mut self, params: &str) -> &mut Self {
        self.stc
            .element("renditionParams", Namespace::XmpResourceRef)
            .value(params);
        self
    }

    /// Write the `stRef:toPart` property.
    ///
    /// For a resource in a [`XmpWriter::ingredients`] array, the part of the root
    /// resource that the the ingredient corresponds to.
    pub fn to_part(&mut self, part: &str) -> &mut Self {
        self.stc.element("toPart", Namespace::XmpResourceRef).value(part);
        self
    }

    /// Write the `stRef:versionID` property.
    ///
    /// The referenced resource's version ID. See [`XmpWriter::version_id`].
    pub fn version_id(&mut self, id: &str) -> &mut Self {
        self.stc.element("versionID", Namespace::XmpResourceRef).value(id);
        self
    }
}

deref!('a, 'n, ResourceRefWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Writer for a resource reference array.
///
/// Created by [`XmpWriter::ingredients`].
pub struct ResourceRefsWriter<'a, 'n: 'a> {
    array: Array<'a, 'n>,
}

impl<'a, 'n: 'a> ResourceRefsWriter<'a, 'n> {
    fn start(array: Array<'a, 'n>) -> Self {
        Self { array }
    }

    /// Add a reference to the array.
    pub fn add_ref(&mut self) -> ResourceRefWriter<'_, 'n> {
        ResourceRefWriter::start(self.array.element().obj())
    }
}

deref!('a, 'n, ResourceRefsWriter<'a, 'n> => Array<'a, 'n>, array);

/// Writer for an event that occurred to a resource.
///
/// Created by [`VersionWriter::event`] and [`ResourceEventsWriter::add_event`].
pub struct ResourceEventWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n: 'a> ResourceEventWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `stEvt:action` property.
    ///
    /// The action that occurred to the resource.
    pub fn action(&mut self, action: ResourceEventAction) -> &mut Self {
        self.stc.element("action", Namespace::XmpResourceEvent).value(action);
        self
    }

    /// Write the `stEvt:changed` property.
    ///
    /// Semicolon-separated list of the parts of the resource that changed.
    pub fn changed(&mut self, parts: &str) -> &mut Self {
        self.stc.element("changed", Namespace::XmpResourceEvent).value(parts);
        self
    }
    /// Write the `stEvt:instanceID` property.
    ///
    /// Value of the [`XmpWriter::instance_id`] property at the time of the action.
    pub fn instance_id(&mut self, id: &str) -> &mut Self {
        self.stc.element("instanceID", Namespace::XmpResourceEvent).value(id);
        self
    }

    /// Write the `stEvt:changed` property.
    ///
    /// Additional parameters for the action.
    pub fn parameters(&mut self, params: &str) -> &mut Self {
        self.stc
            .element("parameters", Namespace::XmpResourceEvent)
            .value(params);
        self
    }

    /// Write the `stEvt:softwareAgent` property.
    ///
    /// The name of the software agent that performed the action.
    pub fn software_agent(&mut self, agent: &str) -> &mut Self {
        self.stc
            .element("softwareAgent", Namespace::XmpResourceEvent)
            .value(agent);
        self
    }

    /// Write the `stEvt:when` property.
    ///
    /// The date and time the action occurred.
    pub fn when(&mut self, date: DateTime) -> &mut Self {
        self.stc.element("when", Namespace::XmpResourceEvent).value(date);
        self
    }
}

deref!('a, 'n, ResourceEventWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Writer for a resource event array.
///
/// Created by [`XmpWriter::history`].
pub struct ResourceEventsWriter<'a, 'n: 'a> {
    array: Array<'a, 'n>,
}

impl<'a, 'n: 'a> ResourceEventsWriter<'a, 'n> {
    fn start(array: Array<'a, 'n>) -> Self {
        Self { array }
    }

    /// Add an event to the array.
    pub fn add_event(&mut self) -> ResourceEventWriter<'_, 'n> {
        ResourceEventWriter::start(self.array.element().obj())
    }
}

deref!('a, 'n, ResourceEventsWriter<'a, 'n> => Array<'a, 'n>, array);

/// Writer for an item in a Pantry array.
///
/// Use the `Deref` impl to access the underlying [`Struct`] and add properties.
/// Created by [`PantryWriter::add_item`].
pub struct PantryItemWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n: 'a> PantryItemWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `xmpMM:instanceID` property. Required.
    pub fn instance_id(&mut self, id: &str) -> &mut Self {
        self.stc.element("instanceID", Namespace::XmpMedia).value(id);
        self
    }
}

deref!('a, 'n, PantryItemWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Writer for a Pantry array.
pub struct PantryWriter<'a, 'n: 'a> {
    array: Array<'a, 'n>,
}

impl<'a, 'n: 'a> PantryWriter<'a, 'n> {
    fn start(array: Array<'a, 'n>) -> Self {
        Self { array }
    }

    /// Add an item to the array.
    pub fn add_item(&mut self) -> PantryItemWriter<'_, 'n> {
        PantryItemWriter::start(self.array.element().obj())
    }
}

deref!('a, 'n, PantryWriter<'a, 'n> => Array<'a, 'n>, array);

/// Writer for a version struct.
///
/// Created by [`VersionsWriter::add_version`].
pub struct VersionWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n: 'a> VersionWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `stVer:comments` property.
    ///
    /// Comments about the version.
    pub fn comments(&mut self, comments: &str) -> &mut Self {
        self.stc.element("comments", Namespace::XmpVersion).value(comments);
        self
    }

    /// Start writing the `stVer:event` property.
    ///
    /// The event that created the version.
    pub fn event(&mut self) -> ResourceEventWriter<'_, 'n> {
        ResourceEventWriter::start(self.stc.element("event", Namespace::XmpVersion).obj())
    }

    /// Write the `stVer:modifier` property.
    ///
    /// The person or organization that created the version.
    pub fn modifier(&mut self, modifier: &str) -> &mut Self {
        self.stc.element("modifier", Namespace::XmpVersion).value(modifier);
        self
    }

    /// Write the `stVer:modifyDate` property.
    ///
    /// The date and time the version was created.
    pub fn modify_date(&mut self, date: DateTime) -> &mut Self {
        self.stc.element("modifyDate", Namespace::XmpVersion).value(date);
        self
    }

    /// Write the `stVer:version` property.
    ///
    /// The new version number.
    pub fn version(&mut self, version: &str) -> &mut Self {
        self.stc.element("version", Namespace::XmpVersion).value(version);
        self
    }
}

deref!('a, 'n, VersionWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Writer for a versions array.
///
/// Created by [`XmpWriter::version_ref`].
pub struct VersionsWriter<'a, 'n: 'a> {
    array: Array<'a, 'n>,
}

impl<'a, 'n: 'a> VersionsWriter<'a, 'n> {
    fn start(array: Array<'a, 'n>) -> Self {
        Self { array }
    }

    /// Add a version to the array.
    pub fn add_version(&mut self) -> VersionWriter<'_, 'n> {
        VersionWriter::start(self.array.element().obj())
    }
}

deref!('a, 'n, VersionsWriter<'a, 'n> => Array<'a, 'n>, array);

/// Writer for a job struct.
///
/// Created by [`JobsWriter::add_job`].
pub struct JobWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n: 'a> JobWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `stJob:id` property.
    ///
    /// The unique identifier for the job.
    pub fn id(&mut self, id: &str) -> &mut Self {
        self.stc.element("id", Namespace::XmpJob).value(id);
        self
    }

    /// Write the `stJob:name` property.
    ///
    /// The name of the job.
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.stc.element("name", Namespace::XmpJob).value(name);
        self
    }

    /// Write the `stJob:url` property.
    ///
    /// Reference an external job management file.
    pub fn url(&mut self, url: &str) -> &mut Self {
        self.stc.element("url", Namespace::XmpJob).value(url);
        self
    }
}

deref!('a, 'n, JobWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Writer for a job array.
///
/// Created by [`XmpWriter::jobs`].
pub struct JobsWriter<'a, 'n: 'a> {
    array: Array<'a, 'n>,
}

impl<'a, 'n: 'a> JobsWriter<'a, 'n> {
    fn start(array: Array<'a, 'n>) -> Self {
        Self { array }
    }

    /// Add a job to the array.
    pub fn add_job(&mut self) -> JobWriter<'_, 'n> {
        JobWriter::start(self.array.element().obj())
    }
}

deref!('a, 'n, JobsWriter<'a, 'n> => Array<'a, 'n>, array);

/// A writer for colorant structs.
///
/// Created by [`ColorantsWriter::add_colorant`].
pub struct ColorantWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n: 'a> ColorantWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `xmpG:type` property.
    ///
    /// Whether this is a spot color or a process color.
    pub fn type_(&mut self, kind: ColorantType) -> &mut Self {
        self.stc.element("type", Namespace::XmpColorant).value(kind);
        self
    }

    /// Write the `xmpG:swatchName` property.
    ///
    /// The name of the colorant.
    pub fn swatch_name(&mut self, name: &str) -> &mut Self {
        self.stc.element("swatchName", Namespace::XmpColorant).value(name);
        self
    }

    /// Write the `xmpG:colorantMode` property.
    ///
    /// In which color space this colorant is defined.
    pub fn colorant_mode(&mut self, mode: ColorantMode) -> &mut Self {
        self.stc.element("colorantMode", Namespace::XmpColorant).value(mode);
        self
    }

    /// Write the `xmpG:colorantType` property.
    ///
    /// The `L` value of a colorant with `xmpG:colorantMode` set to `Lab`.
    pub fn l(&mut self, l: f64) -> &mut Self {
        self.stc.element("L", Namespace::XmpColorant).value(l);
        self
    }

    /// Write the `xmpG:a` property.
    ///     
    /// The `a` value of a colorant with `xmpG:colorantMode` set to `Lab`.
    pub fn a(&mut self, a: i32) -> &mut Self {
        self.stc.element("A", Namespace::XmpColorant).value(a);
        self
    }

    /// Write the `xmpG:b` property.
    ///
    /// The `b` value of a colorant with `xmpG:colorantMode` set to `Lab`.
    pub fn b(&mut self, b: i32) -> &mut Self {
        self.stc.element("B", Namespace::XmpColorant).value(b);
        self
    }

    /// Write the `xmpG:black` property.
    ///
    /// The `K` value of a colorant with `xmpG:colorantMode` set to `CMYK`.
    pub fn black(&mut self, black: f64) -> &mut Self {
        self.stc.element("black", Namespace::XmpColorant).value(black);
        self
    }

    /// Write the `xmpG:cyan` property.
    ///
    /// The `C` value of a colorant with `xmpG:colorantMode` set to `CMYK`.
    pub fn cyan(&mut self, cyan: f64) -> &mut Self {
        self.stc.element("cyan", Namespace::XmpColorant).value(cyan);
        self
    }

    /// Write the `xmpG:magenta` property.
    ///
    /// The `M` value of a colorant with `xmpG:colorantMode` set to `CMYK`.
    pub fn magenta(&mut self, magenta: f64) -> &mut Self {
        self.stc.element("magenta", Namespace::XmpColorant).value(magenta);
        self
    }

    /// Write the `xmpG:yellow` property.
    ///
    /// The `Y` value of a colorant with `xmpG:colorantMode` set to `CMYK`.
    pub fn yellow(&mut self, yellow: f64) -> &mut Self {
        self.stc.element("yellow", Namespace::XmpColorant).value(yellow);
        self
    }

    /// Write the `xmpG:red` property.
    ///
    /// The `R` value of a colorant with `xmpG:colorantMode` set to `RGB`.
    pub fn red(&mut self, red: i32) -> &mut Self {
        self.stc.element("red", Namespace::XmpColorant).value(red);
        self
    }

    /// Write the `xmpG:green` property.
    ///
    /// The `G` value of a colorant with `xmpG:colorantMode` set to `RGB`.
    pub fn green(&mut self, green: i32) -> &mut Self {
        self.stc.element("green", Namespace::XmpColorant).value(green);
        self
    }

    /// Write the `xmpG:blue` property.
    ///
    /// The `B` value of a colorant with `xmpG:colorantMode` set to `RGB`.
    pub fn blue(&mut self, blue: i32) -> &mut Self {
        self.stc.element("blue", Namespace::XmpColorant).value(blue);
        self
    }
}

deref!('a, 'n, ColorantWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Writer for an array of colorants.
///
/// Created by [`XmpWriter::colorants`].
pub struct ColorantsWriter<'a, 'n: 'a> {
    array: Array<'a, 'n>,
}

impl<'a, 'n> ColorantsWriter<'a, 'n> {
    fn start(array: Array<'a, 'n>) -> Self {
        Self { array }
    }

    /// Add a new colorant to the array.
    pub fn add_colorant(&mut self) -> ColorantWriter<'_, 'n> {
        ColorantWriter::start(self.array.element().obj())
    }
}

deref!('a, 'n, ColorantsWriter<'a, 'n> => Array<'a, 'n>, array);

/// Writer for a dimensions struct.
///
/// Created by [`XmpWriter::max_page_size`].
pub struct DimensionsWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n> DimensionsWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `stDim:w` property.
    ///
    /// The width of the resource.
    pub fn width(&mut self, width: f64) -> &mut Self {
        self.stc.element("w", Namespace::XmpDimensions).value(width);
        self
    }

    /// Write the `stDim:h` property.
    ///
    /// The height of the resource.
    pub fn height(&mut self, height: f64) -> &mut Self {
        self.stc.element("h", Namespace::XmpDimensions).value(height);
        self
    }

    /// Write the `stDim:unit` property.
    ///
    /// The unit of the width and height properties.
    pub fn unit(&mut self, unit: DimensionUnit) -> &mut Self {
        self.stc.element("unit", Namespace::XmpDimensions).value(unit);
        self
    }
}

deref!('a, 'n, DimensionsWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Writer for a font struct.
///
/// Created by [`XmpWriter::fonts`].
pub struct FontWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n: 'a> FontWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `stFnt:childFontFiles` property.
    ///
    /// An array of font files that make up this font.
    pub fn child_font_files<'b>(
        &mut self,
        files: impl IntoIterator<Item = &'b str>,
    ) -> &mut Self {
        self.stc
            .element("childFontFiles", Namespace::XmpFont)
            .ordered_array(files);
        self
    }

    /// Write the `stFnt:composite` property.
    ///
    /// Whether the font is a composite font.
    pub fn composite(&mut self, composite: bool) -> &mut Self {
        self.stc.element("composite", Namespace::XmpFont).value(composite);
        self
    }

    /// Write the `stFnt:fontFace` property.
    ///
    /// The font face name.
    pub fn font_face(&mut self, face: &str) -> &mut Self {
        self.stc.element("fontFace", Namespace::XmpFont).value(face);
        self
    }

    /// Write the `stFnt:fontFamily` property.
    ///
    /// The font family name.
    pub fn font_family(&mut self, family: &str) -> &mut Self {
        self.stc.element("fontFamily", Namespace::XmpFont).value(family);
        self
    }

    /// Write the `stFnt:fontFile` property.
    ///
    /// The font file name.
    pub fn font_file(&mut self, file_name: &str) -> &mut Self {
        self.stc.element("fontFileName", Namespace::XmpFont).value(file_name);
        self
    }

    /// Write the `stFnt:fontName` property.
    ///
    /// The PostScript name of the font.
    pub fn font_name(&mut self, name: &str) -> &mut Self {
        self.stc.element("fontName", Namespace::XmpFont).value(name);
        self
    }

    /// Write the `stFnt:fontType` property.
    ///
    /// The font type.
    pub fn font_type(&mut self, font_type: FontType) -> &mut Self {
        self.stc.element("fontType", Namespace::XmpFont).value(font_type);
        self
    }

    /// Write the `stFnt:versionString` property.
    ///
    /// The version string of the font.
    /// Must be chosen depending on the font type.
    /// - **Type 1**: `/version`
    /// - **TrueType / OpenType**: `nameId 5`
    /// - **CID**: `/CIDFontVersion`
    /// - **Bitmap Font**: must be empty
    pub fn version_string(&mut self, version: &str) -> &mut Self {
        self.stc.element("versionString", Namespace::XmpFont).value(version);
        self
    }
}

deref!('a, 'n, FontWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Writer for an array of fonts.
///
/// Created by [`XmpWriter::fonts`].
pub struct FontsWriter<'a, 'n: 'a> {
    array: Array<'a, 'n>,
}

impl<'a, 'n: 'a> FontsWriter<'a, 'n> {
    fn start(array: Array<'a, 'n>) -> Self {
        Self { array }
    }

    /// Add a new font to the array.
    pub fn add_font(&mut self) -> FontWriter<'_, 'n> {
        FontWriter::start(self.array.element().obj())
    }
}

deref!('a, 'n, FontsWriter<'a, 'n> => Array<'a, 'n>, array);
