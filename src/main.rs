mod cli;
mod proc;

fn main() {
    let args = cli::parse_arguments().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let size = proc::calculate_bulk_size(&args.path);

    println!("Path: {}, size: {} bytes", args.path.display(), size);
}
