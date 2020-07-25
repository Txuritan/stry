use {crate::backend::DataBackend, juniper::FieldResult};

pub struct Mutation;

#[juniper::graphql_object(Context = DataBackend)]
impl Mutation {
    fn hello(&self, ctx: &DataBackend) -> FieldResult<&str> {
        Ok("hello world")
    }
}
