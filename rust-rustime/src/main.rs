use std::process::{Command, Stdio};
use std::time::Instant;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args...]", args[0]);
        return;
    }

    let command = &args[1];
    let args = &args[2..];

    let start_time = Instant::now();

    let output = Command::new(command)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");

    let elapsed = start_time.elapsed();

    let exit_status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("Elapsed time: {:?}", elapsed);
    println!("Exit status: {}", exit_status);
    if !stdout.is_empty() {
        println!("Standard output:\n{}", stdout);
    }
    if !stderr.is_empty() {
        println!("Standard error:\n{}", stderr);
    }
}
