mod cli;
mod proc;
mod serialize;
mod utils;

fn main() {
    let args = cli::parse_arguments().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
    println!(
        "Compiling the filesystem tree for {} ...",
        args.path.display()
    );
    let fs_tree = proc::build_fs_tree(&args.path);
    println!("Serializing the compiled fs tree ...");
    serialize::serialize_fs_tree(&args.out_path, &fs_tree).expect("Failed to serialize fs tree");
    println!("JSON data was saved to {}", args.out_path.display());
}
