# PSE #

Command-line utility for recursive size calculation of an arbitrary filesystem item (symlinks are ignored).

## How to install ##
1. Install rust (if not installed already): [Here's the guide](https://www.rust-lang.org/tools/install)
2. Run a command: `cargo install --git https://github.com/stolpa4/pse.git --profile release`

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

## Usage Example ##
Command
```
PSE_MINSIZE='50 KB' pse .
```
Output:
```
Compiling the filesystem tree for /.../pse ...
Compilation ended in 0.003 seconds
Serializing the compiled fs tree ...
Serialization ended in 0.001 seconds. JSON data was saved to /.../fs_tree.json
```
fs_tree.json:
```json
[
  {
    "type": "directory",
    "name": "pse",
    "path": "/.../pse",
    "size": "145.8330 KB",
    "contents": [
      {
        "type": "directory",
        "name": ".git",
        "path": "/.../pse/.git",
        "size": "107.3691 KB",
        "contents": [
          {
            "type": "directory",
            "name": "objects",
            "path": "/.../pse/.git/objects",
            "size": "66.0195 KB",
            "contents": []
          }
        ]
      }
    ]
  }
]
```

## Release Notes ##
### v1.0.1 ###
- Documentation - usage example
- Minor readme fix
- Execution speed log

### v1.0.0 ###
- Filesystem tree creation
- JSON output
- Packaging
- Filesystem item minimum size thresholding