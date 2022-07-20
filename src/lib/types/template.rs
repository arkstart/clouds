use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Varchar;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
pub enum Template {
    Plan,
    Product,
}

impl ToSql<Varchar, Pg> for Template {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let s = match *self {
            Template::Plan => "Plan",
            Template::Product => "Product",
        };
        <&str as ToSql<Varchar, Pg>>::to_sql(&s, out)
    }
}

impl FromSql<Varchar, Pg> for Template {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match <String as FromSql<Varchar, Pg>>::from_sql(bytes)?.as_ref() {
            "Plan" => Ok(Template::Plan),
            "Product" => Ok(Template::Product),
            _ => Ok(Template::Plan),
        }
    }
}
