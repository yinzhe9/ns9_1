pub fn encode(bytes: &[u8]) -> Vec<u16> {
    let mut ret = Vec::new();
    let mut stage = 0x0000u16;
    let mut remaining = 0;
    for byte in bytes {
        let byte = *byte as u16;
        let need = 11 - remaining;
        if need <= 8 {
            remaining = 8 - need;
            let index = (stage << need) | (byte >> remaining);
            ret.push(index);
            stage = byte & ((1 << remaining) - 1);
        } else {
            stage = (stage << 8) | byte;
            remaining += 8;
        }
    }
    ret.push(if remaining <= 3 { stage + 2048 } else { stage << (11 - remaining) });
    ret
}

pub fn decode(chars: &[u16]) -> Vec<u8> {
    let mut ret = Vec::new();
    let mut remaining = 0u8;
    let mut stage = 0x00u32;
    let mut chars = chars.iter().peekable();
    let mut residue = 0;
    while let Some(c) = chars.next() {
        residue = (residue + 11) % 8;
        let (n_new_bits, new_bits) = match c {
            0..=2047 => {
                if chars.peek().is_none() {
                    (11 - residue, *c >> residue)
                } else {
                    (11, *c)
                }
            }
            _ => {
                (8 - remaining, c - 2048)
            }
        };
        remaining += n_new_bits;
        stage = (stage << n_new_bits) | new_bits as u32;
        while remaining >= 8 {
            remaining -= 8;
            ret.push((stage >> remaining) as u8);
            stage = stage & ((1 << remaining) - 1)
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let raw = "你好，世界！".as_bytes();
        assert_eq!(decode(encode(&raw).as_slice()), raw);
    }
}