use nix::unistd::{fork, ForkResult, getpid};
use std::{thread, time::Duration, process};

fn main() {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Parent process (pid: {}) spawned child (pid: {}).", getpid(), child);
            // Simulate some work in the parent.
            thread::sleep(Duration::from_secs(2));
            println!("Parent process (pid: {}) is now exiting.", getpid());
            // The parent exits here. The child will be reparented (typically to init/systemd).
            process::exit(0);
        }
        Ok(ForkResult::Child) => {
            println!("Child process (pid: {}) started.", getpid());
            // Run the child process routine.
            run_child();
        }
        Err(e) => {
            eprintln!("Fork failed: {}", e);
            process::exit(1);
        }
    }
}

/// The routine for the child process.
/// In a real application, this could be a long-running service.
fn run_child() {
    // Simulate work by printing a message every second.
    for i in 1..=10 {
        println!("Child process (pid: {}) running, iteration {}.", getpid(), i);
        thread::sleep(Duration::from_secs(1));
    }
    println!("Child process (pid: {}) finished work.", getpid());
}
