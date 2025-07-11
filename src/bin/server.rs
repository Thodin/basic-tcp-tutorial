use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_connection(stream: TcpStream) -> std::io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("Client {:?} connected", peer_addr);

    let buf_reader = BufReader::new(stream);

    for line in buf_reader.lines() {
        let line = line?;
        println!("Read message with {} bytes: '{}'", line.len(), line);
    }

    println!("Client {:?} disconnected", peer_addr);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12345")?;

    println!(
        "TCP server listening at {}",
        listener
            .local_addr()
            .expect("Could not determine local address")
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => Err(e),
        }?;
    }

    Ok(())
}
