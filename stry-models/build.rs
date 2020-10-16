use std::{fs, io::BufWriter, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=../schema.dal");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();

    let path = Path::new(&out_dir).join("models.rs");

    let file = fs::File::create(path).unwrap();
    let mut writer = BufWriter::new(file);

    rewryte::models_to_writer(&mut writer, "../schema.dal", Some(&["juniper", "serde"]));
}
