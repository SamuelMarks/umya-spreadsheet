// patternFill
use super::Color;
use super::EnumValue;
use super::PatternValues;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Default, Debug, Clone)]
pub struct PatternFill {
    pub(crate) pattern_type: EnumValue<PatternValues>,
    foreground_color: Option<Color>,
    background_color: Option<Color>,
}
impl PatternFill {
    pub fn get_pattern_type(&self) -> &PatternValues {
        &self.pattern_type.get_value()
    }

    pub fn set_pattern_type(&mut self, value: PatternValues) -> &mut Self {
        self.pattern_type.set_value(value);
        self
    }

    pub fn get_foreground_color(&self) -> &Option<Color> {
        &self.foreground_color
    }

    pub fn get_foreground_color_mut(&mut self) -> &mut Option<Color> {
        &mut self.foreground_color
    }

    pub fn set_foreground_color(&mut self, value: Color) -> &mut Self {
        self.foreground_color = Some(value);
        self
    }

    pub fn get_background_color(&self) -> &Option<Color> {
        &self.background_color
    }

    pub fn get_background_color_mut(&mut self) -> &mut Option<Color> {
        &mut self.background_color
    }

    pub fn set_background_color(&mut self, value: Color) -> &mut Self {
        self.background_color = Some(value);
        self
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::compute(format!(
                "{}{}{}",
                &self.pattern_type.get_value_string(),
                match &self.foreground_color {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
                match &self.background_color {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        match get_attribute(e, b"patternType") {
            Some(v) => {
                self.pattern_type.set_value_string(v);
            }
            None => {}
        }

        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name() {
                    b"fgColor" => {
                        let mut obj = Color::default();
                        obj.set_attributes(reader, e);
                        &mut self.set_foreground_color(obj);
                    }
                    b"bgColor" => {
                        let mut obj = Color::default();
                        obj.set_attributes(reader, e);
                        &mut self.set_background_color(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"patternFill" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "patternFill"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let empty_flag = self.foreground_color.is_none() && self.background_color.is_none();

        // patternFill
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.pattern_type.has_value() == true {
            attributes.push(("patternType", &self.pattern_type.get_value_string()));
        }
        write_start_tag(writer, "patternFill", attributes, empty_flag);

        if empty_flag == false {
            // fgColor
            match &self.foreground_color {
                Some(v) => {
                    v.write_to_fg_color(writer);
                }
                None => {}
            }

            // bgColor
            match &self.background_color {
                Some(v) => {
                    v.write_to_bg_color(writer);
                }
                None => {}
            }

            write_end_tag(writer, "patternFill");
        }
    }
}
