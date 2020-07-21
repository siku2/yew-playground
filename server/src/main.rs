#![feature(proc_macro_hygiene, decl_macro)]

mod sandbox;

#[rocket::post("/compile")]
fn compile() -> String {
    format!("Hello!")
}

fn main() {
    rocket::ignite()
        .mount("/", rocket::routes![compile])
        .launch();
}
