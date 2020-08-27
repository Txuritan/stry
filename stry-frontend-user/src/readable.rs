pub trait Readable: std::fmt::Display {
    fn readable(&self) -> String {
        let mut num = self.to_string();

        let values: Vec<char> = num.chars().collect();

        let mut buff = String::with_capacity(values.len() + 6);

        let negative = if values[0] == '-' {
            num.remove(0);

            true
        } else {
            false
        };

        for (i, char) in num.chars().rev().enumerate() {
            if i % 3 == 0 && i != 0 {
                buff.insert(0, ',');
            }

            buff.insert(0, char);
        }

        if negative {
            buff.insert(0, '-');
        }

        buff
    }
}

impl Readable for usize {}
impl Readable for isize {}

impl Readable for u32 {}
impl Readable for u64 {}

impl Readable for i32 {}
impl Readable for i64 {}
