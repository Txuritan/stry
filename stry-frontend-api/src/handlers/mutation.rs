use {juniper::FieldResult, stry_backend::DataBackend};

pub struct Mutation;

#[juniper::graphql_object(Context = DataBackend)]
impl Mutation {
    fn hello(&self, ctx: &DataBackend) -> FieldResult<&str> {
        Ok("hello world")
    }
}
