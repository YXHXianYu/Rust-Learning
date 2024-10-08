use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    path::PathBuf,
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_first_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match &request_first_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 400 NOT FOUND", "404.html"),
    };

    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources").join(file_name);

    let contents =  fs::read_to_string(file_path).unwrap();
    let length = contents.len();
    let response =
        format!("{status_line}\nContent-Length: {length}\n\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
