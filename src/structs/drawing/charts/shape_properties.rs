// c:spPr
use super::super::PatternFill;
use super::super::Transform2D;
use super::super::PresetGeometry;
use super::super::SolidFill;
use super::super::Outline;
use super::super::EffectList;
use super::super::Scene3DType;
use super::super::Shape3DType;
use writer::driver::*;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct ShapeProperties {
    pattern_fill: Option<PatternFill>,
    transform2d: Option<Transform2D>,
    preset_geometry: Option<PresetGeometry>,
    solid_fill: Option<SolidFill>,
    outline: Option<Outline>,
    effect_list: Option<EffectList>,
    scene_3d_type: Option<Scene3DType>,
    shape_3d_type: Option<Shape3DType>,
}
impl ShapeProperties {
    pub fn get_pattern_fill(&self) -> &Option<PatternFill> {
        &self.pattern_fill
    }

    pub fn get_pattern_fill_mut(&mut self) -> &mut Option<PatternFill> {
        &mut self.pattern_fill
    }

    pub fn set_pattern_fill(&mut self, value:PatternFill) -> &mut ShapeProperties {
        self.pattern_fill = Some(value);
        self
    }

    pub fn get_transform2d(&self) -> &Option<Transform2D> {
        &self.transform2d
    }

    pub fn get_transform2d_mut(&mut self) -> &mut Option<Transform2D> {
        &mut self.transform2d
    }

    pub fn set_transform2d(&mut self, value:Transform2D) -> &mut ShapeProperties {
        self.transform2d = Some(value);
        self
    }

    pub fn get_geometry(&self) -> &Option<PresetGeometry> {
        &self.preset_geometry
    }

    pub fn get_geometry_mut(&mut self) -> &mut Option<PresetGeometry> {
        &mut self.preset_geometry
    }

    pub fn set_geometry(&mut self, value:PresetGeometry) -> &mut ShapeProperties {
        self.preset_geometry = Some(value);
        self
    }

    pub fn get_solid_fill(&self) -> &Option<SolidFill> {
        &self.solid_fill
    }

    pub fn get_solid_fill_mut(&mut self) -> &mut Option<SolidFill> {
        &mut self.solid_fill
    }

    pub fn set_solid_fill(&mut self, value:SolidFill) -> &mut ShapeProperties {
        self.solid_fill = Some(value);
        self
    }

    pub fn get_outline(&self) -> &Option<Outline> {
        &self.outline
    }

    pub fn get_outline_mut(&mut self) -> &mut Option<Outline> {
        &mut self.outline
    }

    pub fn set_outline(&mut self, value:Outline) -> &mut ShapeProperties {
        self.outline = Some(value);
        self
    }

    pub fn get_effect_list(&self) -> &Option<EffectList> {
        &self.effect_list
    }

    pub fn get_effect_list_mut(&mut self) -> &mut Option<EffectList> {
        &mut self.effect_list
    }

    pub fn set_effect_list(&mut self, value:EffectList) -> &mut ShapeProperties {
        self.effect_list = Some(value);
        self
    }

    pub fn get_scene_3d_type(&self) -> &Option<Scene3DType> {
        &self.scene_3d_type
    }

    pub fn get_scene_3d_type_mut(&mut self) -> &mut Option<Scene3DType> {
        &mut self.scene_3d_type
    }

    pub fn set_scene_3d_type(&mut self, value:Scene3DType) -> &mut ShapeProperties {
        self.scene_3d_type = Some(value);
        self
    }

    pub fn get_shape_3d_type(&self) -> &Option<Shape3DType> {
        &self.shape_3d_type
    }

    pub fn get_shape_3d_type_mut(&mut self) -> &mut Option<Shape3DType> {
        &mut self.shape_3d_type
    }

    pub fn set_shape_3d_type(&mut self, value:Shape3DType) -> &mut ShapeProperties {
        self.shape_3d_type = Some(value);
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
                        b"a:pattFill" => {
                            let mut obj = PatternFill::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_pattern_fill(obj);
                        },
                        b"a:xfrm" => {
                            let mut obj = Transform2D::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_transform2d(obj);
                        },
                        b"a:prstGeom" => {
                            let mut obj = PresetGeometry::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_geometry(obj);
                        },
                        b"a:ln" => {
                            let mut obj = Outline::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_outline(obj);
                        },
                        b"a:solidFill" => {
                            let mut obj = SolidFill::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_solid_fill(obj);
                        },
                        b"a:effectLst" => {
                            let mut obj = EffectList::default();
                            obj.set_attributes(reader, e, false);
                            &mut self.set_effect_list(obj);
                        }
                        b"a:scene3d" => {
                            let mut obj = Scene3DType::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_scene_3d_type(obj);
                        }
                        b"a:sp3d" => {
                            let mut obj = Shape3DType::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_shape_3d_type(obj);
                        }
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:spPr" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:spPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:spPr
        write_start_tag(writer, "c:spPr", vec![], false);
        
        // a:pattFill
        match &self.pattern_fill {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }

        // a:xfrm
        match &self.transform2d {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }
    
        // a:prstGeom
        match &self.preset_geometry {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }
    
        // a:solidFill
        match &self.solid_fill {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }
    
        // a:ln
        match &self.outline {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }

        // a:effectLst
        match &self.effect_list {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }

        // a:scene3d
        match &self.scene_3d_type {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }
    
        // a:sp3d
        match &self.shape_3d_type {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }

        write_end_tag(writer, "c:spPr");
    }
}
