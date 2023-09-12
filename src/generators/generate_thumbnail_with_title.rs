
use image::{ImageBuffer, ImageFormat, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

pub fn generate_thumbnail_with_title(title: &str, output_path: &str) -> Result<(), String> {
    // Define the desired thumbnail size
    let thumbnail_width = 320;
    let thumbnail_height = 240;

    // Create a black background image
    let mut thumbnail_image = ImageBuffer::from_pixel(thumbnail_width, thumbnail_height, Rgba([0, 0, 0, 255]));

    // Calculate the position to center the title
    let font_size = 20.0;
    let text_width = (font_size * title.len() as f32) as u32;
    let text_height = font_size as u32;

    let x = (thumbnail_width - text_width) / 2;
    let y = (thumbnail_height - text_height) / 2;

    // Add the centered title to the image
    draw_text(&mut thumbnail_image, x as i32, y as i32, title, font_size);

    // Save the thumbnail image to the output path
    if let Err(_) = thumbnail_image.save_with_format(output_path, ImageFormat::Png) {
        return Err("Failed to save the thumbnail image.".to_string());
    }

    Ok(())
}

pub fn draw_text(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: i32, y: i32, text: &str, font_size: f32) {
    let color = Rgba([255, 255, 255, 255]); // White text color with full opacity
    let font_bytes = include_bytes!("../../babyfont.ttf"); // Replace with the path to your downloaded font

    let font = Font::try_from_bytes(font_bytes).expect("Error loading font");
    let scale = Scale::uniform(font_size);

    // let text = Text::new(text, (x, y), color);

    draw_text_mut(image, color, x, y, scale, &font, text);
}
