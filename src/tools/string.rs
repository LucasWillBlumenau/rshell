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

    let text = text.trim_end()
        .trim_start();  

    let mut quote = '\0';
    let mut args: Vec<String> = vec![];
    let mut escaped = false;

    let mut buffer = String::new();

    for c in text.chars() {

        if c == quote && !escaped {
            args.push(buffer.clone());
            buffer.clear();
            quote = '\0';
        } else if c == '\'' && quote == '\0'  {
            quote = '\'';
        } else if c == '"' && quote == '\0' {
            quote = '"';
        } else if c == ' ' && quote == '\0' {
            if buffer != "" {
                args.push(buffer.clone());
                buffer.clear();
            }
        } else if c == '\\'{
            escaped = true;
        }    
        else {
            buffer.push(c);
            escaped = false;
        }

    }

    if quote != '\0' {
        return Err(Error::msg(format!("quote {} not closed", quote)));
    }

    if buffer != "" {
        args.push(buffer);
    }
    Ok(args)
}
