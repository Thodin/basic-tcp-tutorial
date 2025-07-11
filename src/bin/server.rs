use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Client {:?} connected", stream.peer_addr()?);

    let mut buf = [0_u8; 1024];
    let num_read_bytes = stream.read(&mut buf[..])?;

    println!(
        "Read {num_read_bytes} bytes: '{}'",
        String::from_utf8_lossy(&buf[..num_read_bytes])
    );

    // Respond with 'pong'.
    stream.write_all(b"pong")?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12345")?;

    println!("TCP server listening at {}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => Err(e),
        }?
    }

    Ok(())
}
