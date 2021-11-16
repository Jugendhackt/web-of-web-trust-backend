use super::requests::{FetchRequest, UpdateRequest};
use crate::core::config::{CONFIG, PER_PAGE_ERROR};
use crate::core::errors::APIError;
use crate::core::types::APIResponse;
use crate::db::models::{Domain, DomainLink, SimpleDomain};
use crate::db::util::DbPool;
use crate::domains::responses::AggregatedDomainResponse;
use actix_web::web::Data;
use actix_web::{
    get, post,
    web::{Json, ServiceConfig},
    HttpResponse,
};
use lazy_static::lazy_static;
use regex::Regex;

#[get("/fetch")]
async fn fetch(data: Json<FetchRequest>, pool: Data<DbPool>) -> APIResponse {
    if data.fqdn_hash.len() < 64 {
        return Err(APIError::ValidationError(
            ["fqdn_hash"],
            "Hash prefix shouldn't be longer than 64 bytes".to_owned(),
        ));
    } else if data.per_page > CONFIG.database.domains.per_page {
        return Err(APIError::ValidationError(
            ["per_page"],
            PER_PAGE_ERROR.clone(),
        ));
    } else if !data
        .fqdn_hash
        .chars()
        .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit() || c.is_ascii_lowercase())
    {
        return Err(APIError::ValidationError(
            ["fqdn_hash"],
            "Hash Prefix may only consist of ascii characters".to_owned(),
        ));
    }

    let domains =
        SimpleDomain::by_hash(pool.as_ref(), &data.fqdn_hash, data.per_page, data.page).await?;

    match domains.len() {
        0 => Err(APIError::NotFoundError),
        _ => Ok(HttpResponse::Ok().json(AggregatedDomainResponse { domains })),
    }
}

#[post("/update")]
async fn update(data: Json<UpdateRequest>, pool: Data<DbPool>) -> APIResponse {
    // basic check if supplied source is a valid FQDN
    // see the attribution in the lazy_static reference for a complete explanation
    let regex_match_len = FQDN_REGEX.shortest_match(&data.fqdn);
    if regex_match_len.is_none() || regex_match_len.unwrap() != data.fqdn.len() {
        // If we have an invalid fqdn reject request
        return Err(APIError::ValidationError(
            ["fqdn"],
            "The supplied source is not a valid fqdn".to_owned(),
        ));
    }

    // pre-convert the pool data to a pool reference for re-usability
    let pool_ref = pool.as_ref();

    // get or create source domain entry. The grace condition handling is kinda bodged … Feel free to improve
    let source = match SimpleDomain::get_by_fqdn(pool_ref, &data.fqdn).await {
        Ok(domain) => domain,
        Err(e) => match e {
            APIError::NotFoundError => {
                match SimpleDomain::create(pool_ref, data.last_updated, &data.fqdn).await {
                    Ok(domain) => domain,
                    Err(APIError::IntegrityError) => {
                        SimpleDomain::get_by_fqdn(pool_ref, &data.fqdn).await?
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            err => {
                return Err(err);
            }
        },
    };

    // Optionally refresh last_updated value of source domain
    if source.last_updated != data.last_updated {
        source.refresh(pool_ref, data.last_updated).await?;
    }

    for link in &data.links {
        DomainLink::upsert(
            pool_ref,
            source.id,
            Domain::get_or_create(pool_ref, link, data.last_updated).await?,
        )
        .await?;
    }

    Ok(HttpResponse::Accepted().finish())
}

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(update);
    cfg.service(fetch);
}

lazy_static! {
    // Domain regex from Anton Nikiforov as published on https://stackoverflow.com/a/44534191 under CC-By-Sa 3.0 with SA being satisfied by the AGPL
    static ref FQDN_REGEX: Regex = Regex::new(r"^(?!:\/\/)(?=.{1,255}$)((.{1,63}\.){1,127}(?![0-9]*$)[a-z0-9-]+\.?)$").unwrap();
}
