extern crate actix_web;
extern crate askama;
extern crate nom;

use std::time::Duration;

use actix_web::{App, HttpServer};
use actix_web::middleware::Compress;
use actix_web::middleware::Logger;
use env_logger;

use crate::identity::{CheckAdmin, identity_service};

pub mod highlight;
pub mod web;
pub mod identity;
pub mod pool;
pub mod post;
pub mod post_repository;
pub mod slugify;
pub mod config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let config = config::cfg();

    let pool: pool::Pool = r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(4))
        .build(pool::ConnectionManager::new(config.postgres.clone()))
        .expect("Failed to create pool");

    let listen_address = format!("{}:{}", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(identity_service(config.clone()))
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(actix_web::web::scope("/admin")
                    .wrap(CheckAdmin {})
                    .service(actix_web::web::resource("").route(actix_web::web::get().to(web::admin::index)))
                    .service(actix_web::web::scope("/drafts")
                            .service(actix_web::web::resource("").route(actix_web::web::get().to(web::admin::drafts)))
                            .service(actix_web::web::resource("/{draft_id}")
                                .route(actix_web::web::get().to(web::admin::draft))
                                .route(actix_web::web::post().to(web::admin::edit_draft)))
                            .service(actix_web::web::resource("/{draft_id}/preview").route(actix_web::web::get().to(web::admin::preview_draft)))
                            .service(actix_web::web::resource("/{draft_id}/publish").route(actix_web::web::post().to(web::admin::publish_draft)))
                            .service(actix_web::web::resource("/{draft_id}/make-public").route(actix_web::web::post().to(web::admin::make_draft_public)))
                            .service(actix_web::web::resource("/{draft_id}/delete").route(actix_web::web::post().to(web::admin::draft)))
                    )
                    .service(actix_web::web::scope("/posts")
                            .service(actix_web::web::resource("").route(actix_web::web::get().to(web::admin::posts)))
                            .service(actix_web::web::resource("/{post_id}")
                                .route(actix_web::web::get().to(web::admin::post))
                                .route(actix_web::web::post().to(web::admin::edit_post)))
                    )
            )
            .service(actix_web::web::resource("/login")
                .route(actix_web::web::get().to(web::login))
                .route(actix_web::web::post().to(web::submit_login))
            )
            .service(actix_web::web::resource("/").route(actix_web::web::get().to(web::post::index)))
            .service(actix_web::web::resource("/feed").route(actix_web::web::get().to(web::post::feed)))
            .service(actix_web::web::resource("/posts/{slug:.*}").route(actix_web::web::get().to(web::post::post_by_slug)))
            .service(actix_web::web::resource("/health").route(actix_web::web::get().to(web::health)))
            .service(actix_web::web::resource("/dist/{filename:.*}").route(actix_web::web::get().to(web::dist)))
    })
        .bind(listen_address.clone())?.run()
        .await
}