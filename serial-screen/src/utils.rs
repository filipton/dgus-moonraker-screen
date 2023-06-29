// TODO: optimize
pub fn center_pad(s: &str, pad_char: char, width: usize) -> String {
    let l = width / 2;
    let r = width - l;

    format!(
        "{}{}{}",
        pad_char.to_string().repeat(l),
        s,
        pad_char.to_string().repeat(r)
    )
}
