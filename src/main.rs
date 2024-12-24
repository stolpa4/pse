mod cli;
mod proc;
mod utils;

fn main() {
    let args = cli::parse_arguments().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let size = proc::calculate_bulk_size(&args.path);

    println!(
        "Path: {}, size: {}",
        args.path.display(),
        utils::size_to_label(size)
    );
}
