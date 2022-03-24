mod matrix;
use matrix::func::Func;

fn main(){
  let m = matrix::Matrix::new(vec![vec![0, 1], vec![2, 3]]);
  println!("{:?}", m.trans());

  let sin = Func::new(|x| f64::sin(x));
  println!("{}", sin.0(2.0));
}