pub mod default {
    use actix_web::HttpResponse;

    pub async fn not_implemented() -> HttpResponse {
        HttpResponse::NotImplemented()
            .body("Not Implemented")
    }
}
