/*---------------------------------------------------------------------------
  Region 1 API

 Author: Maohua Cheng
---------------------------------------------------------------------------*/
use crate::common::constant::*;

use crate::r1::region1_T_phps::*;
use crate::r1::region1_pT::*;
use crate::r1::region1_p_hs::*;

pub fn pT_reg1(p: f64, T: f64, o_id: i32) -> f64
// o_id: output propertry
{
    match o_id {
        OT => T,
        OP => p,
        OV => pT2v_reg1(p, T),
        OD => 1.0 / pT2v_reg1(p, T),
        OH => pT2h_reg1(p, T),
        OS => pT2s_reg1(p, T),
        OU => pT2u_reg1(p, T),
        OCV => pT2cv_reg1(p, T),
        OCP => pT2cp_reg1(p, T),
        OW => pT2w_reg1(p, T),
        _ => INVALID_OUTID as f64,
    }
}

pub fn pt_reg1(p:f64,t:f64,o_id:i32) -> f64 {
    let T:f64=t+K;
    pT_reg1(p,T,o_id)
}


pub fn ph_reg1(p: f64, h: f64, o_id: i32) -> f64
// o_id: output propertry
{
    if o_id == OP {
        return p;
    };
    if o_id == OH {
        return h;
    };

    let T: f64 = ph2T_reg1(p, h);
    if o_id == OT {
        return T-273.15;
    };
    return pT_reg1(p, T, o_id);
}

pub fn ps_reg1(p: f64, s: f64, o_id: i32) -> f64
// o_id: output propertry
{
    if o_id == OP {
        return p;
    };
    if o_id == OS {
        return s;
    };

    let T: f64 = ps2T_reg1(p, s);
    if o_id == OT {
        return T-273.15;
    };
    return pT_reg1(p, T, o_id);
}

pub fn hs_reg1(h: f64, s: f64, o_id: i32) -> f64
// o_id: output propertry
{
    if o_id == OH {
        return h;
    };
    if o_id == OS {
        return s;
    };

    let p: f64 = hs2p_reg1(h, s);
    return ph_reg1(p, h, o_id);
}
