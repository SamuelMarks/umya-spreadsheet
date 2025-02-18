// scheme
use super::FontSchemeValues;
use super::EnumValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct FontScheme {
    pub(crate) val: EnumValue<FontSchemeValues>,
}
impl FontScheme {
    pub fn get_val(&self)-> &FontSchemeValues {
        &self.val.get_value()
    }

    pub fn set_val(&mut self, value:FontSchemeValues)-> &mut Self {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader:&mut Reader<R>,
        e:&BytesStart
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // scheme
        if self.val.has_value() == true {
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            attributes.push(("val", &self.val.get_value_string()));
            write_start_tag(writer, "scheme", attributes, true);
        }
    }
}
