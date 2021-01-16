//https://tools.ietf.org/html/rfc1951
//https://fat-crocodile.livejournal.com/194295.html
//http://compression.ru/download/articles/lz/mihalchik_deflate_decoding.html
//https://www.w3.org/Graphics/PNG/RFC-1951
// https://tools.ietf.org/html/rfc1951
// https://gchq.github.io/CyberChef/#recipe=Raw_Deflate('Dynamic%20Huffman%20Coding')To_Hex('Space',0)&input=SSBzYXcgeW91IGRhbmNpbmcgaW4gYSBjcm93ZGVkIHJvb20KWW91IGxvb2sgc28gaGFwcHkgd2hlbiBJJ20gbm90IHdpdGggeW91CkJ1dCB0aGVuIHlvdSBzYXcgbWUsIGNhdWdodCB5b3UgYnkgc3VycHJpc2UKQSBzaW5nbGUgdGVhcmRyb3AgZmFsbGluZyBmcm9tIHlvdXIgZXll

const CODE_MAX_LEN: usize = 15;
const END_OF_DATA: u16 = 256;

struct BitStream<'a> {
    data: &'a [u8],
    current_byte: usize,
    offset_in_byte: u8,
    order: bool,
}

impl <'a> BitStream<'a> {
    fn set_static_huffman_bit_order(&mut self, set: bool) {
        self.order = set;
    }

    fn skip_bits_to_byte_border(&mut self) {
        while self.offset_in_byte != 1 {
            if self.offset_in_byte == 0x80 {
                self.offset_in_byte = 1;
                self.current_byte += 1;
            } else {
                self.offset_in_byte <<= 1;
            }
        }
    }

    fn read(&mut self, bit_count: usize) -> u16 {
        let mut res = 0u16;
        //print!("read: ");
        if !self.order {
            for i in 0..bit_count {
                if (self.data[self.current_byte] & self.offset_in_byte) != 0 {
                    res |= 1 << i;
                    //print!("1");
                } else {
                    //print!("0");
                }

                if self.offset_in_byte == 0x80 {
                    self.offset_in_byte = 1;
                    self.current_byte += 1;
                } else {
                    self.offset_in_byte <<= 1;
                }
            }
        } else {
            for _ in 0..bit_count {
                res <<= 1;
    
                if (self.data[self.current_byte] & self.offset_in_byte) != 0 {
                    res |= 1;
                    print!("1");
                } else {
                    print!("0");
                }
    
                if self.offset_in_byte == 0x80 {
                    self.offset_in_byte = 1;
                    self.current_byte += 1;
                } else {
                    self.offset_in_byte <<= 1;
                }
            }
        }
        //println!();
        res
    }
}

struct Length {
    bits: u8,
    len: u16,
}

const XXX: [Length; 29] = [
    Length { bits: 0, len: 3}, // 257
    Length { bits: 0, len: 4}, // 258
    Length { bits: 0, len: 5}, // 259
    Length { bits: 0, len: 6}, // 260
    Length { bits: 0, len: 7}, // 261
    Length { bits: 0, len: 8}, // 262
    Length { bits: 0, len: 9}, // 263
    Length { bits: 0, len: 10}, // 264
    Length { bits: 1, len: 11}, // 265
    Length { bits: 1, len: 13}, // 266
    Length { bits: 1, len: 15}, // 267
    Length { bits: 1, len: 17}, // 268
    Length { bits: 2, len: 19}, // 269
    Length { bits: 2, len: 23}, // 270
    Length { bits: 2, len: 27}, // 271
    Length { bits: 2, len: 31}, // 272
    Length { bits: 3, len: 35}, // 273
    Length { bits: 3, len: 43}, // 274
    Length { bits: 3, len: 51}, // 275
    Length { bits: 3, len: 59}, // 276
    Length { bits: 4, len: 67}, // 277
    Length { bits: 4, len: 83}, // 278
    Length { bits: 4, len: 99}, // 279
    Length { bits: 4, len: 115}, // 280
    Length { bits: 5, len: 131}, // 281
    Length { bits: 5, len: 163}, // 282
    Length { bits: 5, len: 195}, // 283
    Length { bits: 5, len: 227}, // 284
    Length { bits: 0, len: 258}, // 285
];

const YYY: [Length; 30] = [
    Length { bits: 0, len: 1}, // 0
    Length { bits: 0, len: 2}, // 1
    Length { bits: 0, len: 3}, // 2
    Length { bits: 0, len: 4}, // 3
    Length { bits: 1, len: 5}, // 4
    Length { bits: 1, len: 7}, // 5
    Length { bits: 2, len: 9}, // 6
    Length { bits: 2, len: 13}, // 7
    Length { bits: 3, len: 17}, // 8
    Length { bits: 3, len: 25}, // 9
    Length { bits: 4, len: 33}, // 10
    Length { bits: 4, len: 49}, // 11
    Length { bits: 5, len: 65}, // 12
    Length { bits: 5, len: 97}, // 13
    Length { bits: 6, len: 129}, // 14
    Length { bits: 6, len: 193}, // 15
    Length { bits: 7, len: 257}, // 16
    Length { bits: 7, len: 385}, // 17
    Length { bits: 8, len: 513 }, // 18
    Length { bits: 8, len: 769}, // 19
    Length { bits: 9, len: 1025}, // 20
    Length { bits: 9, len: 1537}, // 21
    Length { bits: 10, len: 2049}, // 22
    Length { bits: 10, len: 3073}, // 23
    Length { bits: 11, len: 4097}, // 24
    Length { bits: 11, len: 6145}, // 25
    Length { bits: 12, len: 8193}, // 26
    Length { bits: 12, len: 12289}, // 27
    Length { bits: 13, len: 16385}, // 28
    Length { bits: 13, len: 24577}, // 29
];

fn huffman_read_length(code: u16, stream: &mut BitStream) -> usize {
    let i = code as usize - 257;
    XXX[i].len as usize + stream.read(XXX[i].bits as usize) as usize
}

fn huffman_read_distance(stream: &mut BitStream) -> usize
{
    let i = stream.read(5) as usize;
    stream.set_static_huffman_bit_order(false);
    let distance = (stream.read(YYY[i].bits as usize) + YYY[i].len) as usize;
    stream.set_static_huffman_bit_order(true);
    distance
}

fn huffman_copy_pair(length: usize, distance: usize, buf: &mut [u8], buf_pos: &mut usize)
{
    let mut start = *buf_pos - distance;

    for _ in 0..length {
        buf[*buf_pos] = buf[start];
        *buf_pos += 1;
        start += 1;
    }
}

fn static_huffman_decode(stream: &mut BitStream, output: &mut [u8], output_pos: usize) -> Option<usize> {
    let mut current_pos = 0;
    stream.set_static_huffman_bit_order(true);

    loop {
        let cmd = stream.read(7);
        // 256-279  -- 7 бит, от 0000000 до 0010111
        if cmd <= 0b0010111 {
            let code = END_OF_DATA + cmd;

            if code == END_OF_DATA {
                // end of block!;
                return Some(current_pos - output_pos);
            }

            let len = huffman_read_length(code, stream);
            let distance = huffman_read_distance(stream);
            huffman_copy_pair(len, distance, output, &mut current_pos);
            continue;
        }

        let cmd = (cmd << 1) | stream.read(1);

        // 0-143  -- 8 bit, from 00110000 to 10111111
        if cmd >= 0b00110000 && cmd <= 0b10111111 {
            let byte = cmd - 0b00110000;
            output[current_pos] = byte as u8;
            current_pos += 1;
            continue;
        }

        // 280-287  -- 8 bit, from 11000000 to 11000111
        if cmd >= 0b11000000 && cmd <= 0b11000111 {
            let code = 280 + (cmd - 0b11000000);
            if code == 286 || code == 287 {
                // invalid value
                return None;
            }

            let len = huffman_read_length(code, stream);
            let distance = huffman_read_distance(stream);
            huffman_copy_pair(len, distance, output, &mut current_pos);
            continue;
        }

        let cmd = (cmd << 1) | stream.read(1);

        // 144-255  -- 9 bit, from 110010000 to 111111111
        if cmd >= 0b110010000 && cmd <= 0b111111111 {
            let byte = 144 + (cmd - 0b110010000);
            output[current_pos] = byte as u8;
            current_pos += 1;
            continue;
        }

        return None;
    }
}

struct DictionaryBuilder {
    cmd_len: [u8; 289],

    len_count: [usize; 16],
    position: usize,
    size: usize,
}

impl DictionaryBuilder {
    fn new(size: usize) -> Self {
        Self {
            cmd_len: [0u8; 289],
            len_count: [0; 16],
            position: 0,
            size,
        }
    }

    fn put(&mut self, len: u8) -> bool {
        if self.position >= self.size {
            return false;
        }

        self.len_count[len as usize] += 1;
        self.cmd_len[self.position] = len;
        self.position += 1;
        true
    }

    fn put_last(&mut self) -> bool {
        self.put(self.cmd_len[self.position- 1])
    }

    fn is_full(&self) -> bool {
        self.position >= self.size
    }

    fn build_dictionary(mut self) -> Dictionary {
        let mut len = [HuffmanLen {count: 0, base_code: 0, first_cmd_index: 0}; 16];
        let mut code = 0u16;
        self.len_count[0] = 0;
    
        for i in 1..16 {
            len[i].count = self.len_count[i] as u16;
            code = (code + self.len_count[i - 1] as u16) << 1;
            len[i].base_code = code;
            len[i].first_cmd_index = len[i - 1].first_cmd_index + len[i - 1].count;
        }

        let mut ordered_cmd = [0u16; 289];
        let mut offsets = [0u8; 289];

        for i in 0..self.size {
            let cmd_len = self.cmd_len[i] as usize;

            if cmd_len == 0 {
                continue;
            }

            let offset_in_cmd_table = len[cmd_len].first_cmd_index + offsets[cmd_len] as u16;
            offsets[cmd_len] += 1;
            ordered_cmd[offset_in_cmd_table as usize] = i as u16;
        }

        Dictionary { len , ordered_cmd }
    }
}

#[derive(Clone, Copy)]
struct HuffmanLen {
    count: u16,
    base_code: u16,
    first_cmd_index: u16,
}

struct Dictionary {
    len: [HuffmanLen; 16],
    ordered_cmd: [u16; 289],
}

impl Dictionary {
    fn find(&self, code: u16, len: u16) -> Option<u16> {
        let a = &self.len[len as usize];

        if a.count == 0 {
            return None;
        }
        
        if code < a.base_code || code >= (a.base_code + a.count) {
            return None;
        }

        let pos = a.first_cmd_index + (code - a.base_code);
        Some(self.ordered_cmd[pos as usize])
    }

    fn find_in_bit_stream(&self, stream: &mut BitStream) -> Option<u16> {
        let mut len = 0;
        let mut cmd = 0u16;
    
        loop {
            cmd = cmd << 1 | (stream.read(1) as u16);
            len += 1;
            
            if let Some(x) = self.find(cmd, len) {
                return Some(x);
            }

            if len > CODE_MAX_LEN as u16 {
                return None;
            }
        }
    }
}

fn dynamic_huffman_decode(stream: &mut BitStream, output: &mut [u8]) -> Option<usize> {
    let hlit = stream.read(5) as usize + 257;
    let hdist = stream.read(5) as usize + 1;
    let hclen = stream.read(4) as usize + 4;

    let mut cmd_len = [0u8; 19];
    let cmd_order: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
    
    for i in 0..hclen {
        cmd_len[cmd_order[i]] = stream.read(3) as u8;
    }

    let mut zdict = DictionaryBuilder::new(19);
    
    for &len in cmd_len.iter() {
        zdict.put(len);
    }

    let zdict = zdict.build_dictionary();
    let mut dict = DictionaryBuilder::new(hlit);
    let mut offsets = DictionaryBuilder::new(hdist);

    while !dict.is_full() || !offsets.is_full() {
        let cmd = zdict.find_in_bit_stream(stream).unwrap() as u8;

        match cmd {
            0..=15 => {
                if !dict.put(cmd) {
                    offsets.put(cmd);
                }
            },
            16 => {
                let count = match stream.read(2) {
                    0b00 => 3,
                    0b11 => 6,
                    _ => return None,
                };
                
                for _ in 0..count {
                    if !dict.put_last() {
                        offsets.put_last();
                    }
                }
            },
            17 => {
                for _ in 0..(stream.read(3) + 3) {
                    if !dict.put(0) {
                        offsets.put(0);
                    }
                }
            },
            18 => {
                for _ in 0..(stream.read(7) + 11) {
                    if !dict.put(0) {
                        offsets.put(0);
                    }
                }
            },
            _ => return None,
        }
    }

    let xdict = dict.build_dictionary();
    let xoffsets = offsets.build_dictionary();
    let mut current_pos = 0;

    loop {
        let x = xdict.find_in_bit_stream(stream)?;

        match x {
            0..=255 => {
                output[current_pos] = x as u8;
                current_pos += 1;
            },
            END_OF_DATA => return Some(current_pos),
            _ => {
                // we got cmd
                let len = huffman_read_length(x, stream);
                let offset = xoffsets.find_in_bit_stream(stream)?;
                let offset = (stream.read(YYY[offset as usize].bits as usize) + YYY[offset as usize].len) as usize;
                huffman_copy_pair(len, offset, output, &mut current_pos);
            },
        }
    }
}

pub fn decode(input: &[u8], output: &mut [u8]) -> Option<usize>
{
    let mut count = 0;

    let mut stream = BitStream {
        data: input,
        current_byte: 0,
        offset_in_byte: 1,
        order: false,
    };

    loop {
        stream.set_static_huffman_bit_order(false);
        let bfinal = stream.read(1);
        
        match stream.read(2) {
            0b00 => {
                println!("no compression");
                stream.skip_bits_to_byte_border();
                
                let len = stream.read(16);
                let _ = stream.read(16); //nlen
                
                for i in 0..(len as usize) {
                    output[count] = input[5 + i];
                    count += 1;
                }
            },
            0b01 => {
                println!("compressed with fixed Huffman codes");
                count += static_huffman_decode(&mut stream, output, count)?;
            },
            0b10 => {
                println!("compressed with dynamic Huffman codes");
                count += dynamic_huffman_decode(&mut stream, &mut output[count..])?;
            },
            _ => {
                println!("undefined compression type");
                return None;
            }
        }

        if bfinal == 1 {
            break; 
        }
    }

    Some(count)
}
/*
fn main()
{
    //let data: [u8; 11] = [0x73, 0x49, 0x4D, 0xCB, 0x49, 0x2C, 0x49, 0x55, 0x00, 0x11, 0x00];
    let data = [
        0xf3, 0xcc, 0x53, 0x28, 0xc9, 0x48, 0x55, 0xc8, 0x49, 0xcc, 
        0x4b, 0x51, 0xc8, 0x4f, 0x53, 0x70, 0xcf, 0x4f, 0x29, 0x56, 
        0x00, 0xb1, 0x7d, 0xf3, 0xf3, 0x8a, 0x4b, 0x52, 0x8b, 0x8a, 
        0xb9, 0x3c, 0x15, 0xca, 0x13, 0x41, 0x42, 0x40, 0x94, 0x9e, 
        0x9a, 0xc3, 0xe5, 0x93, 0x59, 0x96, 0x99, 0xa7, 0xae, 0x90, 
        0x09, 0xd1, 0x96, 0x9e, 0x58, 0x94, 0x92, 0x9a, 0x07, 0xd2, 
        0x98, 0x5a, 0x96, 0x99, 0xc3, 0x15, 0x9c, 0x5c, 0x94, 0x5a, 
        0x9e, 0x9a, 0xa2, 0x50, 0x5a, 0xa0, 0xa3, 0x50, 0x9c, 0x9c, 
        0x58, 0x94, 0x9a, 0xa2, 0xa3, 0x90, 0x92, 0x9f, 0x99, 0x97, 
        0x0e, 0xd4, 0x5c, 0x59, 0x92, 0x01, 0x62, 0x94, 0x64, 0x24, 
        0x96, 0x28, 0x78, 0x2a, 0xe4, 0xa5, 0xa6, 0xa6, 0xa4, 0xa6, 
        0x70, 0x05, 0x03, 0xc5, 0x40, 0xc6, 0xe5, 0x64, 0x66, 0xa7, 
        0x2a, 0x24, 0x2a, 0xa4, 0x65, 0xa6, 0x16, 0x55, 0x2a, 0x24, 
        0xa5, 0x26, 0x26, 0xe7, 0xe7, 0x01, 0x00];

    let mut output = [0u8; 200];

    if let Some(len) = decode(&data, &mut output) {
        println!("decoded data len: {}", len);

        for &c in output.iter().take(len) {
            print!("{}", c as char);
        }
    }
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    fn decode_and_compare(original: &[u8], encoded: &[u8], output: &mut [u8]) -> Result<(), String> {
        match decode(&encoded, output) {
            Some(len) => {
                if len != original.len() {
                    return Err(String::from("Output len not equal to input"))
                }

                for i in 0..original.len() {
                    if original[i] != output[i] {
                        return Err(String::from("Output not equal input"))
                    }
                }

                Ok(())
            },
            None => Err(String::from("Can't decode input"))
        }
    }

    #[test]
    fn dynamic_huffman_0() -> Result<(), String> {
        let original = b"\
            blackbird singing in the dead of night\n\
            take these broken wings and learn to fly\n\
            all your life\n\
            you were only waiting for this moment to arise\n";

        let encoded = [
            0x15, 0x8d, 0x51, 0x0a, 0xc0, 0x20, 0x0c, 0x43, 0xff, 0x3d,
            0x45, 0xae, 0x56, 0x67, 0xdd, 0x8a, 0x5d, 0x0b, 0xd5, 0x21,
            0xde, 0x7e, 0x0a, 0xf9, 0x08, 0x21, 0x2f, 0xc9, 0x4a, 0x57,
            0xcb, 0x12, 0x05, 0x5d, 0xec, 0xde, 0x82, 0x18, 0xc6, 0xc3,
            0x28, 0x4c, 0x05, 0x5e, 0x61, 0x72, 0x3f, 0x23, 0x0d, 0x6a,
            0x7c, 0xe2, 0xce, 0xc8, 0xe1, 0x8d, 0x0d, 0x73, 0x77, 0x3b,
            0xc8, 0x0a, 0x94, 0x29, 0x36, 0xe3, 0xa8, 0xba, 0x12, 0xa9,
            0x62, 0xf9, 0x17, 0x50, 0xa9, 0x9c, 0xb6, 0xc3, 0xe4, 0x60,
            0xb8, 0xe9, 0xc2, 0x24, 0x19, 0xe7, 0xa1, 0x7a, 0xec, 0x2d,
            0xe9, 0x78, 0xfd, 0x65, 0x1b, 0x07, 0xa5, 0x90, 0xce, 0xe9,
            0x07,
        ];

        let mut output = [0u8; 200];
        decode_and_compare(original, &encoded, &mut output)
    }

    #[test]
    fn dynamic_huffman_1() -> Result<(), String> {
        let original = b"\
            I saw you dancing in a crowded room\n\
            You look so happy when I'm not with you\n\
            But then you saw me, caught you by surprise\n\
            A single teardrop falling from your eye";

        let encoded = [
            0x1d, 0x8d, 0x31, 0x0e, 0xc3, 0x20, 0x10, 0x04, 0x7b, 0x5e,
            0xb1, 0x5d, 0x9a, 0x7c, 0x22, 0xe9, 0xfc, 0x84, 0x94, 0x17,
            0x73, 0x36, 0x28, 0xc0, 0xa1, 0x03, 0x84, 0xf8, 0xbd, 0x7d,
            0x69, 0x77, 0x56, 0x33, 0x1b, 0x1a, 0x4d, 0x2c, 0x19, 0xf0,
            0x54, 0xf6, 0x58, 0x4e, 0xc4, 0x02, 0xc2, 0xae, 0x32, 0x3d,
            0x7b, 0xa8, 0x48, 0x76, 0x9f, 0x9b, 0x26, 0x91, 0x1f, 0x9a,
            0x20, 0x50, 0xad, 0x0b, 0x33, 0x70, 0xc1, 0xf6, 0xc8, 0x28,
            0xd2, 0x31, 0x63, 0x0f, 0x66, 0x70, 0xef, 0xd1, 0xd1, 0x8d,
            0x98, 0xce, 0xb4, 0x99, 0x9f, 0xd8, 0x69, 0x9c, 0xa1, 0xff,
            0xa7, 0xef, 0x42, 0x1b, 0x5a, 0x35, 0x36, 0x76, 0x2f, 0xb4,
            0x3b, 0x96, 0x18, 0x9d, 0x49, 0xbd, 0x4a, 0xc5, 0x41, 0x29,
            0x59, 0xff, 0x50, 0xc9, 0x76, 0x57, 0xf0, 0xe2, 0x0b,
        ];

        let mut output = [0u8; 200];
        decode_and_compare(original, &encoded, &mut output)
    }

    #[test]
    fn static_huffman_0() -> Result<(), String> {
        let original = b"Deflate late";

        let encoded = [0x73, 0x49, 0x4D, 0xCB, 0x49, 0x2C, 0x49, 0x55, 0x00, 0x11, 0x00];

        let mut output = [0u8; 200];
        decode_and_compare(original, &encoded, &mut output)
    }

    #[test]
    fn static_huffman_1() -> Result<(), String> {
        let original = b"\
            In the land of Gods and Monsters\n\
            I was an angel\n\
            Livin' in the garden of evil\n\
            Screwed up, scared, doing anything that I needed\n\
            Shinin' like a fiery beacon";

        let encoded = [
            0xf3, 0xcc, 0x53, 0x28, 0xc9, 0x48, 0x55, 0xc8, 0x49, 0xcc, 
            0x4b, 0x51, 0xc8, 0x4f, 0x53, 0x70, 0xcf, 0x4f, 0x29, 0x56, 
            0x00, 0xb1, 0x7d, 0xf3, 0xf3, 0x8a, 0x4b, 0x52, 0x8b, 0x8a, 
            0xb9, 0x3c, 0x15, 0xca, 0x13, 0x41, 0x42, 0x40, 0x94, 0x9e, 
            0x9a, 0xc3, 0xe5, 0x93, 0x59, 0x96, 0x99, 0xa7, 0xae, 0x90, 
            0x09, 0xd1, 0x96, 0x9e, 0x58, 0x94, 0x92, 0x9a, 0x07, 0xd2, 
            0x98, 0x5a, 0x96, 0x99, 0xc3, 0x15, 0x9c, 0x5c, 0x94, 0x5a, 
            0x9e, 0x9a, 0xa2, 0x50, 0x5a, 0xa0, 0xa3, 0x50, 0x9c, 0x9c, 
            0x58, 0x94, 0x9a, 0xa2, 0xa3, 0x90, 0x92, 0x9f, 0x99, 0x97, 
            0x0e, 0xd4, 0x5c, 0x59, 0x92, 0x01, 0x62, 0x94, 0x64, 0x24, 
            0x96, 0x28, 0x78, 0x2a, 0xe4, 0xa5, 0xa6, 0xa6, 0xa4, 0xa6, 
            0x70, 0x05, 0x03, 0xc5, 0x40, 0xc6, 0xe5, 0x64, 0x66, 0xa7, 
            0x2a, 0x24, 0x2a, 0xa4, 0x65, 0xa6, 0x16, 0x55, 0x2a, 0x24, 
            0xa5, 0x26, 0x26, 0xe7, 0xe7, 0x01, 0x00];

        let mut output = [0u8; 200];
        decode_and_compare(original, &encoded, &mut output)
    }

    #[test]
    fn no_compression_0() -> Result<(), String> {
        let original = b"\
            I'd sit alone and watch your light\n\
            My only friend through teenage nights\n\
            And everything I had to know\n\
            I heard it on my radio";

        let encoded = [
            0x01, 0x7c, 0x00, 0x83, 0xff, 0x49, 0x27, 0x64, 0x20, 0x73,
            0x69, 0x74, 0x20, 0x61, 0x6c, 0x6f, 0x6e, 0x65, 0x20, 0x61,
            0x6e, 0x64, 0x20, 0x77, 0x61, 0x74, 0x63, 0x68, 0x20, 0x79,
            0x6f, 0x75, 0x72, 0x20, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x0a,
            0x4d, 0x79, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x66, 0x72,
            0x69, 0x65, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x72, 0x6f, 0x75,
            0x67, 0x68, 0x20, 0x74, 0x65, 0x65, 0x6e, 0x61, 0x67, 0x65,
            0x20, 0x6e, 0x69, 0x67, 0x68, 0x74, 0x73, 0x0a, 0x41, 0x6e,
            0x64, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x74, 0x68, 0x69,
            0x6e, 0x67, 0x20, 0x49, 0x20, 0x68, 0x61, 0x64, 0x20, 0x74,
            0x6f, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x0a, 0x49, 0x20, 0x68,
            0x65, 0x61, 0x72, 0x64, 0x20, 0x69, 0x74, 0x20, 0x6f, 0x6e,
            0x20, 0x6d, 0x79, 0x20, 0x72, 0x61, 0x64, 0x69, 0x6f];

        let mut output = [0u8; 200];
        decode_and_compare(original, &encoded, &mut output)
    }

    #[test]
    fn no_compression_1() -> Result<(), String> {
        let original = b"\
            So you think you can stone me and spit in my eye\n\
            So you think you can love me and leave me to die\n\
            Oh baby - can't do this to me baby\n\
            Just gotta get out - just gotta get right outta here";

        let encoded = [
            0x01, 0xb9, 0x00, 0x46, 0xff, 0x53, 0x6f, 0x20, 0x79, 0x6f,
            0x75, 0x20, 0x74, 0x68, 0x69, 0x6e, 0x6b, 0x20, 0x79, 0x6f,
            0x75, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x73, 0x74, 0x6f, 0x6e,
            0x65, 0x20, 0x6d, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73,
            0x70, 0x69, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x79, 0x20,
            0x65, 0x79, 0x65, 0x0a, 0x53, 0x6f, 0x20, 0x79, 0x6f, 0x75,
            0x20, 0x74, 0x68, 0x69, 0x6e, 0x6b, 0x20, 0x79, 0x6f, 0x75,
            0x20, 0x63, 0x61, 0x6e, 0x20, 0x6c, 0x6f, 0x76, 0x65, 0x20,
            0x6d, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6c, 0x65, 0x61,
            0x76, 0x65, 0x20, 0x6d, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x64,
            0x69, 0x65, 0x0a, 0x4f, 0x68, 0x20, 0x62, 0x61, 0x62, 0x79,
            0x20, 0x2d, 0x20, 0x63, 0x61, 0x6e, 0x27, 0x74, 0x20, 0x64,
            0x6f, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x6f, 0x20,
            0x6d, 0x65, 0x20, 0x62, 0x61, 0x62, 0x79, 0x0a, 0x4a, 0x75,
            0x73, 0x74, 0x20, 0x67, 0x6f, 0x74, 0x74, 0x61, 0x20, 0x67,
            0x65, 0x74, 0x20, 0x6f, 0x75, 0x74, 0x20, 0x2d, 0x20, 0x6a,
            0x75, 0x73, 0x74, 0x20, 0x67, 0x6f, 0x74, 0x74, 0x61, 0x20,
            0x67, 0x65, 0x74, 0x20, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20,
            0x6f, 0x75, 0x74, 0x74, 0x61, 0x20, 0x68, 0x65, 0x72, 0x65];

        let mut output = [0u8; 200];
        decode_and_compare(original, &encoded, &mut output)
    }
}

