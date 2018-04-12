extern crate byteorder;

use byteorder::{BigEndian, ReadBytesExt};
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("192.168.43.93:22222").unwrap();
    loop {
        let mut buf = [0; 14];
        stream.read_exact(&mut buf);

        if buf[0] == 130 {
            let len: i32 = ((buf[1] as i32) << 8) | (buf[2] as i32);

            println!("code: {:?}", buf[0]);
            println!("len: {:?}", len);

            let mut buf_angle = &buf[3..5];
            let mut buf_x = &buf[5..10];
            let mut buf_y = &buf[9..14];

            println!("buf_angle: {:?}", buf_angle);
            println!("buf_x: {:?}", buf_x);
            println!("buf_y: {:?}", buf_y);

            let angle = buf_angle.read_i16::<BigEndian>().unwrap();
            let x = buf_x.read_i32::<BigEndian>().unwrap();
            let y = buf_y.read_i32::<BigEndian>().unwrap();

            println!("{:?}", (angle as f32) / 65536.0 * 360.0);
            println!("{:?}", x);
            println!("{:?}", y);
        }
    }
}
