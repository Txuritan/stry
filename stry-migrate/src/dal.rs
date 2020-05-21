#[derive(pest_derive::Parser)]
#[grammar = "../dal.pest"]
pub struct Parser;

#[cfg(test)]
mod test {
    use {
        super::{Parser, Rule},
        pest::Parser as _,
    };

    const _EXAMPLE: &str = r#"
        table Settings {
            Id text [primary key]

            Key text [unique]
            Value text
        }
    "#;
}
