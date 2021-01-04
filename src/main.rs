/*
use std::io;
use std::io::prelude::*;
use std::fs::File;
*/

//https://tools.ietf.org/html/rfc1951
//https://fat-crocodile.livejournal.com/194295.html
//http://compression.ru/download/articles/lz/mihalchik_deflate_decoding.html
//https://www.w3.org/Graphics/PNG/RFC-1951

struct BitStream<'a> {
    data: &'a [u8],
    current_byte: usize,
    offset_in_byte: u8,
}

impl <'a> BitStream<'a> {
    fn read(&mut self, bit_count: usize) -> u8 {
        let mut res = 0u8;
        print!("read: ");

        for i in 0..bit_count {
            if (self.data[self.current_byte] & self.offset_in_byte) != 0 {
                res |= 1 << i;
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

        println!();
        res
    }
}

#[derive(Clone, Copy)]
struct Code {
    code: u16,
    len: usize,
}

fn deflate(data: &[u8]) {
    let mut stream = BitStream {
        data,
        current_byte: 0,
        offset_in_byte: 1,
    };
    /*
    for _ in 0..8 {
        if stream.read(1) == 1 {
            print!("1");
        } else {
            print!("0");
        }
    }
    */
    if stream.read(1) == 1 {
        println!("The last block");
    }

    match stream.read(2) {
        0b00 => println!("Block without compression"),
        0b01 => println!("Fixed Huffman codes"),
        0b10 => println!("Dynamic Huffman codes"),
        _ => println!("Reserved"),
    }

    let hlit = stream.read(5) as usize + 257;
    println!("hlit: {}", hlit);

    let hdist = stream.read(5) as usize + 1;
    println!("hdist: {}", hdist);

    let hclen = stream.read(4) as usize + 4;
    println!("hclen: {}", hclen);

    let cmd_code_len: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
    let mut alp_len = [Code { code: 0, len: 0}; 19];
    let mut code = 0u16;

    for i in 0..hclen {
        let len = stream.read(3) as usize;
        alp_len[cmd_code_len[i]].len = len;
    }

    for i in 2..16 {
        for x in 0..cmd_code_len.len() {
            if alp_len[x].len == i {
                alp_len[x].code = code;
                code += 1;
            }
        }

        code = code << 1;
    }

    let mut len = 0;
    let mut cmd = 0u16;

    loop {
        cmd = cmd << 1 | (stream.read(1) as u16);
        len += 1;

        for (i, c) in alp_len.iter().enumerate() {
            if c.len == len && c.code == cmd {
                println!("found cmd: {}", i);

                match i {
                    0..=16 => {
                        println!("found len");
                    },
                    17 => {
                        let _ = stream.read(3);
                    },
                    18 => {
                        let _ = stream.read(7);
                    }
                    _ => {
                        panic!();
                    },
                }
                cmd = 0;
                len = 0;
                break;
            }
        }
    }
    /*
    for i in 0..cmd_code_len.len() {
        if alp_len[i].len != 0 {
            print!("{}:", i);

            for x in 0..alp_len[i].len {
                if alp_len[i].code & (1 << (alp_len[i].len - 1 - x)) != 0 {
                    print!("1");
                } else {
                    print!("0");
                }
            }

            println!();
        }
    }

    println!();
    */
}

fn main() {
    let data: [u8; 101] = [
        0x15, 0x8d, 0x51, 0x0a, 0xc0, 0x20, 0x0c, 0x43, 0xff, 0x3d, 0x45, 0xae, 0x56, 0x67, 0xdd, 0x8a,
        0x5d, 0x0b, 0xd5, 0x21, 0xde, 0x7e, 0x0a, 0xf9, 0x08, 0x21, 0x2f, 0xc9, 0x4a, 0x57, 0xcb, 0x12,
        0x05, 0x5d, 0xec, 0xde, 0x82, 0x18, 0xc6, 0xc3, 0x28, 0x4c, 0x05, 0x5e, 0x61, 0x72, 0x3f, 0x23,
        0x0d, 0x6a, 0x7c, 0xe2, 0xce, 0xc8, 0xe1, 0x8d, 0x0d, 0x73, 0x77, 0x3b, 0xc8, 0x0a, 0x94, 0x29,
        0x36, 0xe3, 0xa8, 0xba, 0x12, 0xa9, 0x62, 0xf9, 0x17, 0x50, 0xa9, 0x9c, 0xb6, 0xc3, 0xe4, 0x60,
        0xb8, 0xe9, 0xc2, 0x24, 0x19, 0xe7, 0xa1, 0x7a, 0xec, 0x2d, 0xe9, 0x78, 0xfd, 0x65, 0x1b, 0x07,
        0xa5, 0x90, 0xce, 0xe9, 0x07,
    ];

    deflate(&data);
}