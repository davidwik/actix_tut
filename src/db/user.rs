pub async fn has_with_email(db: &sqlx::MySqlPool, email: &str) -> bool {
    sqlx::query!("SELECT * FROM users WHERE email = ? LIMIT 1", email)
        .fetch_optional(db)
        .await
        .unwrap()
        .is_some()
}
