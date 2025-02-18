// colors
use super::MruColors;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub(crate) struct Colors {
    mru_colors: MruColors,
}
impl Colors {
    pub(crate) fn get_mru_colors(&self)-> &MruColors {
        &self.mru_colors
    }

    pub(crate) fn get_mru_colors_mut(&mut self)-> &mut MruColors {
        &mut self.mru_colors
    }

    pub(crate) fn set_mru_colors(&mut self, value:MruColors)-> &mut Self {
        self.mru_colors = value;
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
                        b"mruColors" => {
                            self.mru_colors.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"colors" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "colors"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.mru_colors.get_color().len() > 0 {
            // colors
            write_start_tag(writer, "colors", vec![], false);

            // mruColors
            &self.mru_colors.write_to(writer);

            write_end_tag(writer, "colors");
        }
    }
}
