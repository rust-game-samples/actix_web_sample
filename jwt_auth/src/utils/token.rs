use actix_web::{HttpRequest};

pub fn get_token(req: HttpRequest) -> String {
    return req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()[7..]
        .to_string();
}