
use chrono::{NaiveDateTime,ParseError};
use serde::Deserialize;

#[derive(Debug,sqlx::FromRow)]
pub struct Student {
    pub student_id:String
}

#[derive(Debug)]
pub enum AttendenceStatus {
    Login,
    Logout
}

#[derive(Debug,Deserialize)]
pub struct AttendenceLogRequest {
    pub uid:String,
    pub suid:String,
    pub tm:String
}

#[derive(Debug)]
pub struct AttendenceLog {
    pub student_unit_id:String,
    pub unit_id:String,
    pub date:String,
    pub login:String,
    pub logout:String
}

#[derive(Debug)]
pub struct AttendenceUpdate {
    pub time:String
}

impl TryFrom<AttendenceLogRequest> for AttendenceLog {

    type Error = ParseError;

    fn try_from(value: AttendenceLogRequest) -> Result<Self, Self::Error> {

        let datetime=NaiveDateTime::parse_from_str(&value.tm, "%Y-%m-%dT%H:%M:%S")?;

        let date=datetime.date().to_string();

        let time=datetime.time().to_string();

        Ok(Self{
            student_unit_id:value.suid,
            unit_id:value.uid,
            date,
            login:time,
            logout:String::from("pending")
        })
    }
}


impl TryFrom<AttendenceLogRequest> for AttendenceUpdate {
    type Error = ParseError;
    fn try_from(value: AttendenceLogRequest) -> Result<Self, Self::Error> {

        let datetime=NaiveDateTime::parse_from_str(&value.tm, "%Y-%m-%dT%H:%M:%S")?;

        let time=datetime.time().to_string();

        Ok(Self { time })

    }
}
