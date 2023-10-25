#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;
extern crate serde_json;

use rocket::http::Method;
use rocket::response::content;

// Endpoint para a lista de clientes
#[get("/clientes")]
fn listagem_clientes() -> content::Json<&'static str> {
    let lista_clientes =
        r#"[
        {
            "id": 100,
            "nome": "Enzo Daniel",
            "email": "enzo@example.com"
        },
        {
            "id": 200,
            "nome": "Dani",
            "email": "dani@example.com"
        },
        {
            "id": 300,
            "nome": "Jess",
            "email": "jess@example.com"
        }
    ]"#;

    content::Json(lista_clientes)
}

fn main() {
    rocket::ignite().mount("/", routes![listagem_clientes]).launch();
}
