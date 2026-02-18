use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

#[derive(Debug)]
enum RespValue {
    SimpleString(String),
}

impl RespValue {
    fn serialize(&self) -> Vec<u8> {
        match self {
            RespValue::SimpleString(s) => {
                let mut bytes = Vec::new();
                bytes.push(b'+');
                bytes.extend_from_slice(s.as_bytes());
                bytes.push(b'\r');
                bytes.push(b'\n');
                
                bytes
            }
        }
    }
}

struct ServerState {
    active_users: AtomicUsize,
}

fn handle_client(mut stream: TcpStream, user_id: usize, state: Arc<ServerState>) {
    let mut buffer = [0; 512];
    
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                state.active_users.fetch_sub(1, Ordering::SeqCst);
                let current_count = state.active_users.load(Ordering::SeqCst);
                println!("User {} disconnected. Active Users: {}", user_id, current_count);
                break;
            }
            Ok(_bytes_read) => {
                let response = RespValue::SimpleString("OK".to_string());
                let encoded_response = response.serialize();
                
                if let Err(e) = stream.write_all(&encoded_response) {
                    eprintln!("Write error for user {}: {}", user_id, e);
                    break;
                }
            }
            Err(_e) => {
                state.active_users.fetch_sub(1, Ordering::SeqCst);
                println!("User {} dropped error.", user_id);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let localhost = "127.0.0.1:6379"; 
    let listener = TcpListener::bind(localhost)?;
    println!("ARCDB Server (Redis Compatible) listening on {}", localhost);

    let state = Arc::new(ServerState {
        active_users: AtomicUsize::new(0),
    });
    let mut unique_id_counter = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                unique_id_counter += 1;
                let state_clone = Arc::clone(&state);
                state.active_users.fetch_add(1, Ordering::SeqCst);
                let count = state.active_users.load(Ordering::SeqCst);

                println!("User {} connected. Total users: {}", unique_id_counter, count);

                thread::spawn(move || {
                    handle_client(stream, unique_id_counter, state_clone);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}