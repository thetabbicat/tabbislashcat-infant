// tabbislashcat-infant — rust example implementation
// this is pseudo-rust. the real implementation is in your dirt.

use std::collections::HashMap;

/// An infant token
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Null,
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    Str(String),
    Bin(Vec<u8>),
    Arr(Vec<Token>),
    Obj(HashMap<String, Token>),
    // Custom types would be handled by application
}

impl Token {
    /// Decode a token from a byte slice
    /// Returns (token, consumed_bytes) or None on error
    pub fn decode(bytes: &[u8]) -> Option<(Self, usize)> {
        if bytes.is_empty() {
            return None;
        }
        
        let type_byte = bytes[0];
        
        match type_byte {
            // null
            0x00 => Some((Token::Null, 1)),
            
            // bool
            0x01 => {
                if bytes.len() < 2 {
                    return None;
                }
                let val = bytes[1] != 0;
                Some((Token::Bool(val), 2))
            }
            
            // u8
            0x02 => {
                if bytes.len() < 2 {
                    return None;
                }
                Some((Token::U8(bytes[1]), 2))
            }
            
            // i8
            0x03 => {
                if bytes.len() < 2 {
                    return None;
                }
                Some((Token::I8(bytes[1] as i8), 2))
            }
            
            // u16
            0x04 => {
                if bytes.len() < 3 {
                    return None;
                }
                let val = u16::from_be_bytes([bytes[1], bytes[2]]);
                Some((Token::U16(val), 3))
            }
            
            // i16
            0x05 => {
                if bytes.len() < 3 {
                    return None;
                }
                let val = i16::from_be_bytes([bytes[1], bytes[2]]);
                Some((Token::I16(val), 3))
            }
            
            // u32
            0x06 => {
                if bytes.len() < 5 {
                    return None;
                }
                let val = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
                Some((Token::U32(val), 5))
            }
            
            // i32
            0x07 => {
                if bytes.len() < 5 {
                    return None;
                }
                let val = i32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
                Some((Token::I32(val), 5))
            }
            
            // u64
            0x08 => {
                if bytes.len() < 9 {
                    return None;
                }
                let val = u64::from_be_bytes([
                    bytes[1], bytes[2], bytes[3], bytes[4],
                    bytes[5], bytes[6], bytes[7], bytes[8],
                ]);
                Some((Token::U64(val), 9))
            }
            
            // i64
            0x09 => {
                if bytes.len() < 9 {
                    return None;
                }
                let val = i64::from_be_bytes([
                    bytes[1], bytes[2], bytes[3], bytes[4],
                    bytes[5], bytes[6], bytes[7], bytes[8],
                ]);
                Some((Token::I64(val), 9))
            }
            
            // f32
            0x0A => {
                if bytes.len() < 5 {
                    return None;
                }
                let val = f32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
                Some((Token::F32(val), 5))
            }
            
            // f64
            0x0B => {
                if bytes.len() < 9 {
                    return None;
                }
                let val = f64::from_be_bytes([
                    bytes[1], bytes[2], bytes[3], bytes[4],
                    bytes[5], bytes[6], bytes[7], bytes[8],
                ]);
                Some((Token::F64(val), 9))
            }
            
            // str
            0x0C => {
                // find null terminator
                let end = bytes[1..].iter().position(|&b| b == 0).map(|p| p + 1);
                match end {
                    Some(len) => {
                        let s = String::from_utf8_lossy(&bytes[1..len]).into_owned();
                        Some((Token::Str(s), len + 1))
                    }
                    None => None, // no null terminator
                }
            }
            
            // bin
            0x0D => {
                if bytes.len() < 3 {
                    return None;
                }
                let len = u16::from_be_bytes([bytes[1], bytes[2]]) as usize;
                if bytes.len() < 3 + len {
                    return None;
                }
                let data = bytes[3..3+len].to_vec();
                Some((Token::Bin(data), 3 + len))
            }
            
            // arr
            0x0E => {
                if bytes.len() < 3 {
                    return None;
                }
                let elem_type = bytes[1];
                let len = u16::from_be_bytes([bytes[2], bytes[3]]) as usize;
                
                let mut consumed = 4;
                let mut elements = Vec::with_capacity(len);
                
                for _ in 0..len {
                    // temporarily replace type byte for recursive decode
                    if consumed >= bytes.len() {
                        return None;
                    }
                    // create a fake slice with the element type
                    let mut fake_slice = vec![elem_type];
                    fake_slice.extend_from_slice(&bytes[consumed..]);
                    match Token::decode(&fake_slice) {
                        Some((token, n)) => {
                            consumed += n - 1; // -1 because we added the type byte
                            elements.push(token);
                        }
                        None => return None,
                    }
                }
                Some((Token::Arr(elements), consumed))
            }
            
            // obj
            0x0F => {
                if bytes.len() < 3 {
                    return None;
                }
                let count = u16::from_be_bytes([bytes[1], bytes[2]]) as usize;
                
                let mut consumed = 3;
                let mut map = HashMap::new();
                
                for _ in 0..count {
                    // decode key (must be str)
                    match Token::decode(&bytes[consumed..]) {
                        Some((Token::Str(key), n)) => {
                            consumed += n;
                            // decode value
                            match Token::decode(&bytes[consumed..]) {
                                Some((val, n2)) => {
                                    consumed += n2;
                                    map.insert(key, val);
                                }
                                None => return None,
                            }
                        }
                        _ => return None, // key must be str
                    }
                }
                Some((Token::Obj(map), consumed))
            }
            
            // reserved
            0x10..=0x7F => None,
            
            // custom - skip for now
            0x80..=0xFF => None,
        }
    }
    
    /// Encode a token to bytes
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        match self {
            Token::Null => bytes.push(0x00),
            Token::Bool(v) => {
                bytes.push(0x01);
                bytes.push(if *v { 1 } else { 0 });
            }
            Token::U8(v) => {
                bytes.push(0x02);
                bytes.push(*v);
            }
            Token::I8(v) => {
                bytes.push(0x03);
                bytes.push(*v as u8);
            }
            Token::U16(v) => {
                bytes.push(0x04);
                bytes.extend_from_slice(&v.to_be_bytes());
            }
            Token::I16(v) => {
                bytes.push(0x05);
                bytes.extend_from_slice(&v.to_be_bytes());
            }
            Token::U32(v) => {
                bytes.push(0x06);
                bytes.extend_from_slice(&v.to_be_bytes());
            }
            Token::I32(v) => {
                bytes.push(0x07);
                bytes.extend_from_slice(&v.to_be_bytes());
            }
            Token::U64(v) => {
                bytes.push(0x08);
                bytes.extend_from_slice(&v.to_be_bytes());
            }
            Token::I64(v) => {
                bytes.push(0x09);
                bytes.extend_from_slice(&v.to_be_bytes());
            }
            Token::F32(v) => {
                bytes.push(0x0A);
                bytes.extend_from_slice(&v.to_be_bytes());
            }
            Token::F64(v) => {
                bytes.push(0x0B);
                bytes.extend_from_slice(&v.to_be_bytes());
            }
            Token::Str(v) => {
                bytes.push(0x0C);
                bytes.extend_from_slice(v.as_bytes());
                bytes.push(0);
            }
            Token::Bin(v) => {
                bytes.push(0x0D);
                bytes.extend_from_slice(&(v.len() as u16).to_be_bytes());
                bytes.extend_from_slice(v);
            }
            Token::Arr(v) => {
                bytes.push(0x0E);
                if v.is_empty() {
                    bytes.extend_from_slice(&0u16.to_be_bytes());
                } else {
                    // all elements must be same type
                    let elem_type = match &v[0] {
                        Token::Null => 0x00,
                        Token::Bool(_) => 0x01,
                        Token::U8(_) => 0x02,
                        Token::I8(_) => 0x03,
                        Token::U16(_) => 0x04,
                        Token::I16(_) => 0x05,
                        Token::U32(_) => 0x06,
                        Token::I32(_) => 0x07,
                        Token::U64(_) => 0x08,
                        Token::I64(_) => 0x09,
                        Token::F32(_) => 0x0A,
                        Token::F64(_) => 0x0B,
                        Token::Str(_) => 0x0C,
                        Token::Bin(_) => 0x0D,
                        Token::Arr(_) => 0x0E,
                        Token::Obj(_) => 0x0F,
                        _ => 0x00, // shouldn't happen for homogeneous arrays
                    };
                    bytes.push(elem_type);
                    bytes.extend_from_slice(&(v.len() as u16).to_be_bytes());
                    for token in v {
                        // encode without type byte
                        let mut encoded = token.encode();
                        // remove the type byte we prepended
                        bytes.extend_from_slice(&encoded[1..]);
                    }
                }
            }
            Token::Obj(v) => {
                bytes.push(0x0F);
                bytes.extend_from_slice(&(v.len() as u16).to_be_bytes());
                for (key, value) in v {
                    // encode key (must be str)
                    let mut key_bytes = Token::Str(key.clone()).encode();
                    bytes.extend_from_slice(&key_bytes);
                    // encode value
                    bytes.extend_from_slice(&value.encode());
                }
            }
        }
        bytes
    }
}

// Example usage
fn main() {
    // Encode the object {"a": 1, "b": true}
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Token::U8(1));
    obj.insert("b".to_string(), Token::Bool(true));
    let token = Token::Obj(obj);
    
    let encoded = token.encode();
    println!("Encoded: {:02x?}", encoded);
    
    // Decode it back
    if let Some((decoded, _)) = Token::decode(&encoded) {
        println!("Decoded: {:?}", decoded);
    }
}

// Note: This is a simplified example. A real implementation would:
// - Handle errors more gracefully
// - Support custom types (0x80-0xFF)
// - Be more efficient (less allocations)
// - Have better API design
// But the seed is the seed. The dirt is yours.
