//! Additional tool-result content formatting tests.

use super::helpers::*;

#[test]
fn test_tool_result_to_content_openai_format() {
    let payload = "A".repeat(300);
    let input = format!("Text: data:image/png;base64,{payload}");
    let result = tool_result_to_content(&input, 50_000, true);
    let arr = result.as_array().unwrap();
    assert_eq!(arr[0]["type"], "text");
    assert!(arr[0]["text"].is_string());
    assert_eq!(arr[1]["type"], "image_url");
    assert!(arr[1]["image_url"].is_object());
    assert!(arr[1]["image_url"]["url"].is_string());
    let url = arr[1]["image_url"]["url"].as_str().unwrap();
    assert!(url.starts_with("data:image/png;base64,"));
}

#[test]
fn test_tool_result_to_content_truncation() {
    let payload = "A".repeat(300);
    let long_text = "X".repeat(10_000);
    let input = format!("{long_text} data:image/png;base64,{payload}");
    let result = tool_result_to_content(&input, 500, true);
    let arr = result.as_array().unwrap();
    let text = arr[0]["text"].as_str().unwrap();
    assert!(
        text.contains("[truncated"),
        "text should be truncated: {text}"
    );
    assert_eq!(arr[1]["type"], "image_url");
}
