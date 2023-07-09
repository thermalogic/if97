
#![allow(warnings)]

use assert_approx_eq::assert_approx_eq;

use if97::pT_reg5;
use if97::ph_reg5;
use if97::ps_reg5;
use if97::hs_reg5;
use if97::hs2p_reg5;
use if97::ph2T_reg5;
use if97::ps2T_reg5;
use if97::common::*;


// Table 42, page 40 T,p  v,h,u,s,cp,w
const data:[[f64;8];3] = [
          [1500.0, 0.5, 1.38455090, 0.521976855e+4, 4527.49310, 9.65408875, 2.61609445, 917.068690],
          [1500.0, 30.0, 0.0230761299, 5167.23514, 4474.95124, 7.72970133, 2.72724317, 928.548002],
          [2000.0, 30.0, 0.0311385219, 6571.22604, 5637.07038, 8.53640523, 2.88569882, 1067.36948]];


#[test]
fn test_pt()
{

  for i in 0..2
  {
    let T:f64 = data[i][0];
    let p:f64 = data[i][1];
    assert_approx_eq!(data[i][2], pT_reg5(p, T, OV), 1.0e-6f64); 
    assert_approx_eq!(data[i][3], pT_reg5(p, T, OH), 1.0e-4f64); 
    assert_approx_eq!(data[i][4], pT_reg5(p, T, OU), 1.0e-4f64);
    assert_approx_eq!(data[i][5], pT_reg5(p, T, OS), 1.0e-5f64);
    assert_approx_eq!(data[i][6], pT_reg5(p,T, OCP), 1.0e-6f64);
    assert_approx_eq!(data[i][7], pT_reg5(p,T, OW), 1.0e-6f64);
   
  }
}

#[test] 
fn test_ph()
{
  // Table 42, page 40 T,p  v,h,u,s,cp,w
  for  i in 0..2
  {
    let p:f64 = data[i][1];
    let h:f64 = data[i][3];
    assert_approx_eq!(data[i][0], ph2T_reg5(p, h));
    assert_approx_eq!(data[i][5], ph_reg5(p, h, OS));
  }
}

#[test] 
fn test_ps()
{
  // Table 42, page 40 T,p  v,h,u,s,cp,w
  for  i in 0..2
  {
    let p:f64 = data[i][1];
    let s:f64 = data[i][5];
    assert_approx_eq!(data[i][0], ps2T_reg5(p, s), 1.0e-4f64);
    assert_approx_eq!(data[i][3], ps_reg5(p, s, OH), 1.0e-4f64);
  }
}

#[test] 
fn test_hs()
{
  // Table 42, page 40 T,p  v,h,u,s,cp,w
  for  i in 0..2
  {
    let h:f64 = data[i][3];
    let s:f64 = data[i][5];
    assert_approx_eq!(data[i][1], hs2p_reg5(h, s), 1.0e-4f64);
    assert_approx_eq!(data[i][0]-273.15, hs_reg5(h, s, OT), 1.0e-4f64);
    assert_approx_eq!(data[i][2], hs_reg5(h, s, OV), 1.0e-4f64);
  }
}
