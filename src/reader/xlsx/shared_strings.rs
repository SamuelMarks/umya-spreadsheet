use structs::Spreadsheet;
use structs::SharedStringTable;
use std::{io, result};
use quick_xml::Reader;
use quick_xml::events::{Event};
use super::XlsxError;

const FILE_PATH: &'static str = "xl/sharedStrings.xml";

pub(crate) fn read<R: io::Read + io::Seek>(arv: &mut zip::ZipArchive<R>, spreadsheet: &mut Spreadsheet) -> result::Result<(), XlsxError> {
    let r = io::BufReader::new(match arv.by_name(FILE_PATH) {
        Ok(v) => v,
        Err(zip::result::ZipError::FileNotFound) => {return Ok(());},
        Err(e) => {return Err(e.into());}
    });
    let mut reader = Reader::from_reader(r);
    reader.trim_text(false);
    let mut buf = Vec::new();

    let theme = spreadsheet.get_theme().clone();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"sst" => {
                        let mut obj = SharedStringTable::default();
                        obj.set_attributes(&mut reader, e);

                        // set ThemeColor
                        for item in obj.get_shared_string_item_mut() {
                            match item.get_rich_text_mut() {
                                Some(v) => {
                                    for element in  v.get_rich_text_elements_mut() {
                                        match element.get_run_properties_crate() {
                                            Some(r) => {
                                                let color = r.get_color_mut();
                                                color.set_argb_by_theme(&theme);
                                            },
                                            None => {},                           
                                        }
                                    }
                                },
                                None => {},                           
                            }
                        }

                        spreadsheet.set_shared_string_table(obj);
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok(())
}
