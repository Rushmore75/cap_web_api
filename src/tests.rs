use diesel::result::Error;
use dotenvy::dotenv;
use rocket::{serde::json::{serde_json::json, Json, self},  uri, http::Status, local::blocking::Client};

use crate::{db::BodyAccount, main, start, pages::api};



#[test]
fn dept_not_found() {
    use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
    dotenv().ok();

    use crate::{schema::dept::dsl::*, db::{Dept, establish_connection}};
    let results: Result<Dept, Error> = dept
        .filter(dept_name.eq("not_in_the_database"))
        .first(&mut establish_connection());

    assert!(results.is_err_and(|x| x == Error::NotFound)) 
}

#[test]
fn create_users() {
    let accounts = [
        BodyAccount { email: "bob@mail.com", password: "1234" },
        BodyAccount { email: "jim@mail.com", password: "1234" },
        BodyAccount { email: "ted@mail.com", password: "1234" },
        BodyAccount { email: "guy@mail.com", password: "1234" },
        BodyAccount { email: "kyle@mail.com", password: "1234" },
        BodyAccount { email: "zack@mail.com", password: "1234" },
    ];
    

    let client = Client::tracked(start()).expect("valid rocket instance");
    for acc in accounts  {
        let response = client
            .post(uri!(api::create_user))
            .body::<String>(json::to_string(&acc).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Accepted);
    }
}

#[test]
fn create_tickets() {

}