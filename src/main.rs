use std::{
    thread,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc
    }
};

mod resp;

use resp::RespValue;

struct ServerState {
    active_users: AtomicUsize,
}

fn handle_client(mut stream: TcpStream, user_id: usize, state: Arc<ServerState>) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                let current_count = state.active_users.fetch_sub(1, Ordering::SeqCst) - 1;
                println!("User {} connected. Active Users: {}", user_id, current_count);
                break;
            }
            Ok(_bytes_read) => {  // change bytes_read --> _bytes_read to ignore the size for now, blindly replying "OK"
                let response = RespValue::SimpleString("OK".to_string());

                // We call the function we wrote in resp.rs to get the protocol bytes (+OK\r\n) and then 
                // us object ko bytes mein convert kiya (0x2B 0x4F 0x4B 0x0D 0x0A).
                let encoded_bytes = response.encode();

                // Send Over TCP. Write those specific bytes to the stream.
                // This pushes the byte buffer into the kernel's TCP socket buffer.
                if let Err(e) = stream.write_all(&encoded_bytes)  {
                    eprintln!("Write error for user {}: {}", user_id, e);
                    break;
                }
            }
            Err(_e) => {
                let current_count = state.active_users.fetch_sub(1, Ordering::SeqCst) - 1;
                println!("User {} dropped. Active Users: {}", user_id, current_count);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let localhost = "127.0.0.1:9999";
    let listener = TcpListener::bind(localhost)?;
    println!("Concurrent Server is listening on {}", localhost);

    let state = Arc::new(ServerState {
        active_users: AtomicUsize::new(0),
    });
    let mut unique_id_counter = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                unique_id_counter += 1;
                let state_clone = Arc::clone(&state);
                let count = state.active_users.fetch_add(1, Ordering::SeqCst) + 1;

                println!("User {} connected. Total users: {}", unique_id_counter, count);

                thread::spawn(move || {handle_client(stream, unique_id_counter, state_clone);});
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}