use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use redis::aio::MultiplexedConnection;

pub struct RedisService {
    pub list_name:String,
    pub connection:Arc<Mutex<MultiplexedConnection>>
}   

impl RedisService {
    pub async  fn init()->Result<Self,redis::RedisError> {
        let redis_uri=env::var("REDIS_URI").expect("misssing env variable REDIS_URI");
        let redis_list_name=env::var("REDIS_LIST_NAME").expect("missing env variable REDIS_LIST_NAME");

        let client=redis::Client::open(redis_uri)?;

        let mut connection=client.get_multiplexed_async_connection().await?;

        let redis_result:Result<String,redis::RedisError>=redis::cmd("PING").query_async(&mut connection).await;
        

        match redis_result {
            Ok(_) =>  {

                println!("connected to redis");

                return Ok(Self {
                    list_name:redis_list_name,
                    connection:Arc::new(Mutex::new(connection))
                })
            },
            Err(err) => return  Err(err)
        }
    }

    pub async fn find_list_length(self:&Self) ->Result<u32,redis::RedisError> {

        let list_len_check_command="LLEN";
        let list_name=&self.list_name;

        let arguements=[list_name];
        let mut redis_connection=self.connection.lock().await;

        let list_count:u32=redis::cmd(list_len_check_command).arg(&arguements).query_async(&mut *redis_connection).await?;

        Ok(list_count)
    }

    pub async fn get_list_log(self:&Self) -> Result<Option<Vec<String>>,redis::RedisError> {
        
        let list_log_get_command="LRANGE";
        let list_name=&self.list_name;
        let list_start_index="0";
        let list_end_index="-1";

        let arguements=[list_name,list_start_index,list_end_index];

        let mut redis_connection=self.connection.lock().await;

        let attendence_log:Option<Vec<String>>=redis::cmd(list_log_get_command).arg(&arguements).query_async(&mut *redis_connection).await?;

        Ok(attendence_log)
    }

    pub async fn remove_list_log(self:&Self) -> Result<(),redis::RedisError> {

        let list_log_remove_command="RPOP";
        let list_name=&self.list_name;

        let arguements=[list_name];

        let mut redis_connection=self.connection.lock().await;

        redis::cmd(list_log_remove_command).arg(&arguements).exec_async(&mut *redis_connection).await?;

        Ok(())
    }

}