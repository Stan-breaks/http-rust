use std::{
    io::{Result, Write, copy, stdout},
    net::TcpStream,
};

fn main() -> Result<()> {
    let host = "localhost:80";
    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?;

    conn.write_all(b"Host: localhost")?;
    conn.write_all(b"\r\n")?;
    copy(&mut conn, &mut stdout())?;
    Ok(())
}
