pub mod web {
    use crate::constants::web::ACTIX_WEB_PARALLELISM_KEY;

    pub fn parallelism() -> usize {
        std::env::var(ACTIX_WEB_PARALLELISM_KEY)
            .unwrap_or(String::from("4")).parse::<usize>().unwrap()
    }
}

pub mod database {
    use crate::constants::database::{DATABASE_CONNECTION_POOL_SIZE_KEY, DATABASE_URL_KEY};

    pub fn url() -> String {
        std::env::var(DATABASE_URL_KEY)
            .expect("DATABASE_URL not set")
    }

    pub fn connection_pool_size() -> u32 {
        std::env::var(DATABASE_CONNECTION_POOL_SIZE_KEY)
            .unwrap_or(String::from("10")).parse::<u32>().unwrap()
    }
}




