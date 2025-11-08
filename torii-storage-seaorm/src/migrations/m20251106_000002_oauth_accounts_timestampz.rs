use sea_orm_migration::prelude::*;

use crate::migrations::OauthAccounts;

#[derive(DeriveMigrationName)]
pub struct OauthAccountsTimestampZ;

#[async_trait::async_trait]
impl MigrationTrait for OauthAccountsTimestampZ {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(OauthAccounts::Table)
                    .modify_column(
                        ColumnDef::new(OauthAccounts::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .modify_column(
                        ColumnDef::new(OauthAccounts::UpdatedAt)
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
                    .table(OauthAccounts::Table)
                    .modify_column(
                        ColumnDef::new(OauthAccounts::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .modify_column(
                        ColumnDef::new(OauthAccounts::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }
}
