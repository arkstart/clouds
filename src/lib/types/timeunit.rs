use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Varchar;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year,
}

impl ToSql<Varchar, Pg> for TimeUnit {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let s = match *self {
            TimeUnit::Second => "Second",
            TimeUnit::Minute => "Minute",
            TimeUnit::Hour => "Hour",
            TimeUnit::Day => "Day",
            TimeUnit::Month => "Month",
            TimeUnit::Year => "Year",
        };
        <&str as ToSql<Varchar, Pg>>::to_sql(&s, out)
    }
}

impl FromSql<Varchar, Pg> for TimeUnit{
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match <String as FromSql<Varchar, Pg>>::from_sql(bytes)?.as_ref() {
            "Second" => Ok(TimeUnit::Second),
            "Minute" => Ok(TimeUnit::Minute),
            "Hour" => Ok(TimeUnit::Hour),
            "Day" => Ok(TimeUnit::Day),
            "Month" => Ok(TimeUnit::Month),
            "Year" => Ok(TimeUnit::Year),
            _=> Ok(TimeUnit::Month)
        }
    }
}