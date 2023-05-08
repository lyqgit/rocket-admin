use crate::model::common::CustomResponse;
use rocket::serde::json::Json;

pub type ResError = Json<CustomResponse<()>>;

