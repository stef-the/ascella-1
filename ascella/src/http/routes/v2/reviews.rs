// TODO: type this result

use crate::bot::bot::REVIEWS;
use crate::prelude::*;

#[api_v2_operation(tags(Etc), summary = "get reviews", description = "Get ascella reviews!", consumes = "application/json", produces = "application/json")]
#[get("/reviews")]
pub async fn get() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().json(REVIEWS.get().unwrap()))
}