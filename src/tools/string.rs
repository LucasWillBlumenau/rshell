pub fn add_new_line_to_string_if_its_missing_and_its_not_empty(text: &str) -> String {   
    let mut buffer = String::from(text);
    if !text.ends_with("\n") && text.len() > 0 {
        buffer.push('\n');
    }
    buffer
}