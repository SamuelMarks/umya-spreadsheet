// a:bgClr
use super::SchemeColor;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct BackgroundColor {
    scheme_color: SchemeColor,
}
impl BackgroundColor {
    pub fn get_scheme_color(&self) -> &SchemeColor {
        &self.scheme_color
    }

    pub fn get_scheme_color_mut(&mut self) -> &mut SchemeColor {
        &mut self.scheme_color
    }

    pub fn set_scheme_color(&mut self, value:SchemeColor) -> &mut BackgroundColor {
        self.scheme_color = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"a:schemeClr" => {
                            self.scheme_color.set_attributes(reader, e, false);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"a:schemeClr" => {
                            self.scheme_color.set_attributes(reader, e, true);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:bgClr" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:bgClr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:bgClr
        write_start_tag(writer, "a:bgClr", vec![], false);

        // a:schemeClr
        &self.scheme_color.write_to(writer);

        write_end_tag(writer, "a:bgClr");
    }
}
