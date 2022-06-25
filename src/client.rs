use std::net::UdpSocket;
use std::time::SystemTime;

extern crate clap;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, about="A UDP Test Client")]
struct Args {
    /// Server port
    #[clap(short, long, value_parser, default_value="54321")]
    port: String,

    /// Server IP
    #[clap(long, value_parser, default_value="127.0.0.1")]
    ip: String,

    /// Sending interval in micro seconds
    #[clap(short, long, value_parser, default_value="8")]
    interval: u128,
}

fn busy_wait(expected_us: u128) {
    let start = SystemTime::now();
    loop {
        let actual_us = SystemTime::now().duration_since(start).unwrap().as_micros();
        if actual_us > expected_us {
            break;
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let socket = UdpSocket::bind("127.0.0.1:53135").expect("failed to bind!");
    let addr = format!("{}:{}", args.ip, args.port);
    println!("Connecting to {} ...", &addr);
    socket.connect(&addr)?;
 
    let mut id: u32 = 0;
    let mut buf: [u8; 1450] = [0; 1450];
    loop {
        buf[0] = id as u8;
        buf[1] = (id >> 8) as u8;
        buf[2] = (id >> 16) as u8;
        buf[3] = (id >> 24) as u8;
        socket.send(&buf)?;
        id = id.wrapping_add(1);
        busy_wait(args.interval);
   }
}
