
const CHARMAP: [u8; 65] = [
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
    0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58,
    0x59, 0x5A, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E,
    0x6F, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
    0x77, 0x78, 0x79, 0x7A, 0x30, 0x31, 0x32, 0x33,
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2B, 0x2F,
    0x3D // << extra byte for padding
];

#[allow(dead_code)]
fn lookup(c: u8) -> u8 {
    match CHARMAP.iter().position(|x| *x == c) {
        Some(f) => {
            if f == 64 { 0 } else { f as u8 }
        },
        None => 0
    }
}

/// Base64加密
pub extern fn encode(bytes: &[u8]) -> String {
    let mut encoded_data: Vec<u8> = Vec::new();
    for chunk in bytes.chunks(3) {
        let tmp_chunk: [u8; 3] = [
            { if chunk.len() < 1 { 0 } else { chunk[0] } },
            { if chunk.len() < 2 { 0 } else { chunk[1] } },
            { if chunk.len() < 3 { 0 } else { chunk[2] } },
        ];
        let word: u32 =
            ((tmp_chunk[0] as u32) << 16) |
            ((tmp_chunk[1] as u32) << 8) |
             (tmp_chunk[2] as u32);
        let out_chunk: [u8; 4] = [
            (word >> 18) as u8,
            (word << 14 >> 26) as u8,
            (word << 20 >> 26) as u8,
            (word << 26 >> 26) as u8,
        ];
        encoded_data.push(out_chunk[0]);
        encoded_data.push(out_chunk[1]);
        encoded_data.push(out_chunk[2]);
        encoded_data.push(out_chunk[3]);
    }
    let pad_size = 3 - (bytes.len() % 3);
    if pad_size != 3 {
        for i in 0..pad_size {
            let position: usize = (encoded_data.len() - 1 - i) as usize;
            encoded_data[position] = 64;
        }
    }
    encoded_data.iter().map(|c| CHARMAP[*c as usize] as char).collect()
}

/// Base64解密
#[allow(dead_code)]
pub extern fn decode(string: &str) -> Vec<u8> {
    let mut decoded_data: Vec<u8> = Vec::new();
    let bytes = string.as_bytes();
    for chunk in bytes.chunks(4) {
        let restored_chunk: [u8; 4] = [
            { lookup(chunk[0]) },
            { lookup(chunk[1]) },
            { lookup(chunk[2]) },
            { lookup(chunk[3]) },
        ];
        let word: u32 =
            ((restored_chunk[0] as u32) << 18) |
            ((restored_chunk[1] as u32) << 12) |
            ((restored_chunk[2] as u32) << 6) |
             (restored_chunk[3] as u32);
        let out_chunk: [u8; 3] = [
            (word <<  8 >> 24) as u8,
            (word << 16 >> 24) as u8,
            (word << 24 >> 24) as u8,
        ];
        if out_chunk[0] != 0 { decoded_data.push(out_chunk[0]); };
        if out_chunk[1] != 0 { decoded_data.push(out_chunk[1]); };
        if out_chunk[2] != 0 { decoded_data.push(out_chunk[2]); };
    }
    decoded_data
}