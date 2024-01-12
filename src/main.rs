#![feature(str_split_remainder)]
#![feature(ascii_char)]
#![feature(slice_split_once)]

use std::io::{BufRead, Read};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("ais.plovput.hr:36602").expect("cannt connect");
    let mut buff: [u8; 512] = [0; 512];

    let sr = "alalr  3al  sa";
    println!("{:?}", sr.chars().nth(sr.find("3").unwrap()));

    loop {
        stream.read(&mut buff);

        for line in buff.lines() {
            match line {
                Ok(linex) => {
                    //println!("{}", linex.trim_start_matches('!'));
                    if linex.contains("BSVDM") {
                        // let x =  linex.trim_start_matches(',');
                        match linex.find('!') {
                            Some(x) => {
                                let y = &linex[x..];
                                 if y.len() > 14 {
                                    let data = &y[14..];
                                    println!("{}", data);
                                }

                            }
                            None => {}
                        }
                    }
                }
                Err(e) => {
                    eprint!("{}", e);
                }
            };
        }
    }
}
