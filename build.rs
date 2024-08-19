extern crate cc;

use std::path::Path;

fn recursion<P: AsRef<Path>>(v: &mut Vec<String>, dir: P) -> std::io::Result<()> {
    let rd = std::fs::read_dir(dir)?;
    for x in rd {
        let de = x?;
        let path = de.path();
        if path.is_dir() {
            recursion(v, path)?;
        } else {
            let path = path.into_os_string().into_string().unwrap();
            if path.ends_with(".c") {
                v.push(path);
            }
        }
    }
    Ok(())
}

fn main() {
    let mut files = Vec::new();
    recursion(&mut files, "silk/interface").unwrap();
    recursion(&mut files, "silk/src").unwrap();
    println!("cargo:rustc-link-lib=static=silk");
    println!("cargo:rerun-if-changed=wrapper.h");
    cc::Build::new()
        .includes(["silk/src", "silk/interface"])
        .files(files)
        .compile("silk");
}
