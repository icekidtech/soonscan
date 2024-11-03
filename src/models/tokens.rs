use diesel::{Queryable, Insertable};
#[derive(Debug, Queryable, Insertable)]
#[table_name = "tokens"]
pub struct Token {
    pub id: i32,
    pub token_address: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: i32,
    pub total_supply: BigDecimal,
    pub created_at: NaiveDateTime,
}

table! {
    tokens (id) {
        id -> Int4,
        token_address -> Varchar,
        name -> Nullable<Varchar>,
        symbol -> Nullable<Varchar>,
        decimals -> Int4,
        total_supply -> Numeric,
        created_at -> Timestamp,
    }
}
