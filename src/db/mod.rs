use diesel::{prelude::*, sqlite::SqliteConnection};
pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task<'a>(connection: &SqliteConnection, title: &'a str) {
    let task = models::NewTask { title };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("Error loading tasks")
}


pub fn delete_task(connection: &SqliteConnection, id: &i32) -> usize {

    diesel::delete(schema::task::table.filter(schema::task::id.eq(id))).execute(connection)
        .expect("Failed to clean up photos")
}