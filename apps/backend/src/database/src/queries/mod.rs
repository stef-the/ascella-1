pub mod claim_code;
pub mod create_code;
pub mod create_image;
pub mod create_paste;
pub mod create_redirect;
pub mod create_user;
pub mod delete_image;
pub mod get_all_users;
pub mod get_codes;
pub mod get_discord_id;
pub mod get_domains;
pub mod get_embed;
pub mod get_image;
pub mod get_image_owner;
pub mod get_image_vanity;
pub mod get_image_vanity_only;
pub mod get_images;
pub mod get_latest_image;
pub mod get_paste_content;
pub mod get_unclaimed_code;
pub mod get_user;
pub mod get_user_discord;
pub mod get_user_image_count;
pub mod get_user_token;
pub mod make_public;
pub mod set_domain;
pub mod set_domain_discord;
pub mod set_embed;
pub mod set_emojis;

pub mod prelude {
    pub use crate::structs::*;
    pub use crate::*;
    pub use anyhow::anyhow;
    pub use anyhow::Result;
    pub use cached::{proc_macro::cached, Cached, CachedAsync};
    pub use tokio_pg_mapper::FromTokioPostgresRow;
}