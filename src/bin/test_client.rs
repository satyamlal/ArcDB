use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9999")?;
    println!("Connected to server");

    // Sending random data, and ignoring parsing for now
    let test_data = b"PING\r\n";
    stream.write_all(test_data)?;
    println!("Sent: {:?}", test_data);

    // Reading the response
    let mut buffer = [0u8; 512];
    let bytes_read = stream.read(&mut buffer)?;
    
    // Convert the byte to String
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received: {:?}", response);
    println!("Received bytes: {:?}", &buffer[..bytes_read]);

    Ok(())
}