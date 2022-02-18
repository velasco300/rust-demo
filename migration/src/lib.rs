mod m20220101_000001_create_users;
mod m20220213_102936_add_age_to_users;
mod m20220213_183618_create_cars;
mod m20220216_091122_add_user_id_to_cars;
mod m30220101_000001_insert_base_data;

pub use sea_schema::migration::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users::Migration),
            Box::new(m20220213_102936_add_age_to_users::Migration),
            Box::new(m20220213_183618_create_cars::Migration),
            Box::new(m20220216_091122_add_user_id_to_cars::Migration),
            Box::new(m30220101_000001_insert_base_data::Migration),
        ]
    }
}
