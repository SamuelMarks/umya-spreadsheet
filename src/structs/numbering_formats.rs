// numFmts
use std::collections::HashMap;
use super::NumberingFormat;
use super::Style;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub(crate) struct NumberingFormats {
    numbering_format: HashMap<u32, NumberingFormat>,
}
impl NumberingFormats {
    pub(crate) fn get_numbering_format(&self)-> &HashMap<u32, NumberingFormat> {
        &self.numbering_format
    }

    pub(crate) fn get_numbering_format_mut(&mut self)-> &mut HashMap<u32, NumberingFormat> {
        &mut self.numbering_format
    }

    pub(crate) fn set_numbering_format(&mut self, value:NumberingFormat)-> &mut Self {
        let number_format_id = value.get_number_format_id();
        self.numbering_format.insert(number_format_id.clone(), value);
        self
    }

    pub(crate) fn init_setup(&mut self)-> &mut Self {
        self.get_build_in_formats();
        self
    }

    pub(crate) fn get_build_in_formats(&mut self) {
        for (index, code) in super::numbering_format::FILL_BUILT_IN_FORMAT_CODES.iter() {
             let mut obj = NumberingFormat::default();
             obj
             .set_number_format_id_crate(index.clone())
             .set_format_code_crate(code.clone());
             self.set_numbering_format(obj);
        }
     }
 
     pub(crate) fn set_style(&mut self, style:&Style) -> u32 {
        match style.get_numbering_format() {
            Some(v) => {
                let hash_code = v.get_hash_code();
                let mut id = 175;
                for (index, numbering_format) in &self.numbering_format {
                    if numbering_format.get_hash_code() == hash_code {
                        return index.clone();
                    }
                    if &id < index {
                        id = index.clone();
                    }
                }
                id += 1;
                let mut num = v.clone();
                num.set_number_format_id_crate(id);
                self.set_numbering_format(num);
                return id;
            },
            None => {0}
        }
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"numFmt" => {
                            let mut obj = NumberingFormat::default();
                            obj.set_attributes(reader, e);
                            self.set_numbering_format(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"numFmts" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "numFmts"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let mut has_format = false;
        let mut cnt = 0;
        for (_, numbering_format) in &self.numbering_format {
            if numbering_format.get_is_build_in() == &false {
                has_format = true;
                cnt += 1;
            }
        }

        if has_format {
            // numFmts
            let cnt_str = cnt.to_string();
            write_start_tag(writer, "numFmts", vec![
                ("count", &cnt_str),
            ], false);

            // numFmt
            for (index, numbering_format) in &self.numbering_format {
                if numbering_format.get_is_build_in() == &false {
                    numbering_format.write_to(writer, index);
                }
            }

            write_end_tag(writer, "numFmts");
        }
    }
}
