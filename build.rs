// Deprecated because asset copying never works correctly damnit!

/*
use std::io;
use std::fs::{copy, create_dir_all, read_dir};
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let mut maindir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("..");
    let mut outdir = Path::new(&env::var("OUT_DIR").unwrap()).join("..");
    maindir.push("data");
    for _ in 0..2 {
        outdir.push("..");
    }
    outdir.push("data");

    copydir(&maindir, &outdir).expect("Failed to copy data due to error");

    if env!("CARGO_PKG_NAME") == "server" {
        println!("Compiling for server: copying config.");
        let maindir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("config");
        outdir.pop();
        outdir.push("config");
        copydir(&maindir, &outdir).expect("Failed to copy config due to error");
    }
}

fn copydir(source: &PathBuf, target: &PathBuf) -> io::Result<()> {
    println!("Source: {:?}, target: {:?}", source, target);
    try!(create_dir_all(target));
    for entry in try!(read_dir(source)) {
        let entry = try!(entry);
        let source = entry.path();
        let target = target.join(entry.file_name());
        println!("{:?}", source);
        println!("{:?}", target);
        if try!(entry.file_type()).is_dir() {
            try!(copydir(&source, &target));
        } else {
            try!(copy(&source, &target));
        }
    }
    Ok(())
}
*/
