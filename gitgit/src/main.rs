use anyhow::Result;
use log::{info, warn};
use std::io;
use std::process::Command;
use std::process::Output;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init
    env_logger::init();

    // get args and check
    let args = Cli::from_args();
    let path = args.path;
    let wait_millis = 5 * 1000;

    loop {
        println!("{}", "Process started.");

        run_cmd(&path);

        println!("{}", "Taking a rest...");
        let pb = indicatif::ProgressBar::new(100);
        for i in 0..100 {
            thread::sleep(Duration::from_millis(wait_millis / 100));
            pb.inc(1);
        }

        println!("\n");
    }

    std::process::exit(0)
}

fn run_cmd(path: &std::path::PathBuf) {
    if cfg!(target_os = "windows") {
        let add_cmd = Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(&path)
            .output();
        handle_cmd_output("git add", add_cmd, "git add command failed to start");

        let commit_cmd_output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("\"auto commit\"")
            .current_dir(&path)
            .output();
        handle_cmd_output(
            "git commit",
            commit_cmd_output,
            "git commit  command failed to start",
        );

        let push_cmd = Command::new("git").arg("push").current_dir(&path).output();
        handle_cmd_output("git push", push_cmd, "git push command failed to start");
    }
}

fn handle_cmd_output(cmd: &str, output: Result<Output, io::Error>, msg: &str) {
    info!("Command \"{}\" have been called, output: {:?}", cmd, output);
    let output = output.expect(msg);

    match output.status.code() {
        Some(code) if code <= 1 => println!("Command \"{}\" success.", cmd),
        _ => {
            eprintln!(
                "Command \"{}\" failed: {}",
                cmd,
                String::from_utf8_lossy(&output.stderr)
            );
            // panic!()
        }
    }
}

/*

       .arg("&&")
       .arg("git")
       .arg("commit")
       .arg("-m")
       .arg("\"add some file\"")
*/
