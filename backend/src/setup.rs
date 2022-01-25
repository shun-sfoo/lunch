use sea_orm::sea_query::{self, ColumnDef, TableCreateStatement};
use sea_orm::{ConnectionTrait, DbConn, DbErr, ExecResult};

async fn create_table(db: &DbConn, stmt: &TableCreateStatement) -> Result<ExecResult, DbErr> {
    let builder = db.get_database_backend();
    db.execute(builder.build(stmt)).await
}

pub async fn create_post_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(crate::entity::post::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(crate::entity::post::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(crate::entity::post::Column::Title)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(crate::entity::post::Column::Text)
                .string()
                .not_null(),
        )
        .to_owned();

    create_table(db, &stmt).await
}

pub async fn create_user_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(crate::entity::user::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(crate::entity::user::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(crate::entity::user::Column::Username)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(crate::entity::user::Column::Password)
                .string()
                .not_null(),
        )
        .to_owned();

    create_table(db, &stmt).await
}

pub async fn create_payload_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(crate::entity::payload::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(crate::entity::payload::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(crate::entity::payload::Column::Appid)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(crate::entity::payload::Column::Name)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(crate::entity::payload::Column::IsDelta)
                .string()
                .not_null(),
        )
        .col(ColumnDef::new(crate::entity::payload::Column::MetadataSignature).string())
        .col(
            ColumnDef::new(crate::entity::payload::Column::MetadataSize)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(crate::entity::payload::Column::Sha256Hex)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(crate::entity::payload::Column::Size)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(crate::entity::payload::Column::TargetVersion)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(crate::entity::payload::Column::Version)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(crate::entity::payload::Column::Published)
                .boolean()
                .not_null(),
        )
        .col(
            ColumnDef::new(crate::entity::payload::Column::PublishAt)
                .date_time()
                .not_null(),
        )
        .to_owned();

    create_table(db, &stmt).await
}
