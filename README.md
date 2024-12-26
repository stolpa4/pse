# PSE #

Command-line utility for recursive size calculation of an arbitrary filesystem item (symlinks are ignored).

## How to install ##
1. Install rust (if not installed already): [Here's the guide](https://www.rust-lang.org/tools/install)
2. Run a command: `cargo install --git https://github.com/stolpa4/pse.git`

## How to use ##
Command: `pse <path> [<out_json_path>]`
- `<path>` - path to a local filesystem item (directory or file)
- `<out_json_path>` - path to an output json file with filesystem tree, sorted by item sizes

### Environment variables ###

#### PSE_MINSIZE ####
Minimum size threshold for a filesystem item to be included in the output json. By default is equal to `0 bytes`.

Accepted formats: 
 - `123123 bytes` - size in bytes
 - `100 KB` - size in kilobytes
 - `10 MB` - size in megabytes
 - `1 GB` - size in gigabytes
 - `1 TB` - size in terabytes

## Release Notes ##
### v1.0.0 ###
- Filesystem tree creation
- JSON output
- Packaging
- Filesystem item minimum size thresholding