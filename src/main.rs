use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use thread_pool::ThreadPool;
use std::env;


fn main() {
    let port = match env::var("HTTPS_PLATFORM_PORT") {
        Ok(val) => val,
        Err(_) => "443".to_string(),
    };

    let listener = TcpListener::bind(port).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET mdaley.dev/ HTTP/1.1\r\n";
    let sleep = b"GET mdaley.dev/sleep HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(get)
    {
        ("HTTP/1.1 200 OK\r\n\r\n", "./html/index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5)); // for testing concurrency 
        ("HTTP/1.1 200 OK\r\n\r\n", "./html/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./html/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}