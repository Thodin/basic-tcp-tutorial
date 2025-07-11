use std::{io::Write, net::TcpStream};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:12345")?;

    stream.write_all(b"ping")?;
    stream.write_all(b"another ping")?;

    Ok(())
}
