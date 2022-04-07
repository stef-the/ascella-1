use crate::{bot::commands::funny_redirect::FUNNY_WORDS, database::s3::S3};
use lazy_static::lazy_static;
use rand::prelude::SliceRandom;

use crate::prelude::*;
const ANIMALS: &str = include_str!("../../../../assets/animals.txt");
const ADJECTIVES: &str = include_str!("../../../../assets/adjectives.txt");
const ZWS_CHARS: [char; 4] = ['\u{200C}', '\u{200B}', '\u{200D}', '\u{2060}'];

/// Upload styles
/// 0 = default
/// 1 = ulid id
/// 2 = gfycat
/// 3 = zws

fn default_url() -> String {
  ran_str()
}
fn ulid_url() -> String {
  ulid::Ulid::new().to_string()
}
fn gfycat_url() -> String {
  lazy_static! {
    static ref ANIMALS_ARRAY: Vec<&'static str> = ANIMALS.lines().collect::<Vec<&str>>();
    static ref ADJECTIVES_ARRAY: Vec<&'static str> = ADJECTIVES.lines().collect::<Vec<&str>>();
  }
  let mut rng = rand::thread_rng();
  let mut s: Vec<&str> = vec![];

  for _ in 0..3 {
    s.push(ADJECTIVES_ARRAY.choose(&mut rng).unwrap());
  }
  s.push(ANIMALS_ARRAY.choose(&mut rng).unwrap());
  s.join("-")
}
fn zws_url() -> String {
  let mut rng = rand::thread_rng();
  let mut s = String::new();
  for _ in 0..8 {
    s.push(*ZWS_CHARS.choose(&mut rng).unwrap());
  }
  s
}
pub fn hacker_url() -> String {
  let mut clone = FUNNY_WORDS.clone();

  clone.shuffle(&mut rand::thread_rng());

  clone.into_iter().take(rand::thread_rng().gen_range(4..10)).collect::<Vec<&str>>().join("")
}

#[cfg(test)]
mod test_urls {
  use super::*;

  #[test]

  fn test_add() {
    println!("{}", default_url());
    println!("{}", ulid_url());
    println!("{}", gfycat_url());
    println!("{}", zws_url());
    println!("{}", hacker_url());
  }
}
/// create a image
///
/// Upload a image to ascella
#[api_v2_operation(tags(Images), consumes = "multipart/form-data", produces = "application/json")]
#[post("/upload")]
pub async fn post(mut payload: Multipart, data: AccessToken) -> Result<OkResponse<UploadSuccess>, Error> {
  if let Ok(Some(mut field)) = payload.try_next().await {
    let mut file_size: usize = 0;
    let mut buf: Vec<u8> = Vec::new();
    while let Some(chunk) = field.next().await {
      let data = chunk.map_err(|_| Error::BadRequest)?;
      file_size += data.len();

      if file_size > 1000000 {
        return Err(Error::BadRequest);
      }

      buf.append(&mut data.to_vec());
    }

    let content_type = tree_magic::from_u8(&buf);
    let s = &content_type[..];

    match s {
      //"image/png" |
      "image/png" | "image/gif" | "image/webp" => {}
      _ => return Err(Error::FileTypeNotAllowed),
    };

    let url = match data.url_style() {
      0 => default_url(),
      1 => ulid_url(),
      2 => gfycat_url(),
      3 => zws_url(),
      4 => hacker_url(),
      _ => default_url(),
    };

    let img = create_image::exec(data.id(), content_type.clone(), url).await.unwrap();

    let dest = format!("{}/{}", data.id(), img.id,);

    S3.upload_file(&content_type, dest.as_str(), buf.into()).await.map_err(|_| Error::BadRequest)?;
    // i dont want to have to do this but its neccasry
    actix_web::rt::spawn(send_text_webhook(format!(
      "**[IMAGE]** [image](<https://ascella.wtf/v2/ascella/view/{image}.png>) **[OWNER]** {name} ({id})",
      image = &img.vanity,
      name = &data.name(),
      id = &data.id()
    )));
    Ok(OkResponse(UploadSuccess::new(&img.vanity, &data.domain())))
  } else {
    Err(Error::BadRequest)
  }
}
