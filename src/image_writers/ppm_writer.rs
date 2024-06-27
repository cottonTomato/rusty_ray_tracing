use crate::{Color, ImgWriter};

pub struct PpmWriter;

impl ImgWriter for PpmWriter {
    fn write(
        &self,
        file_name: &str,
        width: u32,
        height: u32,
        data: &[Color],
    ) -> std::io::Result<()> {
        let mut buf = String::new();
        buf.push_str(&format!("P3\n{width} {height}\n255\n"));

        for color in data {
            buf.push_str(&format!("{}\n", color));
        }

        std::fs::write(file_name, buf)
    }
}
