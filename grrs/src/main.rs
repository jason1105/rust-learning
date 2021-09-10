use anyhow::{Context, Result};
use log::{info, warn};
use std::process::Command;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

// env RUST_LOG=info cargo run --bin output-log
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        // pb.println(format!("[+] finished #{}", i));
        thread::sleep(Duration::from_millis(10));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    info!("starting up");
    warn!("oops, nothing implemented!");

    let out = Command::new("git")
        .arg("status")
        .output()
        .expect("ls command failed to start");

    print!("{:?}", out);

    Ok(())
}
