use sglib03::p_31::ld_sb_eb_proj;
use sglib03::p_31::ld_sb_et_proj;
use sglib03::p_31::ld_sb_ev_proj;
use sglib04::aoj::ld_sb_inf;
use sglib04::calc::EnergyProfile;
use sglib04::prc41::ld_sb_tr0;
use std::error::Error;

use crate::p01::get_sbrw00;
use sglib05::c04::VarType;

pub fn calc00() -> Result<(), Box<dyn Error>> {
    let sbtr = ld_sb_tr0();
    let sb_inf = ld_sb_inf()?;
    let sbrw = get_sbrw00();
    let ev = ld_sb_ev_proj().unwrap();
    let eb = ld_sb_eb_proj().unwrap();
    let et = ld_sb_et_proj().unwrap();
    let enpf = EnergyProfile { ev, et, eb };
    let mut ii = 0;
    let mut ss = String::new();
    for (_i, sr) in sbrw.iter().enumerate() {
        let note = format!("{}", sr.v[VarType::TakeNote as usize].v);
        if note != "1" {
            continue;
        }
        if sr.sbid == "SQB" {
            continue;
        }
        ii += 1;
        use std::fmt::Write;
        write!(ss, "{}", sr.sbid)?;
        println!("{ii}.{}", sr.sbid);
        let sb = sr.sbid.to_string();
        if let (Some(sbtr), Some(gs)) = (sbtr.get(&sb), sb_inf.get(&sb)) {
            calc1_a(sbtr, gs, &enpf, &mut ss)?;
        }
        writeln!(ss)?;
    }
    std::fs::write("ben.txt", ss)?;
    Ok(())
}
use num_traits::Pow;
use sglib03::prc4::ld_ben_bess1;
use sglib03::prc4::SubBenInfo;
use sglib04::aoj::GridSubst;
use sglib04::calc::sub_proj_rate;
use sglib04::calc::SubstReport;
use sglib04::prc41::SubCalc;
use sglib04::prc43::BENET;
use sglib04::web1::ben_amt_proj;
use sglib04::web1::ben_bill_accu;
use sglib04::web1::ben_boxline_save;
use sglib04::web1::ben_cash_flow;
use sglib04::web1::ben_dr_save;
use sglib04::web1::ben_emeter;
use sglib04::web1::ben_mt_disconn;
use sglib04::web1::ben_mt_read;
use sglib04::web1::ben_non_tech;
use sglib04::web1::ben_outage_labor;
use sglib04::web1::ben_reduce_complain;
use sglib04::web1::ben_sell_meter;
use sglib04::web1::ben_tou_read;
use sglib04::web1::ben_tou_sell;
use sglib04::web1::ben_tou_update;
use sglib04::web1::ben_trx;
use sglib04::web1::ben_unbalan;
use sglib04::web1::ben_work_save;
use sglib04::web1::tr_val;
use sglib04::web1::AmtProj;
use sglib04::web1::BenProj;
//use sglib04::web1::CALL_CENTER_COST_UP;
use sglib04::web1::EB_UNIT_PRICE;
use sglib04::web1::ET_UNIT_PRICE;
use sglib04::web1::EV_UNIT_PRICE;
use sglib04::web1::M1P_COST;
use sglib04::web1::M3P_COST;
use sglib04::web1::OP_YEAR_END;
use sglib04::web1::OP_YEAR_START;
use sglib04::web1::TRX_COST;
use std::collections::HashMap;
use std::rc::Rc;

pub const CALL_CENTER_COST_UP: f32 = 0.04f32;
pub const ASSET_WORTH_RATIO: f32 = 0.1f32;
pub const MODEL_ENTRY_RATIO: f32 = 0.05f32;
pub const MODEL_ENTRY_COST: f32 = 1000f32;

pub fn calc1_a(
    sbtr: &SubCalc,
    gs: &GridSubst,
    enpf: &EnergyProfile,
    ss: &mut String,
) -> Result<(), Box<dyn Error>> {
    let sb = sbtr.sb.to_string();
    let va_p = tr_val(&sbtr.p_tx_cn_m);
    let _va_c = tr_val(&sbtr.c_tx_cn_m);
    let ben = ld_ben_bess1(&sb);
    let mut emp = Vec::<(u32, f32)>::new();
    for y in OP_YEAR_START..=OP_YEAR_END {
        emp.push((y, 0f32));
    }

    let ben1 = Rc::new(sub_proj_rate(&sb, &enpf.ev, EV_UNIT_PRICE));
    let ben2 = Rc::new(sub_proj_rate(&sb, &enpf.eb, EB_UNIT_PRICE));
    let ben3 = Rc::new(sub_proj_rate(&sb, &enpf.et, ET_UNIT_PRICE));
    let ben4 = Rc::new(BenProj { proj: emp.clone() });
    let ben5 = Rc::new(ben_trx(&va_p));
    let ben6 = Rc::new(ben_unbalan(sbtr));
    let ben7 = Rc::new(ben_non_tech(sbtr, &ben));
    let ben8 = Rc::new(ben_bill_accu(sbtr, &ben));
    let ben9 = Rc::new(ben_cash_flow(sbtr, &ben));
    let ben10 = Rc::new(ben_dr_save(sbtr, &ben));
    let mut ben11 = Rc::new(BenProj { proj: emp.clone() });
    let mut ben12 = Rc::new(BenProj { proj: emp.clone() });
    let mut ben13 = Rc::new(BenProj { proj: emp.clone() });
    let mut ben14 = Rc::new(BenProj { proj: emp.clone() });
    if ben.mx_pw > 0f32
        && ben.grw < 7f32
        && ben.be_start <= 3
        && ben.trlm > 40f32
        && (gs.conf == "AIS" || gs.conf == "GIS")
    {
        let (be_sub_save, be_re_diff, be_svg_save, be_en_added) = ben_amt_proj(&ben);
        ben11 = Rc::new(be_sub_save);
        ben12 = Rc::new(be_svg_save);
        ben13 = Rc::new(be_en_added);
        ben14 = Rc::new(be_re_diff);
    }
    let ben15 = Rc::new(ben_boxline_save(sbtr, &ben));
    let ben16 = Rc::new(ben_work_save(sbtr, &ben));
    let ben17 = Rc::new(ben_sell_meter(sbtr, &ben));
    let ben18 = Rc::new(ben_emeter(sbtr, &ben));
    let ben19 = Rc::new(ben_mt_read(sbtr, &ben));
    let ben20 = Rc::new(ben_mt_disconn(sbtr, &ben));
    let ben21 = Rc::new(ben_tou_sell(sbtr, &ben));
    let ben22 = Rc::new(ben_tou_read(sbtr, &ben));
    let ben23 = Rc::new(ben_tou_update(sbtr, &ben));
    let ben24 = Rc::new(ben_outage_labor(sbtr, &ben));
    let ben25 = Rc::new(ben_reduce_complain(sbtr, &ben));
    let ben26 = Rc::new(ben_asset_value(sbtr, &ben));
    let ben27 = Rc::new(ben_model_entry(sbtr, &ben));

    let sbrep = SubstReport {
        sbtr,
        gs,
        enpf,
        ben,
        ben1,
        ben2,
        ben3,
        ben4,
        ben5,
        ben6,
        ben7,
        ben8,
        ben9,
        ben10,
        ben11,
        ben12,
        ben13,
        ben14,
        ben15,
        ben16,
        ben17,
        ben18,
        ben19,
        ben20,
        ben21,
        ben22,
        ben23,
        ben24,
        ben25,
        ben26,
        ben27,
    };
    let mut benr = HashMap::<u32, Rc<dyn AmtProj>>::new();
    benr.insert(1, sbrep.ben1.clone());
    benr.insert(2, sbrep.ben2.clone());
    benr.insert(3, sbrep.ben3.clone());
    benr.insert(4, sbrep.ben4.clone());
    benr.insert(5, sbrep.ben5.clone());
    benr.insert(6, sbrep.ben6.clone());
    benr.insert(7, sbrep.ben7.clone());
    benr.insert(8, sbrep.ben8.clone());
    benr.insert(9, sbrep.ben9.clone());
    benr.insert(10, sbrep.ben10.clone());
    benr.insert(11, sbrep.ben11.clone());
    benr.insert(12, sbrep.ben12.clone());
    benr.insert(13, sbrep.ben13.clone());
    benr.insert(14, sbrep.ben14.clone());
    benr.insert(15, sbrep.ben15.clone());
    benr.insert(16, sbrep.ben16.clone());
    benr.insert(17, sbrep.ben17.clone());
    benr.insert(18, sbrep.ben18.clone());
    benr.insert(19, sbrep.ben19.clone());
    benr.insert(20, sbrep.ben20.clone());
    benr.insert(21, sbrep.ben21.clone());
    benr.insert(22, sbrep.ben22.clone());
    benr.insert(23, sbrep.ben23.clone());
    benr.insert(24, sbrep.ben24.clone());
    benr.insert(25, sbrep.ben25.clone());
    benr.insert(26, sbrep.ben26.clone());
    benr.insert(27, sbrep.ben27.clone());
    let mut ii = 0u32;
    for (h, _c) in &BENET {
        ii += 1;
        let mut va = 0f32;
        if let Some(ampj) = benr.get(&ii) {
            for y in OP_YEAR_START..=OP_YEAR_END {
                //let yt = y + 543;
                let v = ampj.get_amt(y).unwrap();
                va += v;
                //let v = v as u32;
                //println!("  {y} - {v}");
            }
        }
        let h = h.trim();
        println!("{h}. {va}");
        use std::fmt::Write;
        write!(ss, "\t{va}")?;
    }

    Ok(())
}

pub fn ben_asset_value(sbtr: &SubCalc, ben: &SubBenInfo) -> BenProj {
    //print!("====  ASSET");
    let m1i = sbtr.mt_1_ph as f64 * M1P_COST as f64;
    let m3i = sbtr.mt_3_ph as f64 * M3P_COST as f64;
    let txp = sbtr.p_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let txc = sbtr.c_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let txi = (txp + txc) as f64 * TRX_COST as f64;
    let mut esi = 0f64;
    if ben.mx_pw > 0f32 && ben.grw < 7f32 && ben.be_start <= 3 && ben.trlm > 40f32 {
        esi = ben.bat_cost as f64 * 1_000_000_f64;
    }
    let ass = (m1i + m3i + txi + esi) * ASSET_WORTH_RATIO as f64;
    //print!("  m1:{m1i} m3:{m3i} t:{txi} b:{esi} = as:{ass}\n");
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..11 {
        proj.push((y + 2028, 0f32));
    }
    proj.push((11 + 2028, ass as f32));
    //println!();
    BenProj { proj }
}

pub fn ben_model_entry(sbtr: &SubCalc, ben: &SubBenInfo) -> BenProj {
    //print!("====  MODEL ENTRY");
    let txp = sbtr.p_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let txc = sbtr.c_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let mut cnt = (txp + txc + sbtr.mt_1_ph as u32 + sbtr.mt_3_ph as u32) as f64;
    if ben.mx_pw > 0f32 && ben.grw < 7f32 && ben.be_start <= 3 && ben.trlm > 40f32 {
        cnt += 1.0;
    }
    let ent_cn = cnt * MODEL_ENTRY_RATIO as f64;
    let ent_ex = ent_cn * MODEL_ENTRY_COST as f64;

    //print!("  cn:{ent_cn} ex:{ent_ex} \n");
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = ent_ex;
        let be = be * Pow::pow(1f64 + CALL_CENTER_COST_UP as f64, y as f64);
        //print!(" {} - {be}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

use strum::IntoEnumIterator;
//use strum_macros::EnumIter;

pub fn var_type() -> Result<(), Box<dyn Error>> {
    for v in VarType::iter() {
        println!("{v:?}");
    }
    println!("var type");
    Ok(())
}
