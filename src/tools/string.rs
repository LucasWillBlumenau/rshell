use anyhow::Error;

pub fn split(text: &str, sep: char) -> (&str, &str) {
    let mut index = 0;
    let text_length = text.len();
    for c in text.chars() {
        if c == sep {
            return (&text[0..index], &text[index + 1..text_length])
        }
        index += 1;
    }

    return (&text, "")
}

pub fn split_args(text: &str) -> Result<Vec<String>, Error> {

    let mut text = text.trim_end()
        .trim_start();

    let mut args: Vec<String> = vec![];

    while text.len() > 0 {
        let index: usize;
        let arg: String;
        if text.starts_with('\'') {
            (index, arg) = search_until_close_quote(&text, '\'')?;
        } else if text.starts_with('"') {
            (index, arg) = search_until_close_quote(&text, '"')?;
        } else {
            (index, arg) = search_until_whitespace(&text);
        }

        args.push(arg);
        text = &text[index..text.len()].trim_start();
    }

    Ok(args)
}


fn search_until_close_quote(text: &str, quote: char) -> Result<(usize, String), Error> {
    let mut close_buffer = false;
    let mut buffer = String::new();
    let mut escaped = true;
    for (i, c) in text.chars().enumerate() {
        if i == 0 {
            continue;
        }
        
        if escaped {
            let to_append = handle_escaped_char(c);
            buffer.push_str(&to_append);
            escaped = false;
        } else if close_buffer {
            if c != quote {
                return Ok((i + 1, buffer));
            }
            close_buffer = false;
        }
        else if c == '\\' {
            escaped = true;
        } else if c != quote {
            buffer.push(c);
        } else {
            close_buffer = true;
        }
    }

    if close_buffer {
        Ok((text.len(), buffer))
    } else {
        Err(Error::msg(format!("{} not closed", quote)))
    }

}


fn search_until_whitespace(text: &str) -> (usize, String) {
    let mut buffer = String::new();
    let mut escaped = false;
    for (i, c) in text.chars().enumerate() {
        if escaped {
            let to_append = handle_escaped_char(c);
            buffer.push_str(&to_append);
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == ' ' {
            return (i + 1, buffer);
        } else {
            buffer.push(c);
        }
        
    }

    return (text.len(), buffer);
}


fn handle_escaped_char(c: char) -> String {
    match c {
        ' ' => String::from(" "),
        '\'' => String::from("'"),
        '"' => String::from("\""),
        _ => format!("\\{}", c)
    }

}