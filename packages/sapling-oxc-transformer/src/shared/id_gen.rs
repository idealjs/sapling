pub fn get_numbered_id(mut num: u32) -> String {
    const CHARS: &str = "etaoinshrdlucwmfygpbTAOISWCBvkxjqzPHFMDRELNGUKVYJQZX_$";
    const BASE: u32 = CHARS.len() as u32;

    let chars: Vec<char> = CHARS.chars().collect();
    let mut result = String::new();

    loop {
        let digit = (num % BASE) as usize;
        num = num / BASE;
        result.insert(0, chars[digit]);
        if num == 0 {
            break;
        }
    }

    result
}
