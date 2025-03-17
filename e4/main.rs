use core::panic;
use std::net::{UdpSocket, SocketAddr};
use std::{env, thread, time, process::Command};
use std::time::Duration;
use std::thread::sleep;
use std::process::Stdio;


use functions::{create_socket, string_to_bool, spawn_secondary, get_free_socket};

mod functions;
#[derive(Clone)]
struct State {
    primary: bool,
    udp_send: String,
    udp_recv: String,
    update_s: u64,
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let primary = string_to_bool(args.get(1).expect("Primary not passed"));
    let udp_recv = args.get(2).expect("No addr passed").to_string();
    println!("{}", udp_recv);
    let mut state = State {
        primary,
        udp_recv,
        udp_send: "localhost:40010".to_string(),
        update_s: 300,
    };
    


    let mut count: isize = 0;

    if !primary{
        let listening_socket = create_socket(state.udp_recv.clone());
        count = secondary_state(&listening_socket, &state);
        state.primary = true;
        drop(listening_socket);
    }

    let sending_socket = create_socket(state.udp_recv.clone());
    state.udp_send = get_free_socket();
    spawn_secondary(&state.udp_send.clone());
    primary_state(&sending_socket, state, count);

}

fn secondary_state(listening_socket: &UdpSocket, state: &State) -> isize {
    println!("Starting as secondary");
    let duration = Duration::from_millis(state.update_s);
    let mut buf = [0;10];

    let mut count: isize = 0;
    let mut attempts = 0;


    loop {
        sleep(duration);
        listening_socket.set_nonblocking(true).expect("Failed to set non-blocking");
        match listening_socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
            let received = String::from_utf8_lossy(&buf[..amt]);
            if let Ok(parsed) = received.trim().parse::<isize>() {
                count = parsed;
                println!("Secondary has received {}", count);
                attempts = 0;
            } else {
                attempts += 1;
            }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // No data to read, skip this iteration
            attempts +=1;
            }
            Err(_) => {
            attempts += 1;
            }
        }
        if attempts >= 3 {
            //drop(listening_socket);
            return count;
        }

    }



    count
}
fn primary_state(sending_socket: &UdpSocket, state: State, mut count: isize ) {
    println!("Starting as primary");
    let duration = Duration::from_millis(state.update_s);
    loop {
        count += 1;
        match sending_socket.send_to(count.to_string().as_bytes(), &state.udp_send) {
            Ok(_) => println!("Primary has sent {}", count),
            Err(e) => println!("Send error: {:?}", e),
        }
        if count % 100 == 0 {panic!("Whoa");}
        sleep(duration);
    }

}