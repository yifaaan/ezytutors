use sqlx::postgres::PgPool;
use std::sync::Mutex;

// 多个服务器线程共享AppState数据
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub db: PgPool,
}
