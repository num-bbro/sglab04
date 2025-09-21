use crate::p01::Pan;
use askama::Template;
use askama_web::WebTemplate;
use serde::Deserialize;
use sglib05::c04::PeaAssVar;

#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub sbid: Option<String>,
}

use sglib05::p08::ld_sub_info;
use sglib05::p08::SubInfo;
//use sglab02_lib::sg::prc1::SubstInfo;
//use sglab02_lib::sg::prc5::sub_inf;
use sglib05::c04::VarType;
use sglib05::c04::DNM;
use std::collections::HashMap;

const FLD_LIST: [(VarType, &str); 14] = [
    (VarType::SmallSellTrVt02, ""),
    (VarType::HmChgEvTrVc01, "/tr01"),
    (VarType::CntLvPowSatTrVc03, ""),
    (VarType::ChgStnCapVc04, ""),
    (VarType::ChgStnSellVc05, ""),
    (VarType::MvPowSatTrVc06, ""),
    (VarType::PowSolarVc07, ""),
    (VarType::ZoneTrVt06, ""),
    (VarType::PopTrVt07, ""),
    (VarType::MvVsppVc08, ""),
    (VarType::HvSppVc09, ""),
    (VarType::UnbalPowVc12, ""),
    (VarType::CntUnbalPowVc13, ""),
    (VarType::Uc1ValVc14, ""),
];

#[derive(Template, WebTemplate, Debug, Default)]
#[template(path = "sb03.html")]
pub struct WebTemp {
    name: String,
    assv: Vec<PeaAssVar>,
    sbif: HashMap<String, SubInfo>,
    flds: Vec<(VarType, &'static str)>,
}

//use axum::extract::Query;
//pub async fn sb01(para: Query<Param>) -> WebTemp {
pub async fn sb03() -> WebTemp {
    // ============================
    // ==== read rw3 data
    let Ok(buf) = std::fs::read(format!("{DNM}/000-sbrw.bin")) else {
        println!("NO rw3.bin file:");
        return WebTemp::default();
    };
    // ==== read rw3 data
    let Ok((mut assv, _)): Result<(Vec<sglib05::c04::PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode rw3:");
        return WebTemp::default();
    };
    assv.sort_by(|b, a| {
        a.v[VarType::HmChgEvTrVc01.tousz()]
            .v
            .partial_cmp(&b.v[VarType::HmChgEvTrVc01.tousz()].v)
            .unwrap()
    });

    //let sbif = sub_inf(); //HashMap<String, SubstInfo>
    let sbif = ld_sub_info();
    WebTemp {
        name: "Substation sort by EV".to_string(),
        assv,
        sbif: sbif.clone(),
        flds: FLD_LIST.to_vec(),
    }
}
