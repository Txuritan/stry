use {juniper::FieldResult, stry_backend::DataBackend};

pub struct Query;

#[juniper::graphql_object(Context = DataBackend)]
impl Query {
    fn hello(&self, ctx: &DataBackend) -> FieldResult<&str> {
        Ok("hello world")
    }
}
