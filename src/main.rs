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
    data: String,
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

    fn encode(mode: &Mode, data: &str) -> String {
        match mode {
            // Mode::Numeric => {
            // }
            Mode::Alphanumeric => {
                let foo = data.chars().into_iter().collect::<Vec<_>>();
                let bar = foo
                    .chunks(2)
                    .map(|chunk| {
                        if chunk.len() == 2 {
                            let first = Self::encoding_for_alphanumeric(&chunk[0]);
                            let second = Self::encoding_for_alphanumeric(&chunk[1]);

                            let result = first * 45 + second;

                            format!("{:011b}", result)
                        } else {
                            let result = Self::encoding_for_alphanumeric(&chunk[0]);
                            format!("{:06b}", result)
                        }
                    })
                    .fold(String::from(""), |acc, x| acc + &x);

                bar
            }

            _ => unimplemented!(),
            // Mode::Byte => {
            // }
            // Mode::Kanji => {
            // }
        }
    }

    fn encoding_for_alphanumeric(c: &char) -> usize {
        // Table 5 — Alphanumeric mode encoding - ISO 18004:2015
        match c {
            '0'..='9' => *c as usize - 48,
            'A'..='Z' => *c as usize - 55,
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
        }
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

    #[test]
    fn test_encode_alphanum() {
        let qr_code = QRCode::new("AC-42");

        assert_eq!(qr_code.data, "0011100111011100111001000010")
    }

    #[test]
    fn test_encode_alphanum2() {
        let qr_code = QRCode::new("HELLO CC WORLD");

        assert_eq!(
            qr_code.data,
            "01100001011011110001101000101110001000101000110011101001000101001101110111110"
        )
    }
}
