use actix_web::Responder;

#[actix_web::get("/todos")]
pub async fn get_todos() -> impl Responder {
    "Get Todos"
}
