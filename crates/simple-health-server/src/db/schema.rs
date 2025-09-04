use inventory;

pub trait TableRequired {
    const CREATE_TABLE_SQL: &'static str;
    const TABLE_NAME: &'static str;
}

pub struct TableSchema {
    pub name: &'static str,
    pub sql: &'static str,
}

inventory::collect!(TableSchema);

// Macro to make registration easier
#[macro_export]
macro_rules! register_table {
    ($type:ty) => {
        inventory::submit! {
            crate::db::schema::TableSchema {
                name: <$type>::TABLE_NAME,
                sql: <$type>::CREATE_TABLE_SQL,
            }
        }
    };
}

pub fn get_all_table_schemas() -> impl Iterator<Item = &'static TableSchema> {
    inventory::iter::<TableSchema>()
}
