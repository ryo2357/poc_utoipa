use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

extern crate poc_utoipa;
use poc_utoipa::todo;

#[derive(OpenApi)]
#[openapi(
    paths(
        todo::list_todos,
        todo::search_todos,
        todo::create_todo,
        todo::mark_done,
        todo::delete_todo,
    ),
    components(
        schemas(todo::Todo, todo::TodoError)
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "todo", description = "Todo items management API")
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }
}

fn main() {
    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
}
