use std::io::BufRead;
use std::io::BufReader;

pub fn read_line(stream: &mut std::net::TcpStream) -> std::io::Result<String> {
    let mut buffer = BufReader::new(stream);

    let mut line = String::new();
    let len = buffer.read_line(&mut line)?;

    println!("Read {} bytes; line: '{}'", len, line);
    let line = line.replace("\r\n", "").replace("\n", "");
    println!("Improved line: '{}'", line);
    Ok(line)
}
