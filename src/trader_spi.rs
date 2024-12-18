use std::{path::Path, sync::Arc, thread, time::Duration};

use ctp2rs::{
    ffi::{gb18030_cstr_i8_to_str, AssignFromString, WrapToString},
    print_rsp_info,
    v1alpha1::*,
};
use log::*;

use crate::config::{CtpConfig, LibsConfig};

pub struct BaseTraderSpi {
    pub tdapi: Arc<TraderApi>,
    pub request_id: i32,

    ctp_config: CtpConfig,

    session_info: Option<CThostFtdcRspUserLoginField>,
}

impl BaseTraderSpi {
    fn query_order(&mut self) {
        let mut req = CThostFtdcQryOrderField::default();
        req.BrokerID
            .assign_from_str(self.ctp_config.broker_id.as_str());
        req.InvestorID
            .assign_from_str(self.ctp_config.investor_id.as_str());

        self.request_id += 1;
        self.tdapi.req_qry_order(&mut req, self.request_id);
    }

    fn query_trade(&mut self) {
        let mut req = CThostFtdcQryTradeField::default();
        req.BrokerID
            .assign_from_str(self.ctp_config.broker_id.as_str());
        req.InvestorID
            .assign_from_str(self.ctp_config.investor_id.as_str());

        self.request_id += 1;
        self.tdapi.req_qry_trade(&mut req, self.request_id);
    }
}

impl TraderSpi for BaseTraderSpi {
    fn on_front_connected(&mut self) {
        println!("tdspi.on_front_connected !!!");
        let mut req = CThostFtdcReqAuthenticateField::default();
        req.BrokerID
            .assign_from_str(self.ctp_config.broker_id.as_str());
        req.UserID
            .assign_from_str(self.ctp_config.investor_id.as_str());
        req.AppID.assign_from_str(self.ctp_config.app_id.as_str());
        req.AuthCode
            .assign_from_str(self.ctp_config.auth_code.as_str());

        self.request_id += 1;
        self.tdapi.req_authenticate(&mut req, self.request_id);
    }

    fn on_front_disconnected(&mut self, n_reason: i32) {
        println!("on_front_disconnected: reason{n_reason}");
    }

    fn on_heart_beat_warning(&mut self, n_time_lapse: i32) {
        println!("on_heart_beat_warning: {n_time_lapse}");
    }

    fn on_rsp_error(&mut self, p_rsp_info: Option<&CThostFtdcRspInfoField>, _: i32, _: bool) {
        debug!("rsp_error");
        print_rsp_info!(p_rsp_info);
    }

    fn on_rsp_authenticate(
        &mut self,
        _: Option<&CThostFtdcRspAuthenticateField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _: i32,
        b_is_last: bool,
    ) {
        println!("on_rsp_authenticate");
        print_rsp_info!(p_rsp_info);
        if let Some(p_rsp_info) = p_rsp_info {
            if p_rsp_info.ErrorID != 0 {
                return;
            }
        }

        if b_is_last {
            let mut req = CThostFtdcReqUserLoginField::default();
            let user_id = self.ctp_config.investor_id.as_str();
            let password = self.ctp_config.password.as_str();

            req.BrokerID
                .assign_from_str(self.ctp_config.broker_id.as_str());
            req.UserID.assign_from_str(&user_id);
            req.Password.assign_from_str(&password);

            self.request_id += 1;
            let ret = self.tdapi.req_user_login(&mut req, self.request_id);
            println!("req_user_login result: {ret}");
        }
    }

    fn on_rsp_user_login(
        &mut self,
        user_login_field: Option<&CThostFtdcRspUserLoginField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _: i32,
        b_is_last: bool,
    ) {
        print_rsp_info!(p_rsp_info);
        if b_is_last {
            let mut req = CThostFtdcSettlementInfoConfirmField::default();
            req.BrokerID
                .assign_from_str(self.ctp_config.broker_id.as_str());
            req.InvestorID
                .assign_from_str(self.ctp_config.investor_id.as_str());

            self.request_id += 1;
            let ret = self
                .tdapi
                .req_settlement_info_confirm(&mut req, self.request_id);
            debug!("req_user_login result: {ret}");

            if let Some(user_login_field) = user_login_field {
                self.session_info = Some(user_login_field.clone());
            } else {
                error!("user_login_field is None");
            }
        }
    }

    fn on_rsp_settlement_info_confirm(
        &mut self,
        _: Option<&CThostFtdcSettlementInfoConfirmField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _: i32,
        b_is_last: bool,
    ) {
        print_rsp_info!(p_rsp_info);
        if b_is_last {
            std::thread::sleep(std::time::Duration::from_secs(1));
            self.request_id += 1;
            let mut req = CThostFtdcQryInvestorPositionField::default();
            let ret = self
                .tdapi
                .req_qry_investor_position(&mut req, self.request_id);
            println!("req_qry_investor_position result: {ret}");
        }
    }

    fn on_rsp_qry_investor_position(
        &mut self,
        p_investor_position: Option<&CThostFtdcInvestorPositionField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _: i32,
        b_is_last: bool,
    ) {
        print_rsp_info!(p_rsp_info);
        if let Some(p) = p_investor_position {
            let instrument_id = p.InstrumentID.to_string();
            let user_id = p.InvestorID.to_string();
            println!("{user_id} holds {instrument_id}");
        } else {
            println!("position hold None");
        }
        if b_is_last {
            println!("on_rsp_qry_investor_position finish!");

            // self.query_order();
            
            self.query_trade();
        }
    }
    
    fn on_rsp_qry_order(
        &mut self,
        p_order: Option<&CThostFtdcOrderField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _: i32,
        b_is_last: bool,
    ) {
        print_rsp_info!(p_rsp_info);

        if p_order.is_none() || p_rsp_info.is_none() {
            warn!("p_order or p_rsp_info is None");

            return;
        }

        debug!("on_rsp_qry_order: {:?} {b_is_last}", p_order);
    }

    fn on_rsp_qry_trade(
        &mut self,
        p_trade: Option<&CThostFtdcTradeField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _: i32,
        b_is_last: bool,
    ) {
        print_rsp_info!(p_rsp_info);

        if p_trade.is_none() || p_rsp_info.is_none() {
            warn!("p_trade or p_rsp_info is None");
            return;
        }

        info!("trade: {:?}. b_is_last: {}", p_trade.unwrap(), b_is_last);
    }

}

pub fn run_td(ctp_config: CtpConfig, libs_config: LibsConfig) {
    println!("tdapi starting");

    let dynlib_path = libs_config.ctp_thosttraderapi_path.clone();

    let dynlib_path = Path::new(&dynlib_path);

    let tdapi = TraderApi::create_api(dynlib_path, "./td_");
    let tdapi = Arc::new(tdapi);

    tdapi.register_front(ctp_config.trader_api_endpoint.as_str()); // tts 7x24 td

    let base_tdspi = BaseTraderSpi {
        tdapi: Arc::clone(&tdapi),
        request_id: 0,

        ctp_config,

        session_info: None,
    };
    let tdspi_box = Box::new(base_tdspi);
    let tdspi_ptr = Box::into_raw(tdspi_box);

    println!("get_td_api_version: {}", tdapi.get_api_version());

    tdapi.register_spi(tdspi_ptr);
    tdapi.subscribe_private_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK);
    tdapi.subscribe_public_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK);

    tdapi.init();

    println!("tdapi init");

    loop {
        println!("td loop");
        thread::sleep(Duration::from_secs(10));
    }
}
