#[derive(Debug, Queryable, Insertable)]
use diesel::{Queryable, Insertable};
#[table_name = "addresses"]
pub struct Address {
    pub id: i32,
    pub address: String,
    pub balance: BigDecimal,
    pub transaction_count: i32,
    pub created_at: NaiveDateTime,
}

table! {
    addresses (id) {
        id -> Int4,
        address -> Varchar,
        balance -> Numeric,
        transaction_count -> Int4,
        created_at -> Timestamp,
    }
}
