use serde::Serialize;

#[derive(Serialize)]
pub struct Credentials<'a> {
    name: &'a str,
    pass: &'a str,
}