#[cfg(feature = "sqlx-postgres")]
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    prelude::Expr,
    sea_query::{Func, IntoCondition, Order},
    ColumnTrait, Condition, Value,
};
use serde::{Deserialize, Serialize};

mod date_range;

// pub mod pagination;

pub struct ConditionBuilder {
    condition: Condition,
}
/// Enum representing sorting directions, with serialization and deserialization
/// support.
#[derive(Debug, Deserialize, Serialize)]
pub enum SortDirection {
    #[serde(rename = "desc")]
    Desc,
    #[serde(rename = "asc")]
    Asc,
}

impl SortDirection {
    /// Returns the corresponding `Order` enum variant based on the current
    /// `SortDirection`.
    #[must_use]
    pub const fn order(&self) -> Order {
        match self {
            Self::Desc => Order::Desc,
            Self::Asc => Order::Asc,
        }
    }
}

#[must_use]
pub fn condition() -> ConditionBuilder {
    ConditionBuilder {
        condition: Condition::all(),
    }
}

#[must_use]
pub const fn with(condition: Condition) -> ConditionBuilder {
    ConditionBuilder { condition }
}

/// Builder query condition
///
/// # Examples
/// ```
/// use loco_rs::tests_cfg::db::*;
/// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
/// use loco_rs::prelude::*;
/// let date = chrono::NaiveDateTime::parse_from_str("2024-03-01 22:10:57", "%Y-%m-%d %H:%M:%S").unwrap();
///
/// let query_str = test_db::Entity::find()
///         .select_only()
///         .column(test_db::Column::Id)
///         .filter(query::condition().date_range(test_db::Column::CreatedAt).from(&date).build().like(test_db::Column::Name, "%lo").build())
///         .build(sea_orm::DatabaseBackend::Postgres)
///         .to_string();
///
///     assert_eq!(
///         query_str,
///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"created_at\" > '2024-03-01 22:10:57' AND \"loco\".\"name\" LIKE '%lo'"
///     );
/// ````
impl ConditionBuilder {
    /// where condition the given column equals the given value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().eq(test_db::Column::Id, 1).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" = 1"
    ///     );
    /// ````
    ///
    /// On string field
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().eq(test_db::Column::Name, "loco").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" = 'loco'"
    ///     );
    /// ````
    #[must_use]
    pub fn eq<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.eq(value)))
    }

    /// where condition the given column not equals the given value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().ne(test_db::Column::Id, 1).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" <> 1"
    ///     );
    /// ````
    #[must_use]
    pub fn ne<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.ne(value)))
    }

    /// where condition the given column greater than the given value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().gt(test_db::Column::Id, 1).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" > 1"
    ///     );
    /// ````
    #[must_use]
    pub fn gt<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.gt(value)))
    }

    /// where condition the given column greater than or equal to the given
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().gte(test_db::Column::Id, 1).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" >= 1"
    ///     );
    /// ````
    #[must_use]
    pub fn gte<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.gte(value)))
    }

    /// where condition the given column smaller than to the given
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().lt(test_db::Column::Id, 1).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" < 1"
    ///     );
    /// ````
    #[must_use]
    pub fn lt<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.lt(value)))
    }

    /// where condition the given column smaller than or equal to the given
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().lte(test_db::Column::Id, 1).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" <= 1"
    ///     );
    /// ````
    #[must_use]
    pub fn lte<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.lte(value)))
    }

    /// where condition the given column between the given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().between(test_db::Column::Id, 1, 2).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" BETWEEN 1 AND 2"
    ///     );
    /// ````
    #[must_use]
    pub fn between<T: ColumnTrait, V: Into<Value>>(self, col: T, a: V, b: V) -> Self {
        with(self.condition.add(col.between(a, b)))
    }

    /// where condition the given column not between the given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().not_between(test_db::Column::Id, 1, 2).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" NOT BETWEEN 1 AND 2"
    ///     );
    /// ````
    #[must_use]
    pub fn not_between<T: ColumnTrait, V: Into<Value>>(self, col: T, a: V, b: V) -> Self {
        with(self.condition.add(col.not_between(a, b)))
    }

    /// where condition the given column like given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().like(test_db::Column::Name, "%lo").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE '%lo'"
    ///     );
    /// ````
    #[must_use]
    pub fn like<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.like(a)))
    }
    /// where condition the given column ilike given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().ilike(test_db::Column::Name, "%Lo").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    /// assert_eq!(
    ///    query_str,
    ///   "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" ILIKE '%Lo'"
    /// );
    /// ```
    #[must_use]
    pub fn ilike<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        cfg_if::cfg_if! {
        if #[cfg(feature = "sqlx-postgres")] {
                with(
                    self.condition.add(
                        Expr::col((col.entity_name(), col))
                            .ilike(a)
                            .into_condition(),
                    ),
                )
            } else if #[cfg(any(feature = "sqlx-mysql", feature="sqlx-sqlite"))] {
                with(
                    self.condition.add(
                        Expr::expr(Func::lower(Expr::col(col)))
                            .like(a.into().to_lowercase())
                            .into_condition(),
                    ),
                )
            } else {
                with(
                    self.condition.add(
                        Expr::expr(Func::lower(Expr::col(col)))
                            .like(a.into().to_lowercase())
                            .into_condition(),
                    ),
                )
            }
        }
    }

    /// where condition the given column not like given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().not_like(test_db::Column::Name, "%lo").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" NOT LIKE '%lo'"
    ///     );
    /// ````
    #[must_use]
    pub fn not_like<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.not_like(a)))
    }

    /// where condition the given column start with given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().starts_with(test_db::Column::Name, "lo").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE 'lo%'"
    ///     );
    /// ````
    #[must_use]
    pub fn starts_with<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.starts_with(a)))
    }

    /// where condition the given column end with given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().ends_with(test_db::Column::Name, "lo").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE '%lo'"
    ///     );
    /// ````
    #[must_use]
    pub fn ends_with<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.ends_with(a)))
    }

    /// where condition the given column end with given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().contains(test_db::Column::Name, "lo").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE '%lo%'"
    ///     );
    /// ````
    #[must_use]
    pub fn contains<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.contains(a)))
    }

    /// where condition the given column is null
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().is_null(test_db::Column::Name).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" IS NULL"
    ///     );
    /// ````
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    pub fn is_null<T: ColumnTrait>(self, col: T) -> Self {
        with(self.condition.add(col.is_null()))
    }

    /// where condition the given column is not null
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().is_not_null(test_db::Column::Name).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" IS NOT NULL"
    ///     );
    /// ````
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    pub fn is_not_null<T: ColumnTrait>(self, col: T) -> Self {
        with(self.condition.add(col.is_not_null()))
    }

    /// where condition the given column is in
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().is_in(test_db::Column::Id, [1]).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" IN (1)"
    ///     );
    /// ````
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    pub fn is_in<T: ColumnTrait, V: Into<Value>, I: IntoIterator<Item = V>>(
        self,
        col: T,
        values: I,
    ) -> Self {
        with(self.condition.add(col.is_in(values)))
    }

    /// where condition the given column is not in
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(query::condition().is_not_in(test_db::Column::Id, [1]).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" NOT IN (1)"
    ///     );
    /// ````
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    pub fn is_not_in<T: ColumnTrait, V: Into<Value>, I: IntoIterator<Item = V>>(
        self,
        col: T,
        values: I,
    ) -> Self {
        with(self.condition.add(col.is_not_in(values)))
    }

    /// where condition the given column is not null
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::*;
    ///
    /// let from_date = chrono::NaiveDateTime::parse_from_str("2024-03-01
    /// 22:10:57", "%Y-%m-%d %H:%M:%S").unwrap(); let to_date =
    /// chrono::NaiveDateTime::parse_from_str("2024-03-25 22:10:57", "%Y-%m-%d
    /// %H:%M:%S").unwrap();
    ///
    /// let condition = query::condition()
    ///     .date_range(test_db::Column::CreatedAt)
    ///     .dates(Some(&from_date), Some(&to_date))
    ///     .build();
    ///
    /// let query_str = test_db::Entity::find()
    ///     .select_only()
    ///     .column(test_db::Column::Id)
    ///     .filter(condition.build())
    ///     .build(sea_orm::DatabaseBackend::Postgres)
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     query_str,
    ///     "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"created_at\" BETWEEN '2024-03-01 22:10:57' AND '2024-03-25 22:10:57'" );
    /// ````
    #[must_use]
    pub fn date_range<T: ColumnTrait>(self, col: T) -> date_range::DateRangeBuilder<T> {
        date_range::DateRangeBuilder::new(self, col)
    }

    #[must_use]
    pub fn build(&self) -> Condition {
        self.condition.clone().into_condition()
    }
}

#[cfg(test)]
mod tests {

    use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};

    use super::*;
    use crate::tests_cfg::db::*;

    #[test]
    fn condition_eq() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().eq(test_db::Column::Id, 1).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" = 1"
        );
    }

    #[test]
    fn condition_ne() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().ne(test_db::Column::Name, "loco").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" <> 'loco'"
        );
    }

    #[test]
    fn condition_gt() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().gt(test_db::Column::Id, 1).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" > 1"
        );
    }

    #[test]
    fn condition_gte() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().gte(test_db::Column::Id, 1).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" >= 1"
        );
    }

    #[test]
    fn condition_lt() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().lt(test_db::Column::Id, 1).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" < 1"
        );
    }

    #[test]
    fn condition_lte() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().lte(test_db::Column::Id, 1).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" <= 1"
        );
    }

    #[test]
    fn condition_between() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().between(test_db::Column::Id, 1, 2).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" BETWEEN 1 AND 2"
        );
    }

    #[test]
    fn condition_not_between() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().not_between(test_db::Column::Id, 1, 2).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" NOT BETWEEN 1 AND 2"
        );
    }

    #[test]
    fn condition_like() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().like(test_db::Column::Name, "%lo").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE '%lo'"
        );
    }

    #[test]
    #[cfg(feature = "sqlx-postgres")]
    fn postgres_condition_ilike() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().ilike(test_db::Column::Name, "%Lo").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" ILIKE '%Lo'"
        );
    }

    #[test]
    #[cfg(feature = "sqlx-mysql")]
    fn mysql_condition_ilike() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().ilike(test_db::Column::Name, "%Lo").build())
            .build(sea_orm::DatabaseBackend::MySql)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT `loco`.`id` FROM `loco` WHERE LOWER(`name`) LIKE '%lo'"
        );
    }

    #[test]
    #[cfg(feature = "sqlx-sqlite")]
    fn sqlite_condition_ilike() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().ilike(test_db::Column::Name, "%Lo").build())
            .build(sea_orm::DatabaseBackend::Sqlite)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE LOWER(\"name\") LIKE '%lo'"
        );
    }

    #[test]
    fn condition_not_like() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().not_like(test_db::Column::Name, "%lo%").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" NOT LIKE '%lo%'"
        );
    }

    #[test]
    fn condition_starts_with() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().starts_with(test_db::Column::Name, "lo").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE 'lo%'"
        );
    }

    #[test]
    fn condition_ends_with() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().ends_with(test_db::Column::Name, "lo").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE '%lo'"
        );
    }

    #[test]
    fn condition_contains() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().contains(test_db::Column::Name, "lo").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE '%lo%'"
        );
    }

    #[test]
    fn condition_is_null() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().is_null(test_db::Column::Name).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" IS NULL"
        );
    }

    #[test]
    fn condition_is_not_null() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().is_not_null(test_db::Column::Name).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" IS NOT NULL"
        );
    }

    #[test]
    fn condition_is_in() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().is_in(test_db::Column::Id, [1]).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" IN (1)"
        );
    }

    #[test]
    fn condition_is_not_in() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().is_not_in(test_db::Column::Id, [1]).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" NOT IN (1)"
        );
    }
}
