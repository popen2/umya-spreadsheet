// a:camera
use super::super::EnumValue;
use super::PresetCameraValues;
use super::Rotation;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Camera {
    preset: EnumValue<PresetCameraValues>,
    rotation: Option<Rotation>,
}
impl Camera {
    pub fn get_preset(&self) -> &PresetCameraValues {
        self.preset.get_value()
    }

    pub fn set_preset(&mut self, value: PresetCameraValues) -> &mut Self {
        self.preset.set_value(value);
        self
    }

    pub fn get_rotation(&self) -> &Option<Rotation> {
        &self.rotation
    }

    pub fn get_rotation_mut(&mut self) -> &mut Option<Rotation> {
        &mut self.rotation
    }

    pub fn set_rotation(&mut self, value: Rotation) -> &mut Self {
        self.rotation = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        match get_attribute(e, b"prst") {
            Some(v) => {
                self.preset.set_value_string(v);
            }
            None => {}
        }

        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:rot" => {
                        let mut obj = Rotation::default();
                        obj.set_attributes(reader, e);
                        self.rotation = Some(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:camera" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:camera"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let with_inner = self.rotation.is_some();
        // a:camera
        write_start_tag(
            writer,
            "a:camera",
            vec![("prst", self.preset.get_value_string())],
            !with_inner,
        );

        if with_inner {
            // a:rot
            match &self.rotation {
                Some(v) => {
                    v.write_to(writer);
                },
                _ => {}
            }
            write_end_tag(writer, "a:camera");
        }
    }
}
