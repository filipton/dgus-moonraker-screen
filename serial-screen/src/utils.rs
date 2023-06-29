// TODO: optimize
pub fn center_pad(s: &str, pad_char: &str, width: usize) -> String {
    if s.len() >= width {
        return s[..width].to_string();
    }

    let l = (width - s.len()) / 2;
    let r = width - s.len() - l;

    format!("{}{}{}", pad_char.repeat(l), s, pad_char.repeat(r))
}
