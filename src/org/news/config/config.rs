use redis::Connection;
use self::redis::Client;

extern crate redis;
pub struct AppState{
    pub redis: Connection
}

struct RedisActor {
    conn: MultiplexedConnection,
}


impl RedisActor {
    pub async fn new(redis_url: &'static str) -> Self {
        let client = Client::open(redis_url).unwrap();// not recommended
        let (conn, call) = client.get_multiplexed_async_connection().await.unwrap();
        actix_rt::spawn(call);
        RedisActor { conn }
    }
}
