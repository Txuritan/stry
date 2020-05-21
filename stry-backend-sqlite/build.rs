fn main() {
    println!("cargo:rustc-env=DATABASE_URL=sqlite:stry-backend-sqlite/schema.db");
}
