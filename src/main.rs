mod models;

#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};
use serde::{Deserialize, Serialize};

#[derive(Database)]
#[database("exampleDB")]
struct Logs(sqlx::MySqlPool);

#[derive(Serialize, Deserialize)]

struct Log {
    firstName: String,
    lastName: String,
    email: String,
    password: String,
    contactNumber: String,
    dob: String,
    address: String,
    designation: String,
    reportingManager: String,
    doj: String,
    employeeCode: u32,
}
#[get("/hello")]
async fn read(mut db: Connection<Logs>) -> Option<Json<Log>> {
    println!("Endpoint /hello hit");
    sqlx::query("SELECT * FROM AdminEmployee")
        .fetch_one(&mut **db)
        .await
        .map(|row| Log {
            firstName: row.try_get("firstName").ok().unwrap_or_default(),
            lastName: row.try_get("lastName").ok().unwrap_or_default(),
            email: row.try_get("email").ok().unwrap_or_default(),
            password: row.try_get("password").ok().unwrap_or_default(),
            contactNumber: row.try_get("contactNumber").ok().unwrap_or_default(),
            dob: row.try_get("dob").ok().unwrap_or_default(),
            address: row.try_get("address").ok().unwrap_or_default(),
            designation: row.try_get("designation").ok().unwrap_or_default(),
            reportingManager: row.try_get("reportingManager").ok().unwrap_or_default(),
            doj: row.try_get("doj").ok().unwrap_or_default(),
            employeeCode: row.try_get("employeeCode").ok().unwrap_or_default(),
        })
        .map(Json)
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Logs::init())
        .mount("/", routes![read])
}
