use std::env;
use std::fmt::Display;
use std::time::SystemTime;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::{PgConnection, Connection};
use rocket::FromForm;
use serde::{Deserialize, Serialize};

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
#[derive(Queryable, Debug)]
pub struct Dept {
    pub id: i32,
    dept_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::dept)]
pub struct NewDept<'a> {
    dept_name: &'a str,
}

#[derive(Debug)]
pub enum Departments {
    Client,
    Flunky,
    Supervisor,
}

impl Departments {
    fn as_string(&self) -> &'static str {
        match self {
            Departments::Client => "client",
            Departments::Flunky => "flunky",
            Departments::Supervisor => "supervisor",
        }
    }
}

impl Display for Departments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string()) 
    }
}


impl Dept {
    pub fn new(name: &Departments) -> NewDept<'static> {
        NewDept {
            dept_name: name.as_string()
        }
    }
    
    pub fn get_id(name: &Departments) -> Result<Self, Error> {
        use crate::schema::dept::dsl::*;

        let results: Result<Self, Error> = dept
            .filter(dept_name.eq(name.as_string()))
            .first(&mut establish_connection());
        
        results 
    }
    
    pub fn get_or_create(name: &Departments) -> Result<Self, diesel::result::Error> {
        let find = Self::get_id(name);
        match find {
            Ok(x) => Ok(x),
            Err(e) => {
                match e {
                    Error::NotFound => Self::new(name).load(),
                    _ => Err(e),
                }
            }
        }
    }
    
    pub fn get_department_name(&self) -> Departments {
        match self.dept_name.as_str() {
           "flunky" => Departments::Flunky,
           "supervisor" => Departments::Supervisor,
           _ => Departments::Client,
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
#[derive(Deserialize, FromForm)]
pub struct BodyAccount<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

/// For representing [`Account`] in instances where you aren't the owner of the account.
pub struct GenericBodyAccount {
    /// Accounts are verified to exist during deserialization.
    pub account: Account,
}

impl<'de> Deserialize<'de> for GenericBodyAccount {
    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>
    {
        let email: &str =  Deserialize::deserialize(deserializer)?;
        if let Ok(account) = Account::get(email) {
            return Ok(Self {account});
        }
        Err(serde::de::Error::custom(format!("Account not found for email \"{}\".", email)))
    }
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

    pub fn get<'a>(mail: &'a str) -> Result<Account, Error> {
 
        use crate::schema::account::dsl::*;

        let results: Result<Self, Error> = account 
            .filter(email.eq(mail))
            .first(&mut establish_connection());
        
        results
    }

    /// Get the specified user's password hash (if they exist).
    pub fn get_users_hash<'a>(mail: &'a str) -> Option<Vec<u8>> {
        // This has to be "mail" instead of "email" because it
        // has a field named "email", and the collide.
        match Self::get(mail) {
            Ok(w) => Some(w.password_hash),
            Err(_) => None,
        }
    }
    
    fn create_ticket(&self, title: &str, body: &str) -> Result<Ticket, diesel::result::Error> {
        
        let i_title = Message::new(&self, title).load()?;
        let i_body = Message::new(&self, body).load()?;

        Ticket::new(&self, i_title, i_body).load()
    }
    
    pub fn get_dept(&self) -> Result<Dept, Error> {
         
        use crate::schema::dept::dsl::*;

        let results: Result<Dept, Error> = dept 
            .filter(id.eq(self.id))
            .first(&mut establish_connection());
        results
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
    pub content: String
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
    pub fn get(find_id: i64) -> Result<Self, Error> {
        use crate::schema::message::dsl::*;

        let results: Result<Self, Error> = message 
            .filter(id.eq(find_id))
            .first(&mut establish_connection()); 
        results       
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
#[derive(Queryable, Serialize)]
pub struct Ticket {
    pub id: i32,
    pub owner: i32,
    pub title: i64,
    pub description: i64,
}

#[derive(Insertable)]
#[diesel(table_name = schema::ticket)]
pub struct NewTicket {
    owner: i32,
    title: i64,
    description: i64,
}

/// [`Ticket`] as represented by the body of a HTTP request.
#[derive(FromForm)]
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
    
    pub fn get(find_id: i32) -> Result<Ticket, Error> {
        use crate::schema::ticket::dsl::*;

        let results: Result<Self, Error> = ticket 
            .filter(id.eq(find_id))
            .first(&mut establish_connection()); 
        results
    }

    /// Get all the tickets assigned to the passed account.
    pub fn get_all_for(user: &Account) -> Result<Vec<i32>, Error> {
                
        let results = assignment::dsl::assignment
            .inner_join(
                ticket::dsl::ticket.on(
                    assignment::dsl::ticket.eq(ticket::dsl::id).and(assignment::dsl::assigned_to.eq(user.id))
                )
            )
            .select(ticket::dsl::id)
            .load(&mut establish_connection());
        results
    }
    
    pub fn get_all_owned(user: &Account) -> Result<Vec<Self>, Error> {
       use crate::schema::ticket::dsl::*; 

        let results: Result<Vec<Self>, Error> = ticket
            .filter(owner.eq(user.id))
            .load(&mut establish_connection());
        
        results
    }

    pub fn get_all_unassigned() {
        
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

//=======================================
//              Assignment
//=======================================
#[derive(Queryable)]
pub struct Assignment {
    pub id: i32,
    assigned_by: i32,
    assigned_to: i32,
    ticket: i32,
}

#[derive(Insertable)]
#[diesel(table_name = schema::assignment)]
pub struct NewAssignment {
    assigned_by: i32,
    assigned_to: i32,
    ticket: i32,
}

#[derive(Deserialize)]
pub struct BodyAssignment {
    /// Vec of emails of accounts that this is assigned to
    pub assigned_to: Vec<GenericBodyAccount>,
    pub ticket: i32
}

impl Assignment {
    pub fn new(by: &Account, to: &Account, ticket: &Ticket) -> NewAssignment {
        NewAssignment {
            assigned_by: by.id,
            assigned_to: to.id,
            ticket: ticket.id
        } 
    }
}



impl NewAssignment {
    pub fn load(&self) -> Result<Assignment, diesel::result::Error>{
        let mut conn = establish_connection(); 

        let result = diesel::insert_into(assignment::table)
            .values(self)
            .get_result(&mut conn);
        result
    } 
}