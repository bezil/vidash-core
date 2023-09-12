use crate::generators::generate_thumbnail;
use crate::generators::generate_thumbnail_with_title::generate_thumbnail_with_title;
use crate::tests::test_generate_thumbnail::generate_thumbnail::generate_thumbnail;

#[test]
pub fn test_generate_thumbnail() {
    // Define test variables
    let video_path = "video.mp4";
    let interval = 10; // Adjust as needed
    let output_path = "thumbnail.png";

    // Call the function and check the result
    let result = generate_thumbnail(video_path, interval, output_path);
    assert!(result.is_ok(), "Thumbnail generation failed: {:?}", result);
}

#[test]
pub fn test_generate_thumbnail_with_title() {
    // Define test variables
    let title = "New Title"; // Adjust as needed
    let output_path = "thumbnailtitle.png";

    // Call the function and check the result
    let result = generate_thumbnail_with_title(title, output_path);
    assert!(result.is_ok(), "Thumbnail generation failed: {:?}", result);
}
