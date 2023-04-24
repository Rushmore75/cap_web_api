use std::str::{FromStr, Utf8Error};

use bimap::BiMap;
use crypto::{scrypt::{scrypt, ScryptParams}};
use rocket::{request::{FromRequest, self, Outcome}, Request, tokio::sync::RwLock, http::Status};
use serde::Serialize;

use crate::db::Account;

const LOGIN_COOKIE_ID: &str = "session-id";
const EMAIL_HEADER_ID: &str = "email";
const PASSWORD_HEADER_ID: &str = "password";

/// Not too sure if "keyring" is the correct terminology...
/// This holds all the session ids that are currently active.
pub struct Keyring {
    all: BiMap<String, Uuid>
}

impl Keyring {
    const OUTPUT_SIZE: usize = 24;

    pub fn new() -> Self {
        Self {
            all: BiMap::<String, Uuid>::new()
        }
    }
     
    /// A centralized way to hash strings (but mostly just passwords)
    /// for the web api.
    pub fn hash_string(input: &str) -> [u8; Self::OUTPUT_SIZE] {
        let mut hashed_password = [0u8; Self::OUTPUT_SIZE];
       
        // FIXME learn how to salt properly
        scrypt(
            input.as_bytes(),
            &[1, 2, 4, 5],
            &ScryptParams::new(5, 5, 5),
            &mut hashed_password
        );

        hashed_password
    } 

    /// # Login
    /// Will try to log the user designated by the given email and password.
    /// If this attempt it successful it will return them a new [`Session`].
    fn login(&mut self, email: &str, password: &str) -> Option<Session> {
        // search the db for the account under that email.
        match Account::get_users_hash(email) {
            Some(stored_hash) => {
                // then see if the password hashes match.
                if Self::hash_string(password) == stored_hash[..] {
                    // generate them a user id
                    let user_id = Uuid::wrap(uuid::Uuid::new_v4());
                    self.all.insert(email.to_string(), user_id);
                    return Some(Session(user_id));
                } 
            },  
            None => println!("Please create a user for \"{}\" before trying to log in as them.", email),
        };
        None
    }
    
    pub fn logout(&mut self, session: &Session) {
        self.all.remove_by_right(&session.0);
    }
    
    /// Check if the session id you have is valid.
    #[must_use]
    fn is_valid_session(&self, session_id: &Session) -> bool {
        self.get_email(session_id).is_some()
    }
    
    /// Get the email of a user based on their session id.
    pub fn get_email(&self, session_id: &Session) -> Option<String> {
        self.all.get_by_right(&session_id.0).cloned()
    }
     

}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
/// A wrapper around Uuid so I can impl my own methods.
pub struct Uuid {
    uuid: uuid::Uuid,
}

impl Uuid {
    /// From uuid
    pub fn wrap(uuid: uuid::Uuid) -> Self{
        Self {
            uuid
        }
    }
}

/// Represents a user's session, holding their session id.
/// # As a Request Guard
/// This can be used as a rocket request guard. It will check the user's cookies for
/// a valid session id, if that doesn't exist it will check the headers for a email /
/// password combo, and try to log them in that way. If both of these fail, it will
/// throw an error and the request will not continue.
pub struct Session(Uuid);

impl From<uuid::Uuid> for Session {
    /// Automatically wrap the uuid.
    fn from(value: uuid::Uuid) -> Self {
        Self(Uuid { uuid: value })
    }
}

impl Serialize for Session {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
    {
        serializer.serialize_str(&self.0.uuid.to_string())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = LoginError;

    /// # Authenticate User
    /// This will try to authenticate a user via their session id cookie. If this fails
    /// it will fall back to trying to read the `EMAIL_HEADER_ID` and `PASSWORD_HEADER_ID`
    /// (as each defined as const values) from the user's header, if these exist it will
    /// try to authenticate them that way.
    /// # Return
    /// If the function is successful in authenticating the user it will return their 
    /// session id.
    /// If the function is unsuccessful it will return an error.
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {

        // Make get the keyring from rocket
        if let Some(keyring) = req.rocket().state::<RwLock<Keyring>>() {                    
            
            // Check the user's cookies for a session id 
            if let Some(session_cookie) = req.cookies().get(LOGIN_COOKIE_ID) {
                // Extract the cookie into a uuid
                if let Ok(id) = uuid::Uuid::from_str(session_cookie.value()) {
                    if keyring.read().await.is_valid_session(&Session::from(id)) {
                        println!("Authenticating via cookie");
                        // authenticate user
                        return Outcome::Success( Session(Uuid::wrap(id)) );
                    }
                }    
            };
            // Something above, has at this point, gone wrong.

            // TODO potentially shouldn't log in the user with the authenticate method.
            // But at the same time there isn't really a reason to add complexity.

            // If they have both their email and password in the headers, log them in.
            if let Some(email) = req.headers().get_one(EMAIL_HEADER_ID) {
                if let Some(password) = req.headers().get_one(PASSWORD_HEADER_ID) {
                    if let Some(id) = keyring.write().await.login(email, password) {
                        println!("Authenticating via user/pass combo");
                        // TODO add a way to tell the user to change from email / password method
                        // to the session id method
                        return Outcome::Success( id );
                    }
                }
            }
        };

        // Logging in with a session id and email/password combo have both failed        
        Outcome::Failure((Status::Unauthorized, LoginError::Error))
    }
}

#[derive(Debug)]
pub enum LoginError {
    Error,
}
