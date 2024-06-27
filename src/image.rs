use crate::Color;

pub trait ImgWriter {
    fn write(
        &self,
        file_name: &str,
        width: u32,
        height: u32,
        data: &[Color],
    ) -> std::io::Result<()>;
}

pub struct Image {
    img_width: u32,
    img_height: u32,
    aspect_ratio: f64,
    image_data: Vec<Color>,
}

impl Image {
    pub fn new(img_width: u32, aspect_ratio: f64) -> Self {
        let mut img_height = (img_width as f64 / aspect_ratio) as u32;
        img_height = if img_height < 1 { 1 } else { img_height };

        let image_data = vec![Color::default(); (img_height * img_width) as usize];

        Self {
            img_width,
            img_height,
            aspect_ratio,
            image_data,
        }
    }

    pub fn set_px(&mut self, i: u32, j: u32, data: Color) {
        self.image_data[(j * self.img_width + i) as usize] = data;
    }

    pub fn get_px(&mut self, i: u32, j: u32) -> Color {
        self.image_data[(j * self.img_width + i) as usize]
    }

    pub fn img_width(&self) -> u32 {
        self.img_width
    }

    pub fn img_height(&self) -> u32 {
        self.img_height
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn size(&self) -> (u32, u32) {
        (self.img_width, self.img_height)
    }

    pub fn write(&self, file_name: &str, writer: impl ImgWriter) -> std::io::Result<()> {
        writer.write(file_name, self.img_width, self.img_height, &self.image_data)
    }
}
