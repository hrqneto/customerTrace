#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use serde::Serialize;
use serde_json::json;

#[get("/clientes")]
fn listagem_clientes() -> Json<serde_json::Value> {
    let lista_clientes =
        json!([
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
        },
        {
            "id": 400,
            "nome": "kiki",
            "email": "kiki@example.com"
        },
        {
            "id": 500,
            "nome": "duda",
            "email": "duda@example.com"
        },
        {
            "id": 600,
            "nome": "lele",
            "email": "lele@example.com"
        },
    ]);

    Json(lista_clientes)
}

fn main() {
    rocket::ignite().mount("/", routes![listagem_clientes]).launch();
}
