fn main() {
    println!("Hello, world!");
}



fn word_count(str: &str) -> u32 {
    str.split_whitespace()
        .filter(|s| match *s {
            "---" => false,
            "#" | "##" | "###" | "####" | "#####" | "######" => false,
            "*" | "**" => false,
            _ => true,
        })
        .count() as u32
}