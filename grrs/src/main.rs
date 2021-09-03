use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let result = std::fs::read_to_string(&args.path);

    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Can't deal with {}, just exit here", error);
        }
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
