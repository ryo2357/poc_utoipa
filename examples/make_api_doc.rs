use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

extern crate poc_utoipa;
use poc_utoipa::todo;

use std::fs::{create_dir_all, File};
use std::io::{BufWriter, ErrorKind, Write};

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
    // modifiers(&SecurityAddon),
    tags(
        (name = "todo", description = "Todo items management API")
    )
)]
struct ApiDoc;

// struct SecurityAddon;

// impl Modify for SecurityAddon {
//     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
//         if let Some(components) = openapi.components.as_mut() {
//             components.add_security_scheme(
//                 "api_key",
//                 SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
//             )
//         }
//     }
// }

fn main() {
    println!("{}", ApiDoc::openapi().to_yaml().unwrap());
    output_txt_from_string(ApiDoc::openapi().to_yaml().unwrap(), "./doc/api.yml").unwrap();

    // println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
    // output_txt_from_string(
    //     ApiDoc::openapi().to_pretty_json().unwrap(),
    //     "./doc/test2.json",
    // )
    // .unwrap();
}

pub fn output_txt_from_string(string: String, path: &str) -> anyhow::Result<()> {
    let file = File::create(path);
    let file = match file {
        Ok(t) => t,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            create_folder_from_filepath(path)?;
            File::create(path).map_err(|e| anyhow::anyhow!(e))?
        }
        Err(e) => {
            return Err(anyhow::anyhow!(
                "file_operation/output_txt/File::create でエラー:{:?}",
                e
            ))
        }
    };

    let mut buf = BufWriter::new(file);
    buf.write(string.as_bytes()).unwrap();
    Ok(())
}

fn create_folder_from_filepath(path: &str) -> anyhow::Result<()> {
    let vec: Vec<&str> = path.split('/').collect();

    let mut folder_path = String::new();
    for num in 0..vec.len() - 1 {
        folder_path.push_str(vec[num]);
        folder_path.push('/');
    }
    create_dir_all(&folder_path).map_err(|e| anyhow::anyhow!(e))?;

    Ok(())
}
