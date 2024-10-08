use std::time::Duration;
use chrono::NaiveDateTime;
use log::error;

use crate::models::student::AttendenceLogRequest;
use crate::models::student::AttendenceLog;
use crate::models::student::AttendenceUpdate;
use crate::models::student::AttendenceStatus;
use crate::db::db_service::DbService;
use crate::redis::redis_service::RedisService;



pub struct HandlerService {
    db_service:DbService,
    redis_service:RedisService
}


impl HandlerService {
    pub fn init(db_service:DbService,redis_service:RedisService) ->Self {
        Self{
            db_service,
            redis_service
        }
    }

    pub async fn run(self:&Self) {
            loop {
                let redis_result=self.redis_service.find_list_length().await;

                if let Ok(list_len)=redis_result {

                    if list_len > 0 {

                        if let Ok(Some(result))=self.redis_service.get_list_log().await {

                                let attendence_log_request_json=&result[0];

                                if let Ok(attendence_log_request)=serde_json::from_str::<AttendenceLogRequest>(&attendence_log_request_json) {
                                
                                    if let Ok(student_id)=self.db_service.get_student_id(attendence_log_request.uid.clone(), attendence_log_request.suid.clone()).await {
                            

                                        let unit_id=attendence_log_request.uid.clone();
                                        let student_unit_id=attendence_log_request.suid.clone();
                                        if let Ok(datetime)=NaiveDateTime::parse_from_str(&attendence_log_request.tm, "%Y-%m-%dT%H:%M:%S"){

                                        let date=datetime.date().to_string();

                                        if let Ok(login_or_logout)=self.db_service.check_login_or_logout(student_id.clone(),date).await {

                                            match login_or_logout {

                                                AttendenceStatus::Login => {

                                                    if let Ok(attendence_log)=AttendenceLog::try_from(attendence_log_request){

                                                        if let Ok(_)=self.db_service.insert_attendence_log(student_id,attendence_log).await {

                                                            if let Err(_)=self.redis_service.remove_list_log().await {
    
                                                                error!("error occured while removing the attendence log from the redis unit_id -> {} student_unit_id -> {}",unit_id,student_unit_id);
                                                            
                                                            }
    
                                                            println!("login successfull");
    
                                                        }else{
    
                                                            error!("error occurred while inserting the attendence log of unit id -> {} and student_unit_id -> {} to the postgres",unit_id,student_unit_id);    
                                                        
                                                        }   
                                                    }else{
                                                        error!("error occured while parsing the timestamp from  attendence log unit_id -> {} student_unit_id -> {}",unit_id,student_id);
                                                    }         
                                                },
                                                AttendenceStatus::Logout => {

                                                        if let Ok(attendence_update) = AttendenceUpdate::try_from(attendence_log_request) {
                                                            if let Ok(_) = self.db_service.update_attendence_log(student_id, attendence_update.time).await {

                                                                if let Err(_)=self.redis_service.remove_list_log().await {
                                                            
                                                                    error!("error occured while removing the attendence log from the redis unit_id -> {} student_unit_id -> {}",unit_id,student_unit_id);
                                                            
                                                                }
    
                                                                println!("logout successfull");
    
                                                            }else{
    
                                                                error!("error occured while updating the attendence log of unit_id -> {} student_unit_id -> {}",unit_id,student_unit_id);
                                                        
                                                            }
                                                        }else{
                                                            error!("error occured while parsing the timestamp from  attendence log unit_id -> {} student_unit_id -> {}",unit_id,student_id);
                                                        }
                                                    }
                                                }
                                            }else {
                                                error!("error occurred while checking login or logout in database unit_id -> {} student_id -> {}",unit_id,student_unit_id);
                                            }
                                        }else{
                                            error!("error occurred while parsing the time stamp from the attendence log request unit_id -> {} student_id -> {}",unit_id,student_unit_id);
                                        }

                                    }else{
                                        error!("error occured while getting the student id by using unit_id -> {} and student_id -> {} from postgres",attendence_log_request.uid,attendence_log_request.suid);
                                    }
                                }else{
                                    error!("error occured while parsing the attendence log request from [{}]",attendence_log_request_json);
                                }
                            }else{
                                error!("error occured while fetching the attendence log from redis");
                            }
                        }
                }else{
                    error!("error occurred while checking the attendence log list length from redis");
                }
                
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
    }
}