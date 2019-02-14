use image::{ImageBuffer, Rgba};
use rusttype::{point, Font, Scale};
use std::ops::DerefMut;

const FONT_DATA: &[u8] = include_bytes!("../fonts/OpenSans-Regular.ttf");
const FONT_SCALE: f32 = 32.0;

pub struct TextRenderer<'a> {
    font: Font<'a>,
    scale: Scale,
    colour: (u8, u8, u8),
}

impl<'a> TextRenderer<'a> {
    pub fn new(font: Font<'a>, scale: Scale, colour: (u8, u8, u8)) -> Self {
        Self {
            font,
            scale,
            colour,
        }
    }

    pub fn render_text<T>(&self, text: &str, image: &mut ImageBuffer<image::Rgba<u8>, T>)
    where
        T: DerefMut<Target = [<image::Rgba<u8> as image::Pixel>::Subpixel]>,
    {
        let v_metrics = self.font.v_metrics(self.scale);

        // layout the glyphs in a line with 20 pixels padding
        let glyphs: Vec<_> = self
            .font
            .layout(text, self.scale, point(20.0, 20.0 + v_metrics.ascent))
            .collect();

        // work out the layout size
        let _glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let _glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        Rgba {
                            data: [
                                self.colour.0,
                                self.colour.1,
                                self.colour.2,
                                (v * 255.0) as u8,
                            ],
                        },
                    )
                });
            }
        }
    }
}

impl<'a> Default for TextRenderer<'a> {
    fn default() -> Self {
        let font = Font::from_bytes(FONT_DATA).expect("Error constructing Font");
        let scale = Scale::uniform(FONT_SCALE);
        let colour = (255, 255, 255);
        Self::new(font, scale, colour)
    }
}
