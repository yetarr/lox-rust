pub fn is_alpha(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
}

pub fn is_number(c: char) -> bool {
    c.is_ascii_digit()
}

pub fn is_alpha_numeric(c: char) -> bool {
    is_number(c) || is_alpha(c)
}
