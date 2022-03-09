use crate::{ar::UserEntity, error::AppError};
use sea_orm::{ConnectionTrait, DbConn, ExecResult, Schema, Statement};

pub async fn create_user_table(conn: &DbConn) -> Result<ExecResult, AppError> {
    let builder = conn.get_database_backend();
    let schema = Schema::new(builder);
    let stmt: Statement = builder.build(&schema.create_table_from_entity(UserEntity));
    Ok(conn
        .execute(stmt)
        .await
        .map_err(|_| AppError::CreatTable("user".into()))?)
}
