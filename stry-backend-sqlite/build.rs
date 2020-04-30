fn main() {
    println!("cargo:rustc-env=DATABASE_URL=sqlite:schema.db");
}
