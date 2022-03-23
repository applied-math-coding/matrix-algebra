mod matrix;

fn main(){
  let m = matrix::Matrix::new(vec![vec![0, 1], vec![2, 3]]);
  println!("{:?}", m.trans());
}