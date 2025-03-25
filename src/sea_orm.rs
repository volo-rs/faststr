use sea_orm::prelude::StringLen;

use crate::FastStr;

impl std::convert::From<FastStr> for sea_orm::Value {
    fn from(source: FastStr) -> Self {
        sea_orm::Value::String(Some(Box::new(source.into())))
    }
}

impl sea_orm::TryFromU64 for FastStr {
    fn try_from_u64(value: u64) -> Result<Self, sea_orm::DbErr> {
        Ok(FastStr::new(itoa::Buffer::new().format(value)))
    }
}

impl sea_orm::TryGetable for FastStr {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        idx: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let val: String = String::try_get_by(res, idx)?;
        Ok(FastStr::from_string(val))
    }
}

impl sea_orm::sea_query::Nullable for FastStr {
    fn null() -> sea_orm::Value {
        sea_orm::Value::String(None)
    }
}

impl sea_orm::sea_query::ValueType for FastStr {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match v {
            sea_orm::Value::String(Some(x)) => Ok(FastStr::from_string(*x)),
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(FastStr).to_owned()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::String
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::String(StringLen::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{
        entity::prelude::*, ActiveValue::Set, DerivePrimaryKey, MockDatabase, QueryTrait,
        TryFromU64 as _,
    };

    mod test_book {
        use super::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
        #[sea_orm(table_name = "test_book")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: FastStr,
        }
        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {
            #[sea_orm(has_many = "super::test_page::Entity")]
            TestPage,
        }

        impl Related<super::test_page::Entity> for Entity {
            fn to() -> sea_orm::RelationDef {
                Relation::TestPage.def()
            }
        }

        impl ActiveModelBehavior for ActiveModel {}
    }

    mod test_page {
        use super::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
        #[sea_orm(table_name = "test_page")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: FastStr,
            pub book_id: FastStr,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {
            #[sea_orm(
                belongs_to = "super::test_book::Entity",
                from = "Column::BookId",
                to = "super::test_book::Column::Id"
            )]
            TestBook,
        }

        impl Related<super::test_book::Entity> for Entity {
            fn to() -> sea_orm::RelationDef {
                Relation::TestBook.def()
            }
        }

        impl ActiveModelBehavior for ActiveModel {}
    }

    #[test]
    fn test_value_conversion() {
        let test_str = FastStr::from_static_str("test string");

        // Test From<FastStr> for Value
        let value: sea_orm::Value = test_str.clone().into();
        assert!(matches!(value, sea_orm::Value::String(Some(_))));

        // Test null value
        let null_value = <FastStr as sea_orm::sea_query::Nullable>::null();
        assert!(matches!(null_value, sea_orm::Value::String(None)));
    }

    #[test]
    fn test_query() {
        let test_str = FastStr::from_static_str("test string");

        let db = sea_orm::DatabaseBackend::MySql;
        let select_query = test_book::Entity::find()
            .filter(test_book::Column::Id.eq(test_str.clone()))
            .build(db);
        assert_eq!(
            select_query.to_string(),
            "SELECT `test_book`.`id` FROM `test_book` WHERE `test_book`.`id` = 'test string'"
        );

        let mutate_query = test_book::Entity::insert(test_book::ActiveModel {
            id: Set(test_str.clone()),
        })
        .build(db);
        assert_eq!(
            mutate_query.to_string(),
            "INSERT INTO `test_book` (`id`) VALUES ('test string')"
        );

        let remove_query = test_book::Entity::delete(test_book::ActiveModel {
            id: Set(test_str.clone()),
        })
        .build(db);
        assert_eq!(
            remove_query.to_string(),
            "DELETE FROM `test_book` WHERE `test_book`.`id` = 'test string'"
        );
    }

    #[test]
    fn test_relation() {
        // left join
        let test_str = FastStr::from_static_str("test string");
        let db = sea_orm::DatabaseBackend::MySql;
        let select_query = test_book::Entity::find()
            .inner_join(test_page::Entity)
            .filter(test_book::Column::Id.eq(test_str))
            .build(db);
        assert_eq!(
            select_query.to_string(),
            "SELECT `test_book`.`id` FROM `test_book` INNER JOIN `test_page` ON `test_book`.`id` = `test_page`.`book_id` WHERE `test_book`.`id` = 'test string'"
        );
    }

    #[tokio::test]
    async fn test_query_result() {
        let test_str = FastStr::from_static_str("test string");
        let db = MockDatabase::new(sea_orm::DatabaseBackend::MySql)
            .append_query_results([vec![test_book::Model {
                id: test_str.clone(),
            }]])
            .into_connection();

        let result = test_book::Entity::find()
            .filter(test_book::Column::Id.eq(test_str.clone()))
            .one(&db)
            .await
            .unwrap();
        assert_eq!(result, Some(test_book::Model { id: test_str }));
    }

    #[tokio::test]
    async fn test_try_from_u64() {
        assert_eq!(
            FastStr::try_from_u64(1234567890),
            Ok(FastStr::from_static_str("1234567890"))
        );
    }
}
