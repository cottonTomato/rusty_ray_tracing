use crate::{Point3D, Vector3};

#[derive(Default, Debug)]
pub struct ImgAttr {
    pub img_width: u32,
    pub img_height: u32,
    pub aspect_ratio: f64,
}

impl ImgAttr {
    pub fn new(img_width: u32, aspect_ratio: f64) -> Self {
        let mut img_height = (img_width as f64 / aspect_ratio) as u32;
        img_height = if img_height < 1 { 1 } else { img_height };

        Self {
            img_width,
            img_height,
            aspect_ratio,
        }
    }
}

#[derive(Default, Debug)]
pub struct Camera {
    img_attr: ImgAttr,
    focal_length: f64,
    position: Point3D,
    pixel00_loc: Point3D,
    pixel_du: Vector3,
    pixel_dv: Vector3,
}

impl Camera {
    pub fn new(img_width: u32, aspect_ratio: f64) -> Self {
        let img_attr = ImgAttr::new(img_width, aspect_ratio);

        let center = Point3D::from_float(0.0, 0.0, 0.0);

        // Viewport Dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (img_attr.img_width as f64 / img_attr.img_height as f64);

        let viewport_u = Vector3::from_float(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::from_float(0.0, -viewport_height, 0.0);

        let pixel_du = viewport_u / img_attr.img_width as f64;
        let pixel_dv = viewport_v / img_attr.img_height as f64;

        let viewport_origin = center
            - Vector3::from_float(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_origin + (pixel_du + pixel_dv) * 0.5;

        Self {
            img_attr,
            focal_length,
            position: center,
            pixel00_loc,
            pixel_du,
            pixel_dv,
        }
    }

    pub fn img_attr(&self) -> &ImgAttr {
        &self.img_attr
    }

    pub fn focal_length(&self) -> f64 {
        self.focal_length
    }

    pub fn position(&self) -> Point3D {
        self.position
    }

    pub fn image_origin(&self) -> Point3D {
        self.pixel00_loc
    }

    pub fn du(&self) -> Point3D {
        self.pixel_du
    }

    pub fn dv(&self) -> Point3D {
        self.pixel_dv
    }
}
