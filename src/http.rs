use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub fn get_body() -> String {
    let mut stream = TcpStream::connect("example.com:80").unwrap();
    let mut buf = String::from("");
    let req = String::from(
        "\
        GET / HTTP/1.1\r\n\
        Host: example.com\r\n\
        Connection: close\r\n\r\n\
        ",
    );
    println!("{}", req);
    stream.write_all(req.as_bytes()).unwrap();
    stream.read_to_string(&mut buf).unwrap();
    buf
}
