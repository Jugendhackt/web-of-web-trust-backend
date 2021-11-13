use super::util::trim_zero as trim;
use crate::{core::errors::APIError, domains::responses::DomainResponse};
use sqlx::postgres::PgQueryResult;

use super::util::DbPool;

pub struct Client;

pub struct Role;

pub struct ClientRole;

pub struct Privilege;

pub struct RolePrivilege;

pub struct Ruege {
    pub identifier: String,
    pub title: String,
    pub ziffer: String,
    pub year: i16,
}

#[derive(Debug)]
pub struct Domain {
    pub id: i32,
    pub fqdn: String,
    pub fqdn_hash: String,
    pub last_updated: i64,
}

#[derive(Debug)]
pub struct SimpleDomain {
    pub id: i32,
    pub fqdn: String,
    pub last_updated: i64,
}

pub struct DomainLink {
    pub source_id: i32,
    pub target_id: i32,
}

impl DomainLink {
    #[tracing::instrument]
    pub async fn upsert(
        pool: &DbPool,
        source_id: i32,
        target_id: i32,
    ) -> Result<PgQueryResult, APIError> {
        // TODO: Make this upsert gracefull and not such a mess
        Ok(sqlx::query!(
            r#"insert into domain_link (source_id, target_id)
               values ($1, $2)
               on conflict on constraint dl_pk
               do nothing"#,
            source_id,
            target_id
        )
        .execute(pool)
        .await?)
    }
}

impl Domain {
    #[tracing::instrument]
    /// get domain id or create new domain
    /// Is written with select and fallback to insert instead of an upserty-thing
    pub async fn get_or_create(
        pool: &DbPool,
        fqdn: &String,
        last_updated: i64,
    ) -> Result<i32, APIError> {
        match sqlx::query!(r#"select id from domains where fqdn = $1"#, fqdn)
            .fetch_optional(pool)
            .await?
        {
            Some(rec) => Ok(rec.id),
            None => Ok(sqlx::query!(
                r#"
            select last_updated, id from domains where fqdn = $1
            "#,
                fqdn
            )
            .fetch_one(pool)
            .await?.id)
        }
    }
}

impl SimpleDomain {
    #[tracing::instrument]
    pub async fn get_by_fqdn(pool: &DbPool, fqdn: &str) -> Result<SimpleDomain, APIError> {
        match sqlx::query!(
            r#"
        select last_updated, id from domains where fqdn = $1
        "#,
            fqdn
        )
        .fetch_one(pool)
        .await
        {
            Ok(rec) => Ok(SimpleDomain {
                id: rec.id,
                last_updated: rec.last_updated,
                fqdn: fqdn.to_owned(),
            }),
            Err(e) => Err(e.into()),
        }
    }

    #[tracing::instrument]
    // doesn't update internal reference to avoid mutability
    pub async fn refresh(&self, pool: &DbPool, last_updated: i64) -> Result<(), APIError> {
        match sqlx::query!(r#"update domains set last_updated = $1 where id = $2"#, last_updated, self.id).execute(pool).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }

    #[tracing::instrument]
    pub async fn create(pool: &DbPool, last_updated: i64, fqdn: &str) -> Result<Self, APIError> {
        let trimmed_fqdn = trim(fqdn);

        match sqlx::query!(
            r#"insert into domains (fqdn, fqdn_hash, last_updated) values ($1, $2, $3) returning id"#,
            trimmed_fqdn,
            SimpleDomain::hash(trimmed_fqdn.as_bytes()),
            last_updated
        ).fetch_one(pool).await {
        Ok(rec) =>             Ok(SimpleDomain {
                id: rec.id,
                fqdn: trimmed_fqdn.to_owned(),
                last_updated
            }),
        
        Err(e) => Err(e.into())
        }
    }

    #[tracing::instrument]
    // get latest_update_date by
    pub async fn upsert(
        pool: &DbPool,
        fqdn: &str,
        last_updated: i64,
    ) -> Result<SimpleDomain, APIError> {
        todo!()
    }

    // bind to blake 3 hash
    pub fn hash(fqdn: &[u8]) -> String {
        blake3::hash(fqdn).to_string()
    }

    #[tracing::instrument]
    pub async fn by_hash(
        pool: &DbPool,
        hash: &String,
        per_page: u32,
        page: u32,
    ) -> Result<Vec<DomainResponse>, APIError> {
        Ok(sqlx::query!(
            r#"select fqdn, last_updated
        from domains
        where fqdn_hash like concat($1::text, '%')
        limit $2
        offset $3"#,
            trim(hash),
            per_page as i64,
            (per_page * page) as i64
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|rec| DomainResponse {
            fqdn: rec.fqdn,
            score: [0.0, 0.0],
            last_updated: rec.last_updated,
        })
        .collect())
    }
}
