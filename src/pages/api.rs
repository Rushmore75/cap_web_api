use rocket::{get, serde::json::Json, post, response::{status, Redirect}, http::{Status, CookieJar, Cookie}, tokio::sync::RwLock, State, form::{Form, Strict}, uri};

use crate::{db::{Account, BodyAccount, Dept, Ticket, BodyTicket, Message, Assignment, BodyAssignment}, authentication::{Session, Keyring}};

#[post("/api/login", data="<credentials>")]
pub async fn login(credentials: Form<Strict<BodyAccount<'_>>>, keyring: &State<RwLock<Keyring>>, jar: &CookieJar<'_>) -> Result<Redirect, status::BadRequest<&'static str>> {
    // Json::from(auth)
    match keyring.write().await.login(credentials.email, credentials.password, jar) {
        Some(_) => Ok(Redirect::to("/dashboard")),
        None => Err(status::BadRequest(Some("Failed to login."))) 
    }
}

#[get("/api/logout")]
pub async fn logout(auth: Session, keyring: &State<RwLock<Keyring>>, jar: &CookieJar<'_>) -> status::Accepted<&'static str> {
    keyring.write().await.logout(&auth, jar);
    status::Accepted(Some("logged out"))
}

#[post("/api/submit_ticket", data="<body>")]
pub fn submit_ticket(auth: Session, body: Form<Strict<BodyTicket<'_>>>, jar: &CookieJar<'_>) -> Result<Redirect, status::Custom<&'static str>> {

    if let Ok(account) = Account::get(&auth.email) {
        if let Ok(title) = Message::new(&account, body.title).load() {
            if let Ok(content) = Message::new(&account, body.body).load() {
                if let Ok(ticket) = Ticket::new(&account, title, content).load() {
                    jar.add(Cookie::new("submit_success", ticket.id.to_string()));
                    return Ok(Redirect::to(uri!("/dashboard")))
                }
            }
        }
    }
    Err(status::Custom(Status::InternalServerError, "Could not create the ticket."))                        
}

#[get("/api/unassigned")]
pub fn unassigned_tickets(_auth: Session) -> Json<Vec<Ticket>> {
    // TODO make this a supervisor only path
    if let Ok(all) = Ticket::get_all_unassigned() {
        let x = all
            .iter()
            .map(|f| Ticket::get(*f))
            .filter_map(|f| f.ok())
            .collect::<Vec<Ticket>>();
        return Json::from(x);
    }
    Json::from(Vec::new())
}

#[post("/api/assign_ticket", data="<body>")]
pub fn assign_ticket(auth: Session, body: Json<BodyAssignment>) {
    // get the user's email
    if let Ok(from) = Account::get(&auth.email) {
        // make sure the selected ticket is real
        if let Ok(ticket) = Ticket::get(body.ticket) {
            // iterate thru all assignees to make sure they exist
            body
                .assigned_to
                .iter()
                .map(|f| Account::get(f))
                .filter(|f| f.is_ok())
                .map(|f| f.unwrap())
                .fold(Vec::new(), |mut v, f| {
                    // assign the ticket to all of them
                    match Assignment::new(&from, &f, &ticket).load() {
                        Ok(e) => {v.push(e.id)},
                        Err(_) => {
                            // Cancel the operation
                            // TODO undo all tickets assigned thus far
                            // looking for sql transaction iirc
                            todo!()
                        }
                    }
                    v
                });
        }
    }
}

#[get("/api/tickets")]
pub fn my_tickets(auth: Session) -> Json<Vec<(i32, String, String)>> {
    if let Ok(acc) = Account::get(&auth.email) {
        if let Ok(tickets) = Ticket::get_all_for(&acc) {
            let x = tickets
                .iter()
                .filter_map(|f| Ticket::get(*f).ok())
                .map(|f| (f.id, Message::get(f.title), Message::get(f.description))) 
                .fold(Vec::new(), |mut v, f| {
                    if f.1.is_ok() && f.2.is_ok() {
                        v.push((f.0, f.1.unwrap().content, f.2.unwrap().content))    
                    }
                    v
                });
            return Json::from(x);
       }
    }
    Json::from(Vec::new())
}

#[get("/api/owned_tickets")]
pub fn owned_tickets(auth: Session) -> Option<Json<Vec<(String, String)>>> {
    if let Ok(acc) = Account::get(&auth.email) {
        if let Ok(tickets) = Ticket::get_all_owned(&acc) {
            let v = tickets.iter().fold(Vec::new(), |mut v, f| {
                if let Ok(title) = Message::get(f.title) {
                    if let Ok(body) = Message::get(f.description) {
                        v.push((title.content, body.content))
                    }
                }
                v
            });
            return Some(Json::from(v));
        }
    }
    None
}

#[post("/api/create_user", data="<body>")]
pub fn create_user(body: Json<BodyAccount>) -> status::Custom<&'static str> {
    
    match Dept::get_id(&crate::db::Department::Client) {
        Ok(dept) => {
            match Account::new(
                body.email,
                body.password,
                dept
            ).load()
            {
                Ok(_account) => {
                    return status::Custom(Status::Accepted, "Created account.");
                },
                Err(e) => {
                    // I want to specifically handle the already created error.
                    if let diesel::result::Error::DatabaseError(err, _) = e {
                        if let diesel::result::DatabaseErrorKind::UniqueViolation = err {
                                    return status::Custom(Status::BadRequest, "This user is already registered.");           
                        }
                    }
                }
            };
        },
        Err(_) => { /* a database error while trying to get a department. */ }
    };

    status::Custom(Status::InternalServerError, "Unhandled error while creating user.") 
}

#[get("/api/get_employees")]
pub fn get_employees(_auth: Session) -> Json<Vec<String>> {
    // TODO supervisors only path
    if let Ok(accs) = Dept::all_from(crate::db::Department::Flunky) {
        return Json::from(accs);        
    }
    Json::from(Vec::new())
}

#[get("/api/message/<id>")]
pub fn get_msg(id: i64, _auth: Session) -> Json<String> {
    match Message::get(id) {
        Ok(e) => Json::from(e.content),
        Err(_) => Json::from("".to_owned())
    }
}

#[post("/api/complete_ticket/<id>")]
pub fn complete_ticket(_auth: Session, id: i32) -> status::Custom<&'static str> {
    if let Ok(ticket) = Ticket::get(id) {
        if let Ok(_) = ticket.close() {
            return status::Custom(Status::Accepted, "marked as complete");
        }
    }
    status::Custom(Status::BadRequest, "failed to complete ticket")
}
