// Quadratische Gleichung mit Enum und Result 
// include!("args.rs");
mod aup;
use aup::argr_f64;
use std::io::Result;

enum Solution {
    EmptySet,
    SetOfOne(f64),
    SetOfTwo(f64,f64)
}

impl Solution {
    fn printx(self) -> Self {
        if let Self::SetOfTwo(s1,s2) = self {
            println!("L={{{}, {}}}", s1, s2);
        } else if let Self::SetOfOne(s1) = self {
            println!("L={{{}}}",s1);
        } else {
            println!("L=∅");
        }
        self
    }
}

fn main() -> Result<()> {
  let (ar1,ar2) = (argr_f64(1),argr_f64(2));
  if let Err(e1) = ar1 {
    Err(e1)
  } else {
    if let Err(e2) = ar2 {
      Err(e2)
    } else {
      Solution::printx(solveqeq(ar1.unwrap(),ar2.unwrap()));
      Ok(())
    }
  }
}

fn solveqeq(p: f64, q: f64) -> Solution{
  let ep = -p /2.0;
  let dis = (ep * ep - q).sqrt();
  if dis.is_nan() {
    Solution::EmptySet
  } else if dis > 0.0 {
    Solution::SetOfTwo(ep + dis, ep - dis)
  } else {
    Solution::SetOfOne(ep)
  }
}
