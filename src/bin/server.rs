use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::from_utf8,
};

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0_u8; 1024];
    let num_read_bytes = stream.read(&mut buf[..])?;

    println!(
        "Read {num_read_bytes} bytes: '{}'",
        from_utf8(&buf[..num_read_bytes]).unwrap_or("utf8 conversion error")
    );

    // Respond with 'pong'.
    stream.write_all(b"pong")?;

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
