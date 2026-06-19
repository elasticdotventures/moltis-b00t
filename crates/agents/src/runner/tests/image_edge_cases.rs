//! Additional image extraction edge-case tests.

use super::helpers::*;

#[test]
fn test_extract_images_webp() {
    let payload = "B".repeat(300);
    let input = format!("data:image/webp;base64,{payload}");
    let (images, _remaining) = extract_images_from_text(&input);
    assert_eq!(images.len(), 1);
    assert_eq!(images[0].media_type, "image/webp");
}

#[test]
fn test_extract_images_gif() {
    let payload = "C".repeat(300);
    let input = format!("data:image/gif;base64,{payload}");
    let (images, _remaining) = extract_images_from_text(&input);
    assert_eq!(images.len(), 1);
    assert_eq!(images[0].media_type, "image/gif");
}

#[test]
fn test_extract_images_with_special_base64_chars() {
    let payload = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/==";
    let padded = format!("{}{}", payload, "A".repeat(200));
    let input = format!("data:image/png;base64,{padded}");
    let (images, _remaining) = extract_images_from_text(&input);
    assert_eq!(images.len(), 1);
    assert!(images[0].data.contains('+'));
    assert!(images[0].data.contains('/'));
}

#[test]
fn test_extract_images_preserves_surrounding_text() {
    let payload = "A".repeat(300);
    let input = format!(
        "Before the image\n\ndata:image/png;base64,{payload}\n\nAfter the image with special chars: <>&"
    );
    let (images, remaining) = extract_images_from_text(&input);
    assert_eq!(images.len(), 1);
    assert!(remaining.contains("Before the image"));
    assert!(remaining.contains("After the image with special chars: <>&"));
    assert!(!remaining.contains(&payload));
}
