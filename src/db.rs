use std::env;
use std::time::SystemTime;

use diesel::prelude::*;
use diesel::{PgConnection, Connection};
use serde::Deserialize;

use crate::authentication::Keyring;
use crate::schema::{account, dept, message, assignment, ticket, self};

pub fn establish_connection() -> PgConnection {

    // the env should be loaded into ram at this point, so there shouldn't be problems running this lots
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

//=======================================
//              Department
//=======================================
#[derive(Queryable)]
pub struct Dept {
    id: i32,
    dept_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::dept)]
pub struct NewDept<'a> {
    dept_name: &'a str,
}

impl Dept {
    pub fn new(name: &str) -> NewDept {
        NewDept {
            dept_name: name                       
        }
    }
    
    pub fn get_id(name: &str) -> Vec<Self> {
        use crate::schema::dept::dsl::*;

        let results: Vec<Self> = dept
            .filter(dept_name.eq(name))
            .load::<Self>(&mut establish_connection())
            .expect("Error loading departments");
        
        results 
    }
    
    pub fn get_or_create(name: &str) -> Result<Self, diesel::result::Error> {
        let find = Self::get_id(name);
        match find.into_iter().next() {
            Some(x) => Ok(x),
            None => Self::new(name).load(),
        }
    }
}

impl NewDept<'_> {
    pub fn load(&self) -> Result<Dept, diesel::result::Error> {
        let mut conn = establish_connection(); 

        let result = diesel::insert_into(dept::table)
            .values(self)
            .get_result(&mut conn);
        result
    } 
}

//=======================================
//              Account
//=======================================
#[derive(Queryable)]
pub struct Account {
    id: i32,
    email: String,
    dept: Option<i32>,
    password_hash: Vec<u8>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::account)]
pub struct NewAccount<'a> {
    email: &'a str,
    dept: i32,
    password_hash: Vec<u8>, 
}

/// [`Account`] as represented in the body of a HTTP request.
#[derive(Deserialize)]
pub struct BodyAccount<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

/// Represents accounts in the format that the database represents them.
impl Account {
    /// Create a new account, use [`NewAccount::load()`] to put into the database, and get 
    /// an identifier.
    pub fn new<'a>(email: &'a str, password: &'a str, department: Dept) -> NewAccount<'a> {
        let password_hash = Keyring::hash_string(password);
        NewAccount {
            email,
            dept: department.id,
            password_hash: password_hash.to_vec(),
        }
    }

    pub fn get<'a>(mail: &'a str) -> Option<Account> {
 
        use crate::schema::account::dsl::*;

        let results: Vec<Self> = account 
            .filter(email.eq(mail))
            .load::<Self>(&mut establish_connection())
            .expect("Error loading accounts");

        match results.into_iter().next() {
            Some(x) => Some(x),
            None => None,
        }

    }

    /// Get the specified user's password hash (if they exist).
    pub fn get_users_hash<'a>(mail: &'a str) -> Option<Vec<u8>> {
        // This has to be "mail" instead of "email" because it
        // has a field named "email", and the collide.
        match Self::get(mail) {
            Some(w) => Some(w.password_hash),
            None => None,
        }
    }
    
    fn create_ticket(&self, title: &str, body: &str) -> Result<Ticket, diesel::result::Error> {
        
        let i_title = Message::new(&self, title).load()?;
        let i_body = Message::new(&self, body).load()?;

        Ticket::new(&self, i_title, i_body).load()
    }

}

impl NewAccount<'_> {
    /// Put this new account into the database. Returns the newly placed account.
    pub fn load(&self) -> Result<Account, diesel::result::Error> {
        let mut conn = establish_connection(); 

        let result = diesel::insert_into(account::table)
            .values(self)
            .get_result(&mut conn);
        result
    } 
}

//=======================================
//              Message
//=======================================
#[derive(Queryable)]
pub struct Message {
    id: i64,
    author: i32,
    date: SystemTime,
    content: String
}

#[derive(Insertable)]
#[diesel(table_name = schema::message)]
pub struct NewMessage<'a> {
    author: i32,
    content: &'a str, 
}

/// [`Message`] as represented by the body of a HTTP request.
pub struct BodyMessage<'a> {
    content: &'a str,
}

impl Message {
    pub fn new<'a>(author: &'a Account, content: &'a str) -> NewMessage<'a> {
        NewMessage {
            author: author.id,
            content,
        }
    }
}

impl NewMessage<'_> {
    pub fn load(&self) -> Result<Message, diesel::result::Error> {
        let mut conn = establish_connection(); 

        let result = diesel::insert_into(message::table)
            .values(self)
            .get_result(&mut conn);
        result
    } 
}

//=======================================
//              Ticket
//=======================================
#[derive(Queryable)]
pub struct Ticket {
    pub id: i32,
    owner: i32,
    title: i64,
    description: i64,
}

#[derive(Insertable)]
#[diesel(table_name = schema::ticket)]
pub struct NewTicket {
    owner: i32,
    title: i64,
    description: i64,
}

/// [`Ticket`] as represented by the body of a HTTP request.
#[derive(Deserialize)]
pub struct BodyTicket<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

impl Ticket {
    pub fn new(owner: &Account, title: Message, desc: Message) -> NewTicket {
        NewTicket {
            owner: owner.id,
            title: title.id,
            description: desc.id
        }
    }
}

impl NewTicket {
    pub fn load(&self) -> Result<Ticket, diesel::result::Error>{
        let mut conn = establish_connection(); 

        let result = diesel::insert_into(ticket::table)
            .values(self)
            .get_result(&mut conn);
        result
    } 
}


