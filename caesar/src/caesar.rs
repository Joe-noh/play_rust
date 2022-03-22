pub fn encrypt(message: &str) -> String {
    return shift(message, 3);
}

pub fn decrypt(encrypted: &str) -> String {
    return shift(encrypted, -3);
}

fn shift(text: &str, diff: i16) -> String {
    let a_code = 'A' as i16;
    let z_code = 'Z' as i16;

    text.chars()
        .map(|c| c as i16)
        .map(|code| {
            if a_code <= code && code <= z_code {
                (code - a_code + diff + 26) % 26 + a_code
            } else {
                code
            }
        })
        .map(|c| (c as u8) as char)
        .collect()
}
