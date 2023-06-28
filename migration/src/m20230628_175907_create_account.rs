use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Account::ExternalId).uuid().extra("DEFAULT gen_random_uuid()".to_string()).not_null())
                    .col(ColumnDef::new(Account::IsActive).boolean().not_null())
                    .col(ColumnDef::new(Account::Created).date_time().extra("DEFAULT now()".to_string()).not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-account_external_id")
                    .table(Account::Table)
                    .col(Account::ExternalId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx-account_external_id").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Account {
    Table,
    Id,
    ExternalId,
    IsActive,
    Created,
}

