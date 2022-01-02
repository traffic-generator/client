use regex::Regex;
use std::error::Error;
use std::{fmt, fs, num::ParseIntError, path::Path};

/// Read a file in hexdump format and return a byte vector
pub fn read_from_file(file_name: &str) -> Result<Vec<u8>, DecodeHexError> {
    let file_path = Path::new(file_name);
    let file_content = fs::read_to_string(file_path).expect("Could not read file:");

    let re = Regex::new(
        r"(?ixm)
        ^                                   # start of line
        (?P<address>[\d[a-f]]{2,})          # address = [0-f][0-f][0-f][0-f]...
        [^\d[a-f]]+                         # seperator = not [0-f]
        (?P<data>([\d[a-f]]{2,}[\s\t:|])+)  # data bytes = '[0-f][0-f]_[0-f][0-f]_...
        $                                   # end of line
        ",
    )
    .unwrap();
    let caps = re.captures_iter(&file_content); // TODO: error handling
    let mut data: Vec<u8> = Vec::new();
    for cap in caps {
        let mut cap_data = string_to_bytes(&cap["data"]);
        match cap_data {
            Ok(ref mut d) => data.append(d),
            Err(e) => return Err(e),
        }
        match u32::from_str_radix(&cap["address"], 16) {
            Ok(a) => {
                //TODO: check if address matches byte count
            }
            Err(_e) => return Err(DecodeHexError::InvalidAddress),
        }
    }
    //return hex data
    Ok(data)
}

/// Converts hexdump line to byte vector
fn string_to_bytes(hex_string: &str) -> Result<Vec<u8>, DecodeHexError> {
    let hex_string = &hex_string
        .strip_suffix("\r")
        .unwrap_or(hex_string)
        .replace(" ", "")[..];
    decode_hex(hex_string)
}

/// Decode hex string to byte vector
fn decode_hex(s: &str) -> Result<Vec<u8>, DecodeHexError> {
    if s.len() % 2 != 0 {
        Err(DecodeHexError::OddLength)
    } else {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.into()))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeHexError {
    OddLength,
    InvalidAddress,
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for DecodeHexError {
    fn from(e: ParseIntError) -> Self {
        DecodeHexError::ParseInt(e)
    }
}

impl fmt::Display for DecodeHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeHexError::OddLength => "input string has an odd number of bytes".fmt(f),
            DecodeHexError::InvalidAddress => "address is not a valid number".fmt(f),
            DecodeHexError::ParseInt(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for DecodeHexError {}
