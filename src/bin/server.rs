use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("Client {:?} connected", peer_addr);

    let mut buf = [0_u8; 1024];
    loop {
        let num_read_bytes = stream.read(&mut buf[..])?;

        if num_read_bytes == 0 {
            println!("Client {:?} disconnected", peer_addr);
            break;
        }

        println!(
            "Read {num_read_bytes} bytes: '{}'",
            String::from_utf8_lossy(&buf[..num_read_bytes])
        );
    }

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
