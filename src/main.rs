mod logger;
mod db;
mod redis;
mod handler;
mod models;

use dotenv::dotenv;

use logger::init_logger;
use crate::db::db_service::DbService;
use crate::redis::redis_service::RedisService;
use crate::handler::handler_service::HandlerService;

#[tokio::main]
async fn main() {

    //initializing the logger
    init_logger().expect("failed to initialize the logger");

    //loading env variables .env file to environment
    dotenv().ok();

    let db_service=DbService::init().await.expect("failed to initialize the db service");
    let redis_service=RedisService::init().await.expect("failed to intialize the redis service");

    let handler_service=HandlerService::init(db_service, redis_service);

    handler_service.run().await;
}
