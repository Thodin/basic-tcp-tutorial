use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("Client {:?} connected", peer_addr);

    let mut buf = [0_u8; 1024];
    let mut leftover = Vec::new();

    loop {
        let num_read_bytes = stream.read(&mut buf[..])?;

        if num_read_bytes == 0 {
            println!("Client {:?} disconnected", peer_addr);
            break;
        }

        leftover.extend_from_slice(&buf[..num_read_bytes]);

        while let Some(pos) = leftover.iter().position(|&b| b == b'\n') {
            let mut line: Vec<u8> = leftover.drain(..=pos).collect();
            line.pop();
            println!(
                "Read message with {} bytes: '{}'",
                line.len(),
                String::from_utf8_lossy(&line)
            );
        }
    }

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
        }?
    }

    Ok(())
}
