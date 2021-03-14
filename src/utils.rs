use std::io::prelude::*;
use std::net::TcpStream;

pub fn read_stream_line(stream: &mut TcpStream) -> Vec<u8> {
    let mut line_buffer = Vec::new();
    let mut character = [0 as u8; 1];
    let mut last_char = [0 as u8; 1];

    loop {
        stream.read_exact(&mut character).unwrap();
        line_buffer.push(character[0]);
        if last_char[0] == '\r' as u8 && character[0] == '\n' as u8 {
            return line_buffer;
        }
        last_char[0] = character[0];
    }
}

pub fn parse_http_header(mut stream: &mut TcpStream) -> Vec<Vec<u8>> {
    let mut header: Vec<Vec<u8>> = Vec::new();
    loop {
        let line = read_stream_line(&mut stream);
        if line.len() == 2 && line[0] == '\r' as u8 && line[1] == '\n' as u8 {
            return header;
        }
        header.push(line);
    }
}
