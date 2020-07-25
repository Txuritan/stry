// This is ripped from juniper_warp
// I wanted to remove the dependency on failure

use {
    bytes::Bytes,
    futures::future::FutureExt,
    juniper::http::{GraphQLBatchRequest, GraphQLRequest},
    std::{collections::HashMap, sync::Arc},
    tokio::task,
    warp::{body, filters::BoxedFilter, header, http, query, Filter},
};

pub fn make_graphql_filter<Query, Mutation, Subscription, Context, S>(
    schema: juniper::RootNode<'static, Query, Mutation, Subscription, S>,
    context_extractor: BoxedFilter<(Context,)>,
) -> BoxedFilter<(http::Response<Vec<u8>>,)>
where
    S: juniper::ScalarValue + Send + Sync + 'static,
    Context: Send + Sync + 'static,
    Query: juniper::GraphQLTypeAsync<S, Context = Context> + Send + Sync + 'static,
    Query::TypeInfo: Send + Sync,
    Mutation: juniper::GraphQLTypeAsync<S, Context = Context> + Send + Sync + 'static,
    Mutation::TypeInfo: Send + Sync,
    Subscription: juniper::GraphQLSubscriptionType<S, Context = Context> + Send + Sync + 'static,
    Subscription::TypeInfo: Send + Sync,
{
    let schema = Arc::new(schema);
    let post_json_schema = schema.clone();
    let post_graphql_schema = schema.clone();

    let handle_post_json_request = move |context: Context, req: GraphQLBatchRequest<S>| {
        let schema = post_json_schema.clone();

        async move {
            let resp = req.execute(&schema, &context).await;

            Ok::<_, warp::Rejection>(build_response(
                serde_json::to_vec(&resp)
                    .map(|json| (json, resp.is_ok()))
                    .map_err(Into::into),
            ))
        }
    };

    let post_json_filter = warp::post()
        .and(header::exact_ignore_case(
            "content-type",
            "application/json",
        ))
        .and(context_extractor.clone())
        .and(body::json())
        .and_then(handle_post_json_request);

    let handle_post_graphql_request = move |context: Context, body: Bytes| {
        let schema = post_graphql_schema.clone();
        async move {
            let query = std::str::from_utf8(body.as_ref()).map_err(|e| {
                anyhow::format_err!("Request body query is not a valid UTF-8 string: {}", e)
            })?;
            let req = GraphQLRequest::new(query.into(), None, None);

            let resp = req.execute(&schema, &context).await;

            Ok((serde_json::to_vec(&resp)?, resp.is_ok()))
        }
        .then(|res| async { Ok::<_, warp::Rejection>(build_response(res)) })
    };

    let post_graphql_filter = warp::post()
        .and(header::exact_ignore_case(
            "content-type",
            "application/graphql",
        ))
        .and(context_extractor.clone())
        .and(body::bytes())
        .and_then(handle_post_graphql_request);

    let handle_get_request = move |context: Context, mut qry: HashMap<String, String>| {
        let schema = schema.clone();

        async move {
            let req = GraphQLRequest::new(
                qry.remove("query").ok_or_else(|| {
                    anyhow::format_err!("Missing GraphQL query string in query parameters")
                })?,
                qry.remove("operation_name"),
                qry.remove("variables")
                    .map(|vs| serde_json::from_str(&vs))
                    .transpose()?,
            );

            let resp = req.execute(&schema, &context).await;

            Ok((serde_json::to_vec(&resp)?, resp.is_ok()))
        }
        .then(|res| async move { Ok::<_, warp::Rejection>(build_response(res)) })
    };

    let get_filter = warp::get()
        .and(context_extractor)
        .and(query::query())
        .and_then(handle_get_request);

    get_filter
        .or(post_json_filter)
        .unify()
        .or(post_graphql_filter)
        .unify()
        .boxed()
}

#[derive(Debug)]
pub struct JoinError(task::JoinError);

impl warp::reject::Reject for JoinError {}

fn build_response(response: anyhow::Result<(Vec<u8>, bool)>) -> http::Response<Vec<u8>> {
    match response {
        Ok((body, is_ok)) => http::Response::builder()
            .status(if is_ok { 200 } else { 400 })
            .header("content-type", "application/json")
            .body(body)
            .expect("response is valid"),
        Err(_) => http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(Vec::new())
            .expect("status code is valid"),
    }
}

pub fn playground_filter(
    graphql_endpoint_url: &'static str,
    subscriptions_endpoint_url: Option<&'static str>,
) -> warp::filters::BoxedFilter<(http::Response<Vec<u8>>,)> {
    warp::any()
        .map(move || playground_response(graphql_endpoint_url, subscriptions_endpoint_url))
        .boxed()
}

fn playground_response(
    graphql_endpoint_url: &'static str,
    subscriptions_endpoint_url: Option<&'static str>,
) -> http::Response<Vec<u8>> {
    http::Response::builder()
        .header("content-type", "text/html;charset=utf-8")
        .body(
            juniper::http::playground::playground_source(
                graphql_endpoint_url,
                subscriptions_endpoint_url,
            )
            .into_bytes(),
        )
        .expect("response is valid")
}
