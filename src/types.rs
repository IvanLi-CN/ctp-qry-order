use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct SessionInfo {
    pub front_id: i32,
    pub session_id: i32,
    ///  (YYYYMMDD)
    pub trading_day: String,
    ///  (HHMMSS)
    pub login_time: String,
    pub broker_id: String,
    pub user_id: String,
    pub system_name: String,
    pub max_order_ref: i32,
    pub system_version: String,
}

impl Display for SessionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Front:\t{} \nSession:\t{} \nTradingDay:\t{} \nLoginTime:\t{} \nBroker:\t{} \nUser:\t\t{} \nSystem:\t{} {} \nMaxOrderRef:\t{}",
            self.front_id, self.session_id, self.trading_day, self.login_time, self.broker_id, self.user_id, self.system_name, self.system_version, self.max_order_ref
        )
    }
}
