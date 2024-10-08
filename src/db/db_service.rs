use std::env;
use sqlx::{postgres::PgPoolOptions,Pool,Postgres};

use crate::models::student::Student;
use crate::models::student::AttendenceStatus;
use crate::models::student::AttendenceLog;

pub struct DbService {
    pub connection:Pool<Postgres>
}

impl DbService {
    pub async fn init() -> Result<Self,sqlx::Error> {
        let db_uri=env::var("DB_URI").expect("missing DB_URI env variable");

        let connection=PgPoolOptions::new()
                                                .max_connections(5)
                                                .connect(&db_uri)
                                                .await?;
        println!("connected to database");

        Ok(Self {
            connection:connection
        })
    }


    pub async fn get_student_id(self:&Self,unit_id:String,student_unit_id:String)-> Result<String,sqlx::Error> {


        let db_query=format!("SELECT student_id FROM {} WHERE (student_unit_id=$1)",unit_id);

        let db_result=sqlx::query_as::<_,Student>(&db_query)
                                                        .bind(student_unit_id)
                                                        .fetch_one(&self.connection)
                                                        .await?;
        
        Ok(db_result.student_id)
        
    }

    pub async fn check_login_or_logout(self:&Self,student_id:String,date:String) -> Result<AttendenceStatus,sqlx::Error> {


        let student_id=student_id.to_lowercase();

        let db_query=format!("SELECT 1 FROM attendance WHERE date=$1 AND student_id=$2 AND logout=$3");
        
        let db_result=sqlx::query(&db_query)
                                                            .bind(date)
                                                            .bind(student_id)
                                                            .bind(String::from("pending"))
                                                            .execute(&self.connection)
                                                            .await.unwrap();

        if db_result.rows_affected() == 0 {
            return Ok(AttendenceStatus::Login);
        }else{
            return Ok(AttendenceStatus::Logout);
        }  
    }

    pub async fn insert_attendence_log(self:&Self,student_id:String,attendence_log:AttendenceLog)->Result<bool,sqlx::Error> {
        let db_query=format!("INSERT INTO attendance (student_id,student_unit_id,unit_id,date,login,logout) VALUES ($1,$2,$3,$4,$5,$6)");


        let result=sqlx::query(&db_query)
                                                        .bind(student_id)
                                                        .bind(attendence_log.student_unit_id)
                                                        .bind(attendence_log.unit_id.to_uppercase())
                                                        .bind(attendence_log.date)
                                                        .bind(attendence_log.login)
                                                        .bind(attendence_log.logout)
                                                        .execute(&self.connection)
                                                        .await?;

        if result.rows_affected() == 0 {
            return Ok(false);
        }else{
            return Ok(true);
        }

    }

    pub async fn update_attendence_log(self:&Self,student_id:String,logout_time:String) -> Result<bool,sqlx::Error> {
        let db_query=format!("UPDATE attendance SET logout=$1 WHERE student_id=$2 AND logout=$3");

        let student_id=student_id.to_lowercase();

        let result=sqlx::query(&db_query)
                                                        .bind(logout_time)
                                                        .bind(student_id)
                                                        .bind(String::from("pending"))
                                                        .execute(&self.connection)
                                                        .await?;
        if result.rows_affected() == 0 {
            return Ok(false);
        }else{
            return Ok(true);
        }
    }





}