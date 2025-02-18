// c:layout 
use super::ManualLayout;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Layout {
    manual_layout: Option<ManualLayout>,
}
impl Layout {
    pub fn get_manual_layout(&self)-> &Option<ManualLayout> {
        &self.manual_layout
    }

    pub fn get_manual_layout_mut(&mut self)-> &mut Option<ManualLayout> {
        &mut self.manual_layout
    }

    pub fn set_manual_layout(&mut self, value:ManualLayout)-> &mut Layout {
        self.manual_layout = Some(value);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.manual_layout.is_none()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart,
        empty_flag:bool,
    ) {
        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"c:manualLayout" => {
                            let mut obj = ManualLayout::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_manual_layout(obj);
                        }
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:layout" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:layout"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.is_empty() {
            // c:layout
            write_start_tag(writer, "c:layout", vec![], true);

        } else {
            // c:layout
            write_start_tag(writer, "c:layout", vec![], false);

            // c:manualLayout
            match &self.manual_layout {
                Some(v) => {v.write_to(writer);},
                None => {}
            }

            write_end_tag(writer, "c:layout");
        }
    }
}
