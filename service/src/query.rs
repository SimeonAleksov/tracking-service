use ::entity::{account, account::Entity as Account};
use sea_orm::*;
use serde_json;
use uuid::Uuid;


pub struct Query;

impl Query {
    pub async fn find_accounts_by_id(db: &DbConn, id: &str) -> Option<serde_json::value::Value> {
        let external_id = Uuid::parse_str(&*id.to_string()).unwrap();
        Account::find()
            .select_only()
            .columns([
                account::Column::ExternalId,
                account::Column::FirstName,
                account::Column::LastName,
                account::Column::Created,
                account::Column::IsActive]
            )
            .filter(account::Column::ExternalId.eq(external_id))
            .into_json()
            .one(db)
            .await.ok()?
    }
}
