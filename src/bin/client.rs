use std::{
    io::{Read, Write},
    net::TcpStream,
    str::from_utf8,
};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:12345")?;

    stream.write_all(b"ping")?;
    stream.write_all(b"another ping")?;

    // wait for a response
    let mut buf = [0_u8; 1024];
    let num_read_bytes = stream.read(&mut buf)?;

    println!(
        "Server responded with {num_read_bytes} bytes: '{}'",
        from_utf8(&buf[..num_read_bytes]).unwrap_or("utf8 conversion error")
    );

    Ok(())
}
