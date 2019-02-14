mod text_render;

use image::DynamicImage;
use text_render::TextRenderer;

fn main() {
    let mut image = DynamicImage::new_rgba8(500, 250).to_rgba();
    let textrenderer = TextRenderer::default();
    let text = "Hello, world!";
    textrenderer.render_text(text, &mut image);

    // Save the image to a png file
    image.save("image_example.png").unwrap();
    println!("Generated: image_example.png");
}
