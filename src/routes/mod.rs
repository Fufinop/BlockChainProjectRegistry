pub mod blockchain;
pub mod projects;

#[get("/")]
pub fn index() -> &'static str {
    "¡Bienvenido a la API de Blockchain! 🚀"
}
