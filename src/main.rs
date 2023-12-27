#[derive(Debug, PartialEq)]
enum Mode {
    /// Numeric mode is for the digits 0 through 9
    Numeric,
    /// Alphanumeric mode is for the decimal digits 0 through 9, as well as uppercase letters, and the symbols $, %, *, +, -, ., /,:, and space
    Alphanumeric,
    /// Byte mode is for characters from the ISO-8859-1 character set.
    Byte,
    /// Kanji mode is for the double-byte characters from the Shift JIS character set
    Kanji,
}

impl Mode {
    fn to_bits(&self) -> u32 {
        match self {
            Mode::Numeric => 0b0001,
            Mode::Alphanumeric => 0b0010,
            Mode::Byte => 0b0100,
            Mode::Kanji => 0b1000,
        }
    }

    fn character_count(&self) -> u32 {
        match self {
            Mode::Numeric => 10,
            Mode::Alphanumeric => 9,
            Mode::Byte => 8,
            Mode::Kanji => 8,
        }
    }
}

enum ErrorCorrectionLevel {
    /// Recovers 7% of data
    L,
    /// Recovers 15% of data
    M,
    /// Recovers 25% of data
    Q,
    /// Recovers 30% of data
    H,
}

struct QRCode {
    version: u8,
    mode: Mode,
    ec_level: ErrorCorrectionLevel,
    mask_pattern: u8,
    data: Vec<u8>,
}

impl QRCode {
    fn new(data: &str) -> Self {
        let mode = get_mode(data);
        let version = 1;
        let ec_level = ErrorCorrectionLevel::L;
        let mask_pattern = 0;
        let data = Self::encode(&mode, data);

        QRCode {
            version,
            mode,
            ec_level,
            mask_pattern,
            data,
        }
    }

    fn encode(mode: &Mode, data: &str) -> Vec<u8> {
        let encoded_vec: Vec<u8> = match mode {
            // Mode::Numeric => {
            //     let mut encoded_str = String::new();
            //     let mut i = 0;
            //     while i < data.len() {
            //         let mut num = 0;
            //         for _ in 0..3 {
            //             num = num * 10 + data.chars().nth(i).unwrap().to_digit(10).unwrap();
            //             i += 1;
            //         }
            //         encoded_str.push_str(&format!("{:010b}", num));
            //     }
            //     encoded_str
            // }
            Mode::Alphanumeric => {
                let mut encoded_vec: Vec<u8> = Vec::new();
                let mut i = 0;
                while i < data.len() {
                    let mut num = 0;
                    for _ in 0..2 {
                        if let None = data.chars().nth(i) {
                            continue;
                        }
                        num = num * 45
                            + match data.chars().nth(i).unwrap() {
                                '0'..='9' => data.chars().nth(i).unwrap().to_digit(10).unwrap(),
                                'A'..='Z' => data.chars().nth(i).unwrap() as u32 - 55,
                                ' ' => 36,
                                '$' => 37,
                                '%' => 38,
                                '*' => 39,
                                '+' => 40,
                                '-' => 41,
                                '.' => 42,
                                '/' => 43,
                                ':' => 44,
                                _ => panic!("Invalid character"),
                            };
                        i += 1;
                    }
                    dbg!(num);

                    // encoded_vec.push(&format!(num));
                }
                encoded_vec
            }
            _ => unimplemented!(),
            // Mode::Byte => {
            //     let mut encoded_str = String::new();
            //     for c in data.chars() {
            //         encoded_str.push_str(&format!("{:08b}", c as u32));
            //     }
            //     encoded_str
            // }
            // Mode::Kanji => {
            //     let mut encoded_str = String::new();
            //     let mut i = 0;
            //     while i < data.len() {
            //         let mut num = 0;
            //         for _ in 0..2 {
            //             num = num * 256 + data.chars().nth(i).unwrap() as u32;
            //             i += 1;
            //         }
            //         encoded_str.push_str(&format!("{:013b}", num));
            //     }
            //     encoded_str
            // }
        };
        encoded_vec
    }
}

fn get_mode(data: &str) -> Mode {
    if data.chars().all(|c| c.is_numeric()) {
        return Mode::Numeric;
    } else if data
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_numeric() || "$%*+-./:, ".contains(c))
    {
        return Mode::Alphanumeric;
    } else if data.chars().all(|c| c as u32 <= 255) {
        return Mode::Byte;
    } else {
        return Mode::Kanji;
    }
}

fn main() {
    println!("Hello, world!");
    QRCode::new("AC-42");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mode() {
        assert_eq!(get_mode("1234567890"), Mode::Numeric);
        assert_eq!(get_mode("1234567890ABCDEF$%*+-./ "), Mode::Alphanumeric);
        assert_eq!(get_mode("aÄ"), Mode::Byte);
        assert_eq!(get_mode("Ä"), Mode::Byte);
        assert_eq!(get_mode("1234567890ABCDEFabcdefあいうえお"), Mode::Kanji);
    }
}
