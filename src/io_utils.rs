use std::io::Read;
use std::string::FromUtf8Error;

pub fn read_line(stream: &mut std::net::TcpStream) -> Result<String, FromUtf8Error> {
    let mut buffer = [0u8; 100];
    let read = stream.read(&mut buffer).expect("read");
    buffer[read] = '\0' as u8;
    let v = Vec::from(&buffer[0..read]);
    let s = String::from_utf8(v);

    if let Ok(l) = s {
        let l = l.replace("\r\n", "").replace("\n", "");
        return Ok(l);
    }
    s
}
