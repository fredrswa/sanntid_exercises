
use std::net::UdpSocket;
use std::process::{Stdio, Command};

pub fn string_to_bool(s: &str) -> bool {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" => true,
        "false" | "0" | "no" => false,
        _ => panic!("Invalid boolean string: {}", s),
    }
}

pub fn create_socket(addr: String) -> UdpSocket {
    let socket = UdpSocket::bind(addr).expect("Could'nt setup receiver");
    socket
}

pub fn get_free_socket() -> String  {
    let mut port = 40000;
    loop {
        let addr = format!("localhost:{}", port);
        if UdpSocket::bind(&addr).is_ok() {
            let udp_send = format!("localhost:{}", port).to_string();
            return udp_send;
        }
        port += 1;
    }
}

pub fn spawn_secondary(udp_send: &str) {
    let secondary = Command::new("setsid")
        .arg("xterm")
        .arg("-e")
        .arg("cargo")
        .arg("run")
        .arg("false")
        .arg(udp_send) // Pass recv address as argument
        .stdout(Stdio::null())  // Avoid blocking by suppressing stdout
        .stderr(Stdio::null())  // Suppress stderr
        .spawn()
        .expect("Failed to start secondary process in new xterm terminal");
    println!("Secondary spawned in a new xterm window with recv address: {}", udp_send);
}
