use {
    crate::{Value, Values},
    stry_quarry::{prelude::*, Select},
};

impl<'p> Value<'p> {
    fn into_query(self) -> Select<'p> {
        match self {
            Value::Friends(_, characters) => todo!(),
            Value::Pairing(_, characters) => todo!(),
            Value::Character(_, name) => select("Story".alias("S"))
                .table("StoryCharacter".alias("SC"))
                .filter("S.Id".is_eq("SC.StoryId"))
                .filter(
                    "SC.CharacterId".is_eq(
                        select("Character")
                            .field("Id")
                            .filter("LOWER(Name)".is_like("LOWER(?)")),
                    ),
                ),
            Value::Fandom(_, name) => select("Story".alias("S")),
            Value::General(_, name) => select("Story".alias("S")),
            Value::Rating(_, rating) => select("Story")
                .fields(&["Id", "Updated"])
                .filter("Rating".is_eq(rating)),
        }
    }
}

impl<'p> Values<'p> {
    pub fn into_query(self) {
        let (and, not): (Vec<Value<'_>>, Vec<Value<'_>>) =
            self.into_iter().partition(|value| value.is_included());

        let (and_empty, and_len) = (and.is_empty(), and.len());
        let (not_empty, not_len) = (not.is_empty(), not.len());

        let query = select("Story".alias("S"));

        if !and_empty {
            for (i, value) in and.into_iter().enumerate() {
                let value_query = value.into_query();
            }
        }

        if !not_empty {
            for (i, value) in not.into_iter().enumerate() {
                let value_query = value.into_query();
            }
        }
    }
}
