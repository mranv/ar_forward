use std::os::unix::net::UnixDatagram;
use std::error::Error;
use serde_json::json;

const ARQUEUE: &str = "/var/ossec/queue/alerts/ar";

// We create a temporary file path for the client socket using the process ID to ensure uniqueness.
// We bind the UnixDatagram to this temporary path instead of using an unbound socket.
// After sending the message, we clean up by removing the temporary file.


fn send_ar_message(message: &[u8]) -> Result<(), Box<dyn Error>> {
    let client_socket_path = format!("/tmp/ar_client_{}", std::process::id());
    let socket = UnixDatagram::bind(&client_socket_path)?;
    
    socket.send_to(message, ARQUEUE)?;
    
    std::fs::remove_file(client_socket_path)?;
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let json_data = json!({
        "command": "quick-scan0",
        "origin": {
            "module": "API",
            "name": null
        },
        "parameters": {
            "alert": {},
            "extra_args": []
        },
        "version": 1
    });

    let message = format!("(msg_to_agent) [] NNS 003 {}", json_data.to_string());
    let message_bytes = message.as_bytes();

    send_ar_message(message_bytes)?;
    println!("Message sent successfully");
    Ok(())
}