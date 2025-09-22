use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedContent {
    pub id: Uuid,
    pub encrypted_data: String,
    pub private_key: String,
    pub created_at: DateTime<Utc>,
}

pub async fn create_encrypted_content(
    pool: &PgPool,
    encrypted_data: &str,
    private_key: &str,
) -> Result<Uuid> {
    let row = sqlx::query(
        "INSERT INTO encrypted_content (encrypted_data, private_key) VALUES ($1, $2) RETURNING id",
    )
    .bind(encrypted_data)
    .bind(private_key)
    .fetch_one(pool)
    .await
    .context("Failed to insert encrypted content")?;

    let id: Uuid = row.get("id");
    Ok(id)
}

pub async fn get_encrypted_content(pool: &PgPool, id: Uuid) -> Result<Option<EncryptedContent>> {
    let row = sqlx::query_as!(
        EncryptedContent,
        "SELECT id, encrypted_data, private_key, created_at FROM encrypted_content WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
    .context("Failed to fetch encrypted content")?;

    Ok(row)
}

pub async fn delete_encrypted_content(pool: &PgPool, id: Uuid) -> Result<bool> {
    let result = sqlx::query("DELETE FROM encrypted_content WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .context("Failed to delete encrypted content")?;

    Ok(result.rows_affected() > 0)
}