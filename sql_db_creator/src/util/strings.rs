pub fn get_last_of_split<'a>(text: &'a str, seperator: &'a str) -> Option<&'a str> {
    let split = text.split(seperator);
    let parts: Vec<&str> = split.collect();

    if parts.len() > 0 {
        let part = parts.last();
        match part {
            Some(name) => return Some(name),
            None => return None
        }
    }

    None
}

pub fn get_first_of_split<'a>(text: &'a str, seperator: &'a str) -> Option<&'a str> {
    let split = text.split(seperator);
    let parts: Vec<&str> = split.collect();

    if parts.len() > 0 {
        let part = parts.first();
        match part {
            Some(name) => return Some(name),
            None => return None
        }
    }

    None
}

pub fn reduce_str(string_slice: &str, start: usize, end: usize) -> String {
    let string_slice_new = &string_slice[start..string_slice.len() - end];
    
    String::from(string_slice_new)
}
