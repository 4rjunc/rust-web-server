use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // .unwrap will stop the 
    // incase an error occur if the port is already binded or the port reuires admin
    // previlage

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);

    //let http_request: Vec<_> = buf_reader
    //    .lines()
    //    .map(|result| result.unwrap())
    //    .take_while(|line| !line.is_empty())
    //    .collect();
    //println!("Request: {http_request:#?}");

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (statusline, filename) = if request_line == "GET / HTTP/1.1"{
        ("HTTP/1.1 200 OK", "pages/hello.html")
    } else{
        ("HTTP/1.1 404 NOT FOUND", "pages/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{statusline}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
