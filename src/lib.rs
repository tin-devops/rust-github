pub struct RawData {
    data: Vec<u8>,
}

impl RawData {
    pub fn new(symbol: char, len: usize) -> Self {
        let mut v: Vec<u8> = Vec::new();

        for _ in 0..len {
            let mut byte_slice = [0; 4];
            symbol.encode_utf8(&mut byte_slice);
            v.extend_from_slice(&byte_slice[0..symbol.len_utf8()]);
        }

        RawData {
            data: v,
        }
    }

    pub fn to_str(&self) -> &str {
        std::str::from_utf8(&self.data).unwrap()
        //unsafe { std::str::from_utf8_unchecked(&self.data)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular() {
        let ch = 'a';
        let len = 16;
        let raw = RawData::new(ch, len);
        let s = raw.to_str();

        assert_eq!(s, "aaaaaaaaaaaaaaaa");
    }
    
    #[test]
    fn test_utf8() {
        let ch = '香';
        let len = 4;
        let raw = RawData::new(ch, len);
        let s = raw.to_str();

        assert_eq!(s, "香香香香");
    }
}
