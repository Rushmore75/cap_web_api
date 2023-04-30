use crate::WWW;
use rocket::{get, fs::NamedFile};

use crate::{authentication::Session, db::Account};

#[get("/dashboard")]
pub async fn dashboard(auth: Session) -> Result<NamedFile, std::io::Error> {
    if let Ok(acc) = Account::get(&auth.email) {
        if let Ok(dept) = acc.get_dept() {
            match dept.get_department_name() {
                crate::db::Department::Client =>       return NamedFile::open(format!("{WWW}/dashboard/client.html")).await,
                crate::db::Department::Supervisor =>   return NamedFile::open(format!("{WWW}/dashboard/supervisor.html")).await,
                crate::db::Department::Flunky => todo!(),
            }
        }
    }
    todo!();
    // File::open(format!("{WWW}/www/login.html")).await
}

