use crate::routes::prelude::*;

#[get("/view/{image}/stats")]
pub async fn get(image: web::Path<String>) -> Result<HttpResponse, Error> {
    let data = get_image_vanity_only::exec(image.to_string()).await;

    if let Ok(image) = data {
        if let Ok(user) = get_user::exec(image.owner).await {
            let data = if image.redirect.is_none() {
                let data = metadata(format!("./images/{}/{}", image.owner, image.id)).unwrap();
                let bytes = Byte::from_bytes(data.len().into());
                let adjusted_byte = bytes.get_appropriate_unit(false);

                Some(adjusted_byte.to_string())
            } else {
                None
            };
            let json: Value = if let Ok(embed) = get_embed::exec(user.id).await {
                serde_json::json!({
                    "user_name": user.name,
                    "user_id": user.id,
                    "id": image.id,
                    "redirect": image.redirect,
                    "content_type": image.content_type,
                    "image_size": data,
                    "embed": {
                        "color": embed.color,
                        "description": embed.description,
                        "title": embed.title,
                        "url": embed.url
                    },
                })
            } else {
                serde_json::json!({
                    "user_name": user.name,
                    "user_id": user.id,
                    "id": image.id,
                    "redirect": image.redirect,
                    "content_type": image.content_type,
                    "image_size": data,
                })
            };

            Ok(HttpResponse::Ok().json(&json))
        } else {
            Err(Error::MissingData)
        }
    } else {
        Err(Error::NotFound)
    }
}