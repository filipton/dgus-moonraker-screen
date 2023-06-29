// TODO: optimize
pub fn center_pad(s: &str, pad_char: char, width: usize) -> String {
    let l = (width - s.len()) / 2;
    let r = width - s.len() - l;

    println!("{} {} {}", l, s.len(), r);

    format!(
        "{}{}{}",
        pad_char.to_string().repeat(l),
        s,
        pad_char.to_string().repeat(r)
    )
}
