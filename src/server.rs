use std::net::UdpSocket;
use std::time::SystemTime;

extern crate clap;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, about="A UDP Test Server")]
struct Args {
    /// Listening port
    #[clap(short, long, value_parser, default_value="54321")]
    port: String,

    /// Binding IP
    #[clap(long, value_parser, default_value="127.0.0.1")]
    ip: String,

    /// Checking batch
    #[clap(short, long, value_parser, default_value="100000")]
    batch: i32,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let addr = format!("{}:{}", args.ip, args.port);
    println!("Listening on {} ...", &addr);
    let socket = UdpSocket::bind(&addr).expect("Failed to bind!");

    let mut start_time = SystemTime::now();
    let mut buf: [u8; 1500] = [0; 1500];

    let mut expected_id: u32 = 0;
    let mut total_lost: usize = 0;

    let mut batch_size: usize = 0;
    let mut batch_count: i32 = 0;

    let mut first_recv = true;

    loop {
        let (size, _) = socket.recv_from(&mut buf)?;
        batch_count += 1;
        batch_size += size;

        let received_id = ((buf[3] as u32) << 24) + ((buf[2] as u32) << 16) + ((buf[1] as u32) << 8) + buf[0] as u32;

        if !first_recv && received_id != expected_id {
            println!("unexpected id: {}, expected id: {}, lost: {}", received_id, expected_id, received_id.wrapping_sub(expected_id));
            total_lost += received_id.wrapping_sub(expected_id) as usize;
        }

        if first_recv {
            first_recv = false;
        }

        expected_id = received_id.wrapping_add(1);

        if batch_count > args.batch {
            let size = batch_size as f32 / 1e6;
            let elapsed = SystemTime::now().duration_since(start_time).unwrap();
            let speed = batch_size as f64 / elapsed.as_micros() as f64;
            println!("received: {size} MB, receving bandwidth: {speed} MB/s, total_lost: {total_lost}");
            batch_count = 0;
            batch_size = 0;
            start_time = SystemTime::now();
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
