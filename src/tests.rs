use diesel::result::Error;
use dotenvy::dotenv;
use rocket::{uri, http::Status, local::blocking::Client, serde::json};

use crate::{db::BodyAccount, start, pages::api};



#[test]
fn dept_not_found() {
    /*
    Try to find a non existent department 
    */
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
    /*
    create a batch of new users 
     */
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
fn start_again() {
    /*
    Make sure that init scripts don't collied with previous runs 
     */
    start();
    start();
}