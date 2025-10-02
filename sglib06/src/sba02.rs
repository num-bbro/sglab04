use crate::p01::Pan;
use askama::Template;
use askama_web::WebTemplate;
use serde::Deserialize;
use sglib05::c04::PeaAssVar;

#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub sbid: Option<String>,
}

use sglib05::c04::VarType;
use sglib05::c04::DNM;
use sglib05::p08::ld_sub_info;
use sglib05::p08::SubInfo;
use std::collections::HashMap;

const FLD_LIST: [(VarType, &str); 17] = [
    (VarType::SmallSellTrVt02, ""),
    (VarType::HmChgEvTrVc01, "/tr01"),
    (VarType::CntLvPowSatTrVc03, ""),
    (VarType::ChgStnCapVc04, ""),
    //(VarType::ChgStnSellVc05, ""),
    (VarType::MvPowSatTrVc06, ""),
    (VarType::PowSolarVc07, ""),
    (VarType::ZoneTrVt06, "/tr02"),
    (VarType::PopTrVt07, "/tr02"),
    (VarType::MvVsppVc08, ""),
    (VarType::HvSppVc09, ""),
    //(VarType::UnbalPowVc12, ""),
    (VarType::CntUnbalPowVc13, ""),
    (VarType::Uc1ValVc14, ""),
    (VarType::Uc2ValVc15, ""),
    (VarType::Uc3ValVc16, ""),
    (VarType::Uc1RankVc17, ""),
    (VarType::Uc2RankVc18, ""),
    (VarType::Uc3RankVc19, ""),
];

#[derive(Template, WebTemplate, Debug, Default)]
#[template(path = "sba02.html")]
pub struct WebTemp {
    name: String,
    assv: Vec<PeaAssVar>,
    sbif: HashMap<String, SubInfo>,
    flds: Vec<(VarType, &'static str)>,
    aojsbh: HashMap<String, Vec<String>>,
}
impl WebTemp {
    fn aoj5(&self, sa: &PeaAssVar) -> bool {
        let (mut c1, mut c2) = (0, 0);
        for s in &sa.aojv {
            c1 += 1;
            if let Some(vv) = self.aojsbh.get(&s.code) {
                c2 += vv.len();
            }
        }
        c1 <= 3 && c2 <= 6
    }
    fn aoj4(&self, sa: &PeaAssVar) -> String {
        let (mut c1, mut c2) = (0, 0);
        let mut ss = String::new();
        use std::fmt::Write;
        for s in &sa.aojv {
            if !ss.is_empty() {
                write!(ss, ",").unwrap();
            }
            write!(ss, "{}", s.name).unwrap();
            c1 += 1;
            if let Some(vv) = self.aojsbh.get(&s.code) {
                write!(ss, "[{}]", vv.len()).unwrap();
                c2 += vv.len();
            }
        }
        format!("{c1}:{c2}:{ss}")
        //format!("{}-{}", sa.aojv.len(), self.aojsbv.len())
    }
}

//use axum::extract::Query;
//pub async fn sb01(para: Query<Param>) -> WebTemp {
pub async fn sba02() -> WebTemp {
    // ============================
    // ==== read rw3 data
    let Ok(buf) = std::fs::read(format!("{DNM}/000-sbno.bin")) else {
        println!("NO rw3.bin file:");
        return WebTemp::default();
    };
    println!("read normalized data");
    // ==== read rw3 data
    let Ok((mut assv, _)): Result<(Vec<sglib05::c04::PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode rw3:");
        return WebTemp::default();
    };
    // ===== AOJ_sbv
    let Ok(buf) = std::fs::read(format!("{DNM}/aoj_sbv.bin")) else {
        println!("NO aoj_sbv.bin");
        return WebTemp::default();
    };
    let Ok((aojsbh, _)): Result<(HashMap<String, Vec<String>>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode aojsbv:");
        return WebTemp::default();
    };

    assv.sort_by(|a, b| {
        let ar = a.v[VarType::Uc1RankVc17.tousz()].v
            + a.v[VarType::Uc2RankVc18.tousz()].v
            + a.v[VarType::Uc3RankVc19.tousz()].v;
        let br = b.v[VarType::Uc1RankVc17.tousz()].v
            + b.v[VarType::Uc2RankVc18.tousz()].v
            + b.v[VarType::Uc3RankVc19.tousz()].v;
        ar.partial_cmp(&br).unwrap()
    });
    //let sbif = sub_inf(); //HashMap<String, SubstInfo>
    let sbif = ld_sub_info();
    WebTemp {
        name: "Substation - sba01 (sort by sub)".to_string(),
        assv,
        sbif: sbif.clone(),
        flds: FLD_LIST.to_vec(),
        aojsbh,
    }
}
