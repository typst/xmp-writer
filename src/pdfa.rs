//! PDF/A extension schema description.
//!
//! Enabled by the `pdfa` feature (enabled by default).

use crate::{deref, Array, Namespace, RdfCollectionType, Struct};

/// Write a extension schema description.
///
/// Created by [`PdfAExtSchemasWriter::add_schema`].
pub struct PdfAExtSchemaWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n: 'a> PdfAExtSchemaWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `pdfaSchema:schema`, `pdfaSchema:namespaceURI`, and
    /// `pdfaSchema:prefix` properties.
    ///
    /// The values are extracted from the given namespace.
    pub fn namespace(&mut self, namespace: Namespace<'n>) -> &mut Self {
        self.schema(&format!("{} schema", namespace.name()));
        self.namespace_uri(namespace.url());
        self.prefix(namespace.prefix());
        self
    }

    /// Write the `pdfaSchema:schema` property.
    ///
    /// Description of the extension schema.
    fn schema(&mut self, schema: &str) -> &mut Self {
        self.stc.element("schema", Namespace::PdfASchema).value(schema);
        self
    }

    /// Write the `pdfaSchema:namespaceURI` property.
    ///
    /// The namespace URI of the extension schema.
    fn namespace_uri(&mut self, uri: &str) -> &mut Self {
        self.stc.element("namespaceURI", Namespace::PdfASchema).value(uri);
        self
    }

    /// Write the `pdfaSchema:prefix` property.
    ///
    /// The prefix of the extension schema.
    fn prefix(&mut self, prefix: &str) -> &mut Self {
        self.stc.element("prefix", Namespace::PdfASchema).value(prefix);
        self
    }

    /// Start writing the `pdfaSchema:property` sequence.
    ///
    /// Describes the properties of the extension schema.
    pub fn properties(&mut self) -> PdfAExtPropertiesWriter<'_, 'n> {
        PdfAExtPropertiesWriter::start(
            self.stc
                .element("property", Namespace::PdfASchema)
                .array(RdfCollectionType::Seq),
        )
    }

    /// Start writing the `pdfaSchema:valueType` sequence.
    ///
    /// Describes the value types of the extension schema.
    pub fn value_types(&mut self) -> PdfAExtTypesWriter<'_, 'n> {
        PdfAExtTypesWriter::start(
            self.stc
                .element("valueType", Namespace::PdfASchema)
                .array(RdfCollectionType::Seq),
        )
    }
}

deref!('a, 'n, PdfAExtSchemaWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Write a property of an extension schema.
///
/// Created by [`PdfAExtPropertiesWriter::add_property`].
pub struct PdfAExtPropertyWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n: 'a> PdfAExtPropertyWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `pdfaProperty:name` property.
    ///
    /// Name of the property.
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.stc.element("name", Namespace::PdfAProperty).value(name);
        self
    }

    /// Write the `pdfaProperty:valueType` property.
    ///
    /// The value type of the property. Shall either be defined in the XMP
    /// specification or in the extension schema.
    pub fn value_type(&mut self, value_type: &str) -> &mut Self {
        self.stc
            .element("valueType", Namespace::PdfAProperty)
            .value(value_type);
        self
    }

    /// Write the `pdfaProperty:category` property.
    ///
    /// Whether the property is generated through the document's contents
    /// (`internal` is true) or input by the user (`internal` is false).
    pub fn category(&mut self, internal: bool) -> &mut Self {
        self.stc
            .element("category", Namespace::PdfAProperty)
            .value(if internal { "internal" } else { "external" });
        self
    }

    /// Write the `pdfaProperty:description` property.
    ///
    /// Description of the property.
    pub fn description(&mut self, description: &str) -> &mut Self {
        self.stc
            .element("description", Namespace::PdfAProperty)
            .value(description);
        self
    }
}

deref!('a, 'n, PdfAExtPropertyWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Write a value type of an extension schema.
///
/// Created by [`PdfAExtTypesWriter::add_value_type`].
pub struct PdfAExtTypeWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n: 'a> PdfAExtTypeWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `pdfaType:type` property.
    ///
    /// The name of the value type.
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.stc.element("type", Namespace::PdfAType).value(name);
        self
    }

    /// Write the `pdfaType:namespaceURI` and `pdfaType:prefix` properties.
    pub fn namespace(&mut self, namespace: Namespace<'n>) -> &mut Self {
        self.namespace_uri(namespace.url());
        self.prefix(namespace.prefix());
        self
    }

    /// Write the `pdfaType:namespaceURI` property.
    ///
    /// The namespace URI of the value type. Consider calling
    /// [`Self::namespace`] instead.
    pub fn namespace_uri(&mut self, uri: &str) -> &mut Self {
        self.stc.element("namespaceURI", Namespace::PdfAType).value(uri);
        self
    }

    /// Write the `pdfaType:prefix` property.
    ///
    /// The prefix of the value type. Consider calling [`Self::namespace`]
    /// instead.
    pub fn prefix(&mut self, prefix: &str) -> &mut Self {
        self.stc.element("prefix", Namespace::PdfAType).value(prefix);
        self
    }

    /// Write the `pdfaType:description` property.
    ///
    /// Human-readable description of the value type.
    pub fn description(&mut self, description: &str) -> &mut Self {
        self.stc
            .element("description", Namespace::PdfAType)
            .value(description);
        self
    }

    /// Start writing the `pdfaType:field` sequence.
    ///
    /// Describes the fields of the value type.
    pub fn fields(&mut self) -> PdfAExtTypeFieldsWriter<'_, 'n> {
        PdfAExtTypeFieldsWriter::start(
            self.stc
                .element("field", Namespace::PdfAType)
                .array(RdfCollectionType::Seq),
        )
    }
}

deref!('a, 'n, PdfAExtTypeWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Write a field of an extension schema value type.
///
/// Created by [`PdfAExtTypeFieldsWriter::add_field`].
pub struct PdfAExtTypeFieldWriter<'a, 'n: 'a> {
    stc: Struct<'a, 'n>,
}

impl<'a, 'n: 'a> PdfAExtTypeFieldWriter<'a, 'n> {
    fn start(stc: Struct<'a, 'n>) -> Self {
        Self { stc }
    }

    /// Write the `pdfaField:name` property.
    ///
    /// Name of the field in the type.
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.stc.element("name", Namespace::PdfAField).value(name);
        self
    }

    /// Write the `pdfaField:valueType` property.
    ///
    /// The value type of the field. Shall either be defined in the XMP
    /// specification or in the extension schema.
    pub fn value_type(&mut self, value_type: &str) -> &mut Self {
        self.stc.element("valueType", Namespace::PdfAField).value(value_type);
        self
    }

    /// Write the `pdfaField:category` property.
    ///
    /// Human-readable description of the field in the type.
    pub fn description(&mut self, description: &str) -> &mut Self {
        self.stc
            .element("description", Namespace::PdfAField)
            .value(description);
        self
    }
}

deref!('a, 'n, PdfAExtTypeFieldWriter<'a, 'n> => Struct<'a, 'n>, stc);

/// Write an array of extension schema value type fields.
///
/// Created by [`PdfAExtTypeWriter::fields`].
pub struct PdfAExtTypeFieldsWriter<'a, 'n: 'a> {
    array: Array<'a, 'n>,
}

impl<'a, 'n: 'a> PdfAExtTypeFieldsWriter<'a, 'n> {
    fn start(array: Array<'a, 'n>) -> Self {
        Self { array }
    }

    /// Start writing a field.
    pub fn add_field(&mut self) -> PdfAExtTypeFieldWriter<'_, 'n> {
        PdfAExtTypeFieldWriter::start(self.array.element().obj())
    }
}

deref!('a, 'n, PdfAExtTypeFieldsWriter<'a, 'n> => Array<'a, 'n>, array);

/// Write an array of extension schema properties.
///
/// Created by [`PdfAExtSchemaWriter::properties`].
pub struct PdfAExtPropertiesWriter<'a, 'n: 'a> {
    array: Array<'a, 'n>,
}

impl<'a, 'n: 'a> PdfAExtPropertiesWriter<'a, 'n> {
    fn start(array: Array<'a, 'n>) -> Self {
        Self { array }
    }

    /// Add a property.
    pub fn add_property(&mut self) -> PdfAExtPropertyWriter<'_, 'n> {
        PdfAExtPropertyWriter::start(self.array.element().obj())
    }
}

deref!('a, 'n, PdfAExtPropertiesWriter<'a, 'n> => Array<'a, 'n>, array);

/// Write an array of extension schema value types.
///
/// Created by [`PdfAExtSchemaWriter::value_types`].
pub struct PdfAExtTypesWriter<'a, 'n: 'a> {
    array: Array<'a, 'n>,
}

impl<'a, 'n: 'a> PdfAExtTypesWriter<'a, 'n> {
    fn start(array: Array<'a, 'n>) -> Self {
        Self { array }
    }

    /// Start writing a value type.
    pub fn add_value_type(&mut self) -> PdfAExtTypeWriter<'_, 'n> {
        PdfAExtTypeWriter::start(self.array.element().obj())
    }
}

/// Write a set of extension schema descriptions.
///
/// Created by [`crate::XmpWriter::extension_schemas`]. Check PDF/A-1 TechNote
/// 0008 to learn which schemas and properties need to be described.
pub struct PdfAExtSchemasWriter<'a, 'n: 'a> {
    array: Array<'a, 'n>,
}

impl<'a, 'n: 'a> PdfAExtSchemasWriter<'a, 'n> {
    pub(crate) fn start(array: Array<'a, 'n>) -> Self {
        Self { array }
    }

    /// Start writing a schema.
    pub fn add_schema(&mut self) -> PdfAExtSchemaWriter<'_, 'n> {
        PdfAExtSchemaWriter::start(self.array.element().obj())
    }

    /// Describe the `pdfaid` schema.
    ///
    /// If `corrigendum` is true, the `pdfaid:corr` property is added.
    pub fn pdfaid(&mut self, corrigendum: bool) -> &mut Self {
        {
            let mut schema = self.add_schema();
            schema.namespace(Namespace::PdfAId);
            let mut properties = schema.properties();

            properties
                .add_property()
                .category(true)
                .description("Part of PDF/A standard")
                .name("part")
                .value_type("Integer");

            properties
                .add_property()
                .category(true)
                .description("Amendment of PDF/A standard")
                .name("amd")
                .value_type("Text");

            if corrigendum {
                properties
                    .add_property()
                    .category(true)
                    .description("Corrigendum of PDF/A standard")
                    .name("corr")
                    .value_type("Text");
            }

            properties
                .add_property()
                .category(true)
                .description("Conformance level of PDF/A standard")
                .name("conformance")
                .value_type("Text");
        }
        self
    }

    /// Start describing the `pdf` schema.
    pub fn pdf(&mut self) -> AdobePdfDescsWriter<'_, 'n> {
        AdobePdfDescsWriter::start(self.add_schema())
    }

    /// Start describing the `xmp` schema.
    pub fn xmp(&mut self) -> XmpDescsWriter<'_, 'n> {
        XmpDescsWriter::start(self.add_schema())
    }

    /// Start describing the `xmpMM` schema.
    pub fn xmp_media_management(&mut self) -> XmpMMDescsWriter<'_, 'n> {
        XmpMMDescsWriter::start(self.add_schema())
    }
}

deref!('a, 'n, PdfAExtSchemasWriter<'a, 'n> => Array<'a, 'n>, array);

/// Writer for the property descriptions of the `xmp` schema.
///
/// Only contains methods for properties that are defined in XMP 2005 or later.
/// Created by [`XmpDescsWriter::properties`].
pub struct XmpPropertiesWriter<'a, 'n: 'a> {
    props: PdfAExtPropertiesWriter<'a, 'n>,
}

impl<'a, 'n: 'a> XmpPropertiesWriter<'a, 'n> {
    fn start(props: PdfAExtPropertiesWriter<'a, 'n>) -> Self {
        Self { props }
    }

    /// Describe the `xmp:Label` property.
    pub fn describe_label(&mut self) -> &mut Self {
        self.add_property()
            .category(false)
            .description("A user-defined label for the resource")
            .name("Label")
            .value_type("Text");
        self
    }

    /// Describe the `xmp:Rating` property.
    pub fn describe_rating(&mut self) -> &mut Self {
        self.add_property()
            .category(false)
            .description("A user-assigned rating of the resource")
            .name("Rating")
            .value_type("Integer");
        self
    }
}

deref!('a, 'n, XmpPropertiesWriter<'a, 'n> => PdfAExtPropertiesWriter<'a, 'n>, props);

/// Writer for describing the XMP schema.
///
/// Created by [`PdfAExtSchemasWriter::xmp`].
pub struct XmpDescsWriter<'a, 'n: 'a> {
    schema: PdfAExtSchemaWriter<'a, 'n>,
}

impl<'a, 'n: 'a> XmpDescsWriter<'a, 'n> {
    fn start(mut schema: PdfAExtSchemaWriter<'a, 'n>) -> Self {
        schema.namespace(Namespace::Xmp);
        Self { schema }
    }

    /// Start describing the properties of the `xmp` schema.
    pub fn properties(&mut self) -> XmpPropertiesWriter<'_, 'n> {
        XmpPropertiesWriter::start(self.schema.properties())
    }
}

deref!('a, 'n, XmpDescsWriter<'a, 'n> => PdfAExtSchemaWriter<'a, 'n>, schema);

/// Writer for the property descriptions of the `xmpMM` schema.
///
/// Created by [`XmpMMDescsWriter::properties`].
pub struct XmpMMPropertiesWriter<'a, 'n: 'a> {
    props: PdfAExtPropertiesWriter<'a, 'n>,
}

impl<'a, 'n: 'a> XmpMMPropertiesWriter<'a, 'n> {
    fn start(props: PdfAExtPropertiesWriter<'a, 'n>) -> Self {
        Self { props }
    }

    /// Describe the `xmpMM:InstanceID` property.
    pub fn describe_instance_id(&mut self) -> &mut Self {
        self.add_property()
            .category(true)
            .description("UUID based identifier for specific incarnation of a document")
            .name("InstanceID")
            .value_type("Text");
        self
    }

    /// Describe the `xmpMM:Ingredients` property.
    pub fn describe_ingredients(&mut self) -> &mut Self {
        self.add_property()
            .category(true)
            .description("List of ingredients that were used to create a document")
            .name("Ingredients")
            .value_type("ResourceRef");
        self
    }

    /// Describe the `xmpMM:OriginalDocumentID` property.
    pub fn describe_original_doc_id(&mut self) -> &mut Self {
        self.add_property()
            .category(true)
            .description("UUID based identifier for original document from which a document is derived")
            .name("OriginalDocumentID")
            .value_type("Text");
        self
    }

    /// Describe the `xmpMM:Pantry` property.
    pub fn describe_pantry(&mut self) -> &mut Self {
        self.add_property()
            .category(true)
            .description("List of ingredients that were used to create a document")
            .name("Pantry")
            .value_type("ResourceRef");
        self
    }
}

deref!('a, 'n, XmpMMPropertiesWriter<'a, 'n> => PdfAExtPropertiesWriter<'a, 'n>, props);

/// Writer for describing the XMP Media Management schema.
///
/// Created by [`PdfAExtSchemasWriter::xmp_media_management`].
pub struct XmpMMDescsWriter<'a, 'n: 'a> {
    schema: PdfAExtSchemaWriter<'a, 'n>,
}

impl<'a, 'n: 'a> XmpMMDescsWriter<'a, 'n> {
    fn start(mut schema: PdfAExtSchemaWriter<'a, 'n>) -> Self {
        schema.namespace(Namespace::XmpMedia);
        Self { schema }
    }

    /// Start describing the properties of the `xmpMM` schema.
    pub fn properties(&mut self) -> XmpMMPropertiesWriter<'_, 'n> {
        XmpMMPropertiesWriter::start(self.schema.properties())
    }
}

deref!('a, 'n, XmpMMDescsWriter<'a, 'n> => PdfAExtSchemaWriter<'a, 'n>, schema);

/// Writer for the property descriptions of the `pdf` schema.
///
/// Created by [`AdobePdfDescsWriter::properties`].
pub struct AdobePdfPropertiesWriter<'a, 'n: 'a> {
    props: PdfAExtPropertiesWriter<'a, 'n>,
}

impl<'a, 'n: 'a> AdobePdfPropertiesWriter<'a, 'n> {
    fn start(props: PdfAExtPropertiesWriter<'a, 'n>) -> Self {
        Self { props }
    }

    /// Describe the `pdf:Keywords` property.
    ///
    /// Optional even if present, see PDF/A-1 TechNote 0008.
    pub fn describe_keywords(&mut self) -> &mut Self {
        self.add_property()
            .category(false)
            .description("Keywords associated with the document")
            .name("Keywords")
            .value_type("Text");
        self
    }

    /// Describe the `pdf:PDFVersion` property.
    ///
    /// Optional even if present, see PDF/A-1 TechNote 0008.
    pub fn describe_pdf_version(&mut self) -> &mut Self {
        self.add_property()
            .category(true)
            .description(
                "Version of the PDF specification to which the document conforms",
            )
            .name("PDFVersion")
            .value_type("Text");
        self
    }

    /// Describe the `pdf:Producer` property.
    ///
    /// Optional even if present, see PDF/A-1 TechNote 0008.
    pub fn describe_producer(&mut self) -> &mut Self {
        self.add_property()
            .category(true)
            .description("Name of the application that created the PDF document")
            .name("Producer")
            .value_type("Text");
        self
    }

    /// Describe the `pdf:Trapped` property.
    pub fn describe_trapped(&mut self) -> &mut Self {
        self.add_property()
            .category(true)
            .description("Whether the document has been trapped")
            .name("Trapped")
            .value_type("Text");
        self
    }
}

deref!('a, 'n, AdobePdfPropertiesWriter<'a, 'n> => PdfAExtPropertiesWriter<'a, 'n>, props);

/// Writer for describing the Adobe PDF extension schema.
///
/// Created by [`PdfAExtSchemasWriter::pdf`].
pub struct AdobePdfDescsWriter<'a, 'n: 'a> {
    schema: PdfAExtSchemaWriter<'a, 'n>,
}

impl<'a, 'n: 'a> AdobePdfDescsWriter<'a, 'n> {
    fn start(mut schema: PdfAExtSchemaWriter<'a, 'n>) -> Self {
        schema.namespace(Namespace::AdobePdf);
        Self { schema }
    }

    /// Start describing the properties of the `pdf` schema.
    pub fn properties(&mut self) -> AdobePdfPropertiesWriter<'_, 'n> {
        AdobePdfPropertiesWriter::start(self.schema.properties())
    }
}

deref!('a, 'n, AdobePdfDescsWriter<'a, 'n> => PdfAExtSchemaWriter<'a, 'n>, schema);
