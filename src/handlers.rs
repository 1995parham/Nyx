use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{config::AppConfig, database, encryption};

#[derive(Debug, Deserialize)]
pub struct EncryptRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct EncryptResponse {
    pub key: String,
}

#[derive(Debug, Serialize)]
pub struct DecryptResponse {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn encrypt_content(
    State((pool, config)): State<(PgPool, AppConfig)>,
    Json(request): Json<EncryptRequest>,
) -> Result<Json<EncryptResponse>, (StatusCode, Json<ErrorResponse>)> {
    let key_pair = encryption::generate_key_pair(config.key_size()).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to generate key pair: {}", e),
            }),
        )
    })?;

    let encrypted_content = encryption::encrypt_content(&request.content, &key_pair.public_key)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to encrypt content: {}", e),
                }),
            )
        })?;

    let private_key_pem =
        encryption::serialize_private_key(&key_pair.private_key).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to serialize private key: {}", e),
                }),
            )
        })?;

    let content_id =
        database::create_encrypted_content(&pool, &encrypted_content, &private_key_pem)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to store encrypted content: {}", e),
                    }),
                )
            })?;

    Ok(Json(EncryptResponse {
        key: content_id.to_string(),
    }))
}

pub async fn decrypt_content(
    State((pool, _config)): State<(PgPool, AppConfig)>,
    Path(key): Path<String>,
) -> Result<Json<DecryptResponse>, (StatusCode, Json<ErrorResponse>)> {
    let content_id = Uuid::parse_str(&key).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid key format".to_string(),
            }),
        )
    })?;

    let encrypted_content = database::get_encrypted_content(&pool, content_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to retrieve encrypted content: {}", e),
                }),
            )
        })?;

    let encrypted_content = encrypted_content.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Content not found".to_string(),
            }),
        )
    })?;

    let private_key =
        encryption::deserialize_private_key(&encrypted_content.private_key).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to deserialize private key: {}", e),
                }),
            )
        })?;

    let decrypted_content =
        encryption::decrypt_content(&encrypted_content.encrypted_data, &private_key).map_err(
            |e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to decrypt content: {}", e),
                    }),
                )
            },
        )?;

    database::delete_encrypted_content(&pool, content_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to delete content: {}", e),
                }),
            )
        })?;

    Ok(Json(DecryptResponse {
        content: decrypted_content,
    }))
}
