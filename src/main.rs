mod cli;
mod proc;
mod serialize;
mod utils;

fn main() {
    let args = cli::parse_arguments().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let fs_tree = proc::build_fs_tree(&args.path);
    serialize::serialize_fs_tree(&fs_tree).expect("Failed to serialize fs tree");
}
