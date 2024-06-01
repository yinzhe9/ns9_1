use rand::Rng;
use crate::b2048;

const ENC_TABLE: &'static [char; 2056] = &include!("./enc_table.src");

pub fn encode(input: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut j = 0.0;
    let mut output = String::new();
    for index in b2048::encode(rmw_utf8::encode(input.as_bytes()).as_slice()) {
        output.push(ENC_TABLE[index as usize]);
        if !rng.gen_bool(1.0 / (12.0 - j)) {
            j += (12.0 - j) / 12.0;
        } else {
            j = 0.0;
            output.push(['，', '。', '？', '！', '…', '~', '：', '、'][rng.gen_range(0..8)])
        }
    }
    output
}

const DEC_TABLE: &'static [u16; 35303] = &include!("./dec_table.src");

pub fn decode(input: &str) -> String {
    let mut output = Vec::new();
    for char in input.chars() {
        let index = char as usize;
        if index >= DEC_TABLE.len() {
            continue;
        }
        let code = DEC_TABLE[index];
        if code == 0xFFFF {
            continue;
        }
        output.push(code)
    }
    rmw_utf8::decode(b2048::decode(output.as_slice()).as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let raw = "你好，世界！";
        assert_eq!(decode(&encode(&raw)), raw);
    }
}