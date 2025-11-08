use sea_orm_migration::prelude::*;

use crate::migrations::PkceVerifiers;

#[derive(DeriveMigrationName)]
pub struct PkceVerifiersTimestampZ;

#[async_trait::async_trait]
impl MigrationTrait for PkceVerifiersTimestampZ {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PkceVerifiers::Table)
                    .modify_column(
                        ColumnDef::new(PkceVerifiers::ExpiresAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .modify_column(
                        ColumnDef::new(PkceVerifiers::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .modify_column(
                        ColumnDef::new(PkceVerifiers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PkceVerifiers::Table)
                    .modify_column(
                        ColumnDef::new(PkceVerifiers::ExpiresAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .modify_column(
                        ColumnDef::new(PkceVerifiers::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .modify_column(
                        ColumnDef::new(PkceVerifiers::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }
}
