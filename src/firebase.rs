use firestore_db_and_auth::{documents, Credentials, ServiceSession};
use serde::Deserialize;
use std::fmt::Display;
use std::path::Path;
use anyhow::Error;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Skill {
    check: bool,
    level: u8,
    name: String,
}

// this layout is inconsistent for some reason, must have this
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum GradYear {
    Str(String),
    Num(u16),
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct RegistrationData {
    admin: bool,
    cad_fill_in: String,
    cad_skills: Vec<Skill>,
    change_reason: String,
    change_teams: bool,
    email: String,
    first_experience: bool,
    first_name: String,
    grad_year: GradYear,
    last_name: String,
    paid: bool,
    parent_coc: bool,
    parent_email: String,
    parent_name: String,
    parent_phone: String,
    permission_form: bool,
    phone: String,
    previous_experience: bool,
    programming_skills: Vec<Skill>,
    registration_status: String,
    student_coc: bool,
    team_preference: Vec<String>,
}

pub(crate) struct Firebase {
    cred: Credentials,
    session: ServiceSession,
}

impl Firebase {
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let cred = Credentials::from_file(&path.as_ref().to_string_lossy())?;
        Ok(Self {
            session: ServiceSession::new(cred.clone())?,
            cred,
        })
    }
    pub(crate) fn refresh_session(&mut self) -> Result<()> {
        self.session = ServiceSession::new(self.cred.clone())?;
        Ok(())
    }
    // todo change to result
    pub(crate) fn fetch_registration(&self) -> Result<()> {
        let values: documents::List<RegistrationData, _> =
            documents::list(&self.session, "registration_data");
        for value in values {
            let (data, _doc) = match value {
                Ok(v) => v,
                Err(_) => return Err(),
            };
            println!("{:?}", data);
        }
        Ok(())
    }
    pub(crate) fn fetch_user_registration(email: &str) -> Self {
        unimplemented!();
    }
}
