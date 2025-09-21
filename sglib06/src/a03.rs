use crate::p01::Pan;
use askama::Template;
use askama_web::WebTemplate;
use sglib05::c04::PeaAssVar;

#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub sbid: Option<String>,
}

use sglab02_lib::sg::prc1::SubstInfo;
use sglab02_lib::sg::prc5::sub_inf;
//use sglib05::c04::VarType;
use sglib05::c04::DNM;
//use sglib05::c04::WE_EV;
use sglib05::c04::WE_UC1;
use std::collections::HashMap;

#[derive(Template, WebTemplate, Debug, Default)]
#[template(path = "a03.html")]
pub struct WebTemp {
    name: String,
    assv: Vec<PeaAssVar>,
    sbif: HashMap<String, SubstInfo>,
}

use serde::Deserialize;
//use axum::extract::Query;
//pub async fn a03(para: Query<Param>) -> WebTemp {
pub async fn a03() -> WebTemp {
    /*
    let Some(ref sbid) = para.sbid else {
        println!("NO SBID");
        return WebTemp::default();
    };
    println!("para:{sbid:?}");
    */
    // ============================
    // ==== read rw3 data
    let Ok(buf) = std::fs::read(format!("{DNM}/000-sbrw.bin")) else {
        println!("NO rw3.bin file:");
        return WebTemp::default();
    };
    // ==== read rw3 data
    let Ok((assv, _)): Result<(Vec<sglib05::c04::PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode rw3:");
        return WebTemp::default();
    };
    let sbif = sub_inf(); //HashMap<String, SubstInfo>
    WebTemp {
        name: "EV calculation % - a01".to_string(),
        assv,
        sbif: sbif.clone(),
    }
}
