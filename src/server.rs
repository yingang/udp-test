use std::net::UdpSocket;
use std::time::SystemTime;

extern crate clap;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, about="A UDP Test Server")]
struct Args {
    /// Binding IP:Port
    #[clap(short, long, value_parser, default_value="127.0.0.1:54321")]
    bind: String,

    /// Checking batch
    #[clap(long, value_parser, default_value="100000")]
    batch: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Listening on {} ...", &args.bind);
    let socket = UdpSocket::bind(&args.bind).expect("Failed to bind!");

    let mut start_time = SystemTime::now();
    let mut buf: [u8; 1500] = [0; 1500];

    let mut expected_id: u32 = 0;

    let mut batch_size: usize = 0;
    let mut batch_count: usize = 0;
    let mut batch_lost: usize = 0;

    let mut first_recv = true;

    loop {
        let (size, _) = socket.recv_from(&mut buf)?;
        batch_count += 1;
        batch_size += size;

        let received_id = ((buf[3] as u32) << 24) + ((buf[2] as u32) << 16) + ((buf[1] as u32) << 8) + buf[0] as u32;

        if !first_recv && received_id != expected_id {
            println!("  unexpected id: {}, expected id: {}, lost: {}", received_id, expected_id, received_id.wrapping_sub(expected_id));
            batch_lost += received_id.wrapping_sub(expected_id) as usize;
        }

        if first_recv {
            first_recv = false;
        }

        expected_id = received_id.wrapping_add(1);

        if batch_count > args.batch {
            let size = batch_size as f32 / 1e6;
            let elapsed = SystemTime::now().duration_since(start_time).unwrap();
            let speed = batch_size as f64 / elapsed.as_micros() as f64;
            let lost_rate = batch_lost as f64 * 100.0f64 / (batch_count + batch_lost) as f64;
            println!("received: {size:.2} MB, bandwidth: {speed:.2} MB/s, lost: {batch_lost} ({lost_rate:.2}%)");
            batch_count = 0;
            batch_lost = 0;
            batch_size = 0;
            start_time = SystemTime::now();
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
