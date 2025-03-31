use ndarray::prelude::*;
use ndarray::{Array, ShapeBuilder};

fn main() {
    let arr1 = array![[1.,2.,3.], [4.,5.,6.]];
    println!("{:?}", arr1);

    println!("zero");
    // https://towardsdatascience.com/the-ultimate-ndarray-handbook-mastering-the-art-of-scientific-computing-with-rust-ef5ab767212a
    let arr2 = Array::<f64, _>::zeros((3, 4, 5).f());
    println!("{:?}", arr2);
}
