mod cli;
mod proc;
mod serialize;
mod utils;

use std::path;
use std::time::Instant;

fn main() {
    let args = cli::parse_arguments().unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    let fs_tree = compile_fs_tree(&args);
    serialize_fs_tree(&args.out_path, &fs_tree);
}

fn compile_fs_tree(args: &cli::Arguments) -> proc::FsTree {
    println!(
        "Compiling the filesystem tree for {} ...",
        args.path.display()
    );
    let compile_start_time = Instant::now();
    let fs_tree = proc::build_fs_tree(&args.path, args.minsize);
    let compile_time = compile_start_time.elapsed().as_secs_f64();
    println!("Compilation ended in {:.3} seconds", compile_time);
    fs_tree
}

fn serialize_fs_tree(out_path: &path::Path, fs_tree: &proc::FsTree) {
    println!("Serializing the compiled fs tree ...");
    let serialize_start_time = Instant::now();
    serialize::serialize_fs_tree(out_path, &fs_tree).expect("Failed to serialize fs tree");
    let serialize_time = serialize_start_time.elapsed().as_secs_f64();
    println!(
        "Serialization ended in {:.3} seconds. JSON data was saved to {}",
        serialize_time,
        out_path.display()
    );
}
