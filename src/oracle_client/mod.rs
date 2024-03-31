pub struct OraclePool(oracle::pool::Pool);

impl OraclePool {
    pub fn new() -> Self {
        let username = std::env::var("ORACLE_USERNAME").unwrap();
        let password = std::env::var("ORACLE_PASSWORD").unwrap();
        let host = std::env::var("ORACLE_HOST").unwrap();

        let pool = oracle::pool::PoolBuilder::new(username, password, host)
            .min_connections(8) // Min == Max always
            .max_connections(8)
            .build();
        OraclePool(pool.unwrap())
    }
    pub fn get_conn(&self) -> oracle::Connection {
        self.0.get().unwrap()
    }
}
