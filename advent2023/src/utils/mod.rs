pub fn extract_between(input: &str, start_char: char, end_char: char) -> Option<&str> {
    let start = input.find(start_char)? + 1; // Find the index of start_char and move past it
    let end = input[start..].find(end_char)? + start; // Find the index of end_char after start_char
    Some(&input[start..end]) // Extract the substring
}

pub fn extract_end(input: &str, start_char: char) -> Option<&str> {
    let start = input.find(start_char)? + 1; // Find the index of start_char and move past it
    Some(&input[start..]) // Extract the substring
}