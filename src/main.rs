use std::os::unix::net::UnixDatagram;
use std::error::Error;

const ARQUEUE: &str = "/var/ossec/queue/alerts/ar";

fn send_ar_message(message: &str) -> Result<(), Box<dyn Error>> {
    let socket = UnixDatagram::unbound()?;
    socket.send_to(message.as_bytes(), ARQUEUE)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let message = "1:) [112.211.112.221] (all) 000 Anubhav active response message";
    
    send_ar_message(message)?;
    println!("Message sent successfully");

    Ok(())
}

