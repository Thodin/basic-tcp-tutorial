use std::{io::Write, net::TcpStream};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:12345")?;

    stream.write_all(b"ping\n")?;
    stream.write_all(b"another ping\n")?;

    // Optional, happens on drop anyways.
    stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}
