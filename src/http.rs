use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

pub fn get_body() -> String {
    let timeout = Duration::from_secs(5);
    let mut stream = TcpStream::connect_timeout(
        &"example.com:80".to_socket_addrs().unwrap().next().unwrap(),
        timeout,
    )
    .unwrap();
    stream.set_read_timeout(Some(timeout)).unwrap();
    stream.set_write_timeout(Some(timeout)).unwrap();
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

pub fn get_ip() {
    println!(
        "{:?}",
        "ya.ru:80"
            .to_socket_addrs()
            .unwrap()
            .into_iter()
            .map(|x| { x.ip() })
            .collect::<Vec<_>>()
    );
}
