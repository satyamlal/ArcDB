use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

struct ServerState {
    active_users: AtomicUsize,
}

fn handle_client(mut stream: TcpStream, user_id: usize, state: Arc<ServerState>) {
    let mut buffer = [0; 512]; 

    loop {
        // SYSTEM CALL: read()
        // Thread sleeps here waiting for network packets.
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Disconnect logic
                // Ordering::SeqCst ensures all threads see this change instantly.
                let current_count = state.active_users.fetch_sub(1, Ordering::SeqCst) - 1;
                println!("User {} disconnected. Active users: {}", user_id, current_count);
                break;
            }
            Ok(bytes_read) => {
                // Echo logic
                if let Err(e) = stream.write_all(&buffer[..bytes_read]) {
                    eprintln!("Write error for User {}: {}", user_id, e);
                    break;
                }
            }
            Err(_) => {
                // Unexpected error (connection reset, etc.)
                let current_count = state.active_users.fetch_sub(1, Ordering::SeqCst) - 1;
                println!("User {} dropped. Active users: {}", user_id, current_count);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Concurrent Server listening on 127.0.0.1:8080");

    // HEAP ALLOCATION: State ko Heap pe move kar rahe hain taaki threads share kar sakein.
    // (Moving State to Heap so threads can share it.)
    let state = Arc::new(ServerState {
        active_users: AtomicUsize::new(0),
    });

    // Connection Counter (Just for assigning unique IDs)
    let mut unique_id_counter = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                unique_id_counter += 1;
                
                // CLONE: Arc ka clone bana rahe hain. Data copy nahi ho raha, sirf reference count badh raha hai.
                let state_clone = Arc::clone(&state);

                // ATOMIC OP: Increment active users safely.
                let count = state.active_users.fetch_add(1, Ordering::SeqCst) + 1;
                println!("User {} connected. Active users: {}", unique_id_counter, count);

                thread::spawn(move || {
                    handle_client(stream, unique_id_counter, state_clone);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}