use diesel::result::Error;
use dotenvy::dotenv;



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