use anyhow::Result;
use firestore_db_and_auth::{documents, Credentials, ServiceSession};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

use crate::git::CommitPoint;

const STORED_LENGTH: usize = 8;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Skill {
    check: bool,
    level: u8,
    name: String,
}

// this layout is inconsistent for some reason, must have this
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum GradYear {
    Str(String),
    Num(u16),
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct RegistrationData {
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
    pub(crate) fn fetch_registration(&self) -> Result<Vec<RegistrationData>> {
        let values: documents::List<RegistrationData, _> =
            documents::list(&self.session, "registration_data");
        Ok(values
            .filter_map(|x| if let Ok((v, _)) = x { Some(v) } else { None })
            .collect())
    }

    // pub(crate) fn fetch_user_registration(&self, email: &str) -> Result<RegistrationData, firestore_db_and_auth::errors::FirebaseError> {
    //     documents::read(&self.session, "registration_data", email)
    // }

    pub(crate) fn fetch_json_storage(&self) -> Result<Vec<RegistrationData>, Box<dyn Error>> {
        Ok(read_dir("./json_data")?
            .filter_map(|x| read_to_string(x.ok()?.path()).ok())
            .filter_map(|x| serde_json::from_str::<Vec<RegistrationData>>(&x).ok())
            .flatten()
            .collect())
    }
}
