use crate::utils::parse_http_header;
use crate::utils::read_stream_line;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
mod utils;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let counter: Arc<RwLock<i32>> = Arc::new(RwLock::new(0));

    let producer_lock = counter.clone();

    thread::spawn(|| {
        producer(producer_lock);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let consumer_lock = counter.clone();

        thread::spawn(move || {
            if let Ok(read_guard) = consumer_lock.read() {
                println!("Consumer> {}", read_guard);
            }
            handle_connection(stream);
        });
    }
}

fn producer(producer_lock: Arc<RwLock<i32>>) {
    loop {
        thread::sleep(Duration::from_secs(2));
        if let Ok(mut write_guard) = producer_lock.write() {
            *write_guard += 1;
            println!("Producer {}", *write_guard);
            if *write_guard > 60 {
                std::process::exit(0);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let headers = parse_http_header(&mut stream);
    println!("Headers parsed {}", headers.len());

    for line in headers.iter() {
        println!("{}", String::from_utf8_lossy(&line));
    }

    loop {
        let _line = read_stream_line(&mut stream);
    }
}
