use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Varchar;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
pub enum Currency {
    USD,
    IDR,
}

impl ToSql<Varchar, Pg> for Currency {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let s = match *self {
            Currency::USD => "USD",
            Currency::IDR => "IDR",
        };
        <&str as ToSql<Varchar, Pg>>::to_sql(&s, out)
    }
}

impl FromSql<Varchar, Pg> for Currency {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match <String as FromSql<Varchar, Pg>>::from_sql(bytes)?.as_ref() {
            "US$" => Ok(Currency::USD),
            "IDR" => Ok(Currency::IDR),
            _ => Ok(Currency::USD),
        }
    }
}
