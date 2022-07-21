use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Varchar;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
pub enum PageData {
    Static,
    Dynamic,
}

impl ToSql<Varchar, Pg> for PageData {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let s = match *self {
            PageData::Static => "Static",
            PageData::Dynamic => "Dynamic",
        };
        <&str as ToSql<Varchar, Pg>>::to_sql(&s, out)
    }
}

impl FromSql<Varchar, Pg> for PageData {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match <String as FromSql<Varchar, Pg>>::from_sql(bytes)?.as_ref() {
            "Static" => Ok(PageData::Static),
            "Dynamic" => Ok(PageData::Dynamic),
            _ => Ok(PageData::Dynamic),
        }
    }
}