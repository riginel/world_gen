use std::cmp::Ordering;
use std::ops::{Bound, Mul};
use noise::{Abs, Add, Checkerboard, Clamp, Cylinders, Multiply, NoiseFn, Perlin, PerlinSurflet, Simplex, SuperSimplex};
use rand::{Rng, thread_rng};

trait Bounded<T,const DIM:usize> :NoiseFn<T,DIM> {
    fn min(&self)->T;
    fn max(&self)->T;

}

impl Bounded<f64,2> for Perlin{
    fn min(&self) -> f64 {
        -1.
    }

    fn max(&self) -> f64 {
        1.
    }
}
impl Bounded<f64,2> for PerlinSurflet{
    fn min(&self) -> f64 {
        -1.
    }

    fn max(&self) -> f64 {
        1.
    }
}
impl  Bounded<f64,2> for Simplex{
    fn min(&self) -> f64 {
        -1.
    }

    fn max(&self) -> f64 {
        1.
    }
}
impl Bounded<f64,2> for SuperSimplex{
    fn min(&self)->f64{
        -1.
    }
    fn max(&self)->f64{
        1.
    }
}
impl Bounded<f64,2> for Cylinders{
    fn min(&self) -> f64 {
        -1.
    }
    fn max(&self)->f64{
        1.
    }
}

impl Bounded<f64,2> for Checkerboard{
    fn min(&self) -> f64 {
        -1.
    }

    fn max(&self) -> f64 {
        1.
    }
}
impl<F> Bounded<f64,2> for Abs<f64, F, 2> where F: Bounded<f64,2>{
    fn min(&self) -> f64 {


        //! First scenario: both min and max are negative
        //! Second scenario: min is positive, that implies that max is positive
        //! Therefore the minima will be the minimum between the two absolute values
        if (self.source.min() <0. && self.source.max() <0.) || (self.source.min() >0.){
         return self.source.min().abs().min(self.source.max().abs())
        }
        else {
            return 0.
        }
    }

    fn max(&self) -> f64 {
        self.source.min().abs().max(self.source.max().abs())
    }
}

impl<F,F1> Bounded<f64,2> for Add<f64, F, F1, 2> where F: Bounded<f64,2>,F1: Bounded<f64,2>{
    fn min(&self) -> f64 {
        self.source1.min()+self.source2.min()
    }

    fn max(&self) -> f64 {
        self.source1.max()+self.source2.max()
    }
}
impl <F,F1> Bounded<f64,2> for Multiply<f64, F, F1, 2> where F: Bounded<f64,2>,F1: Bounded<f64,2>{

    fn min(&self) -> f64 {
        let v:[f64;4] = [self.source1.min()*self.source2.min(),self.source1.min()*self.source2.max(), self.source1.max()*self.source2.min(),self.source1.max()*self.source2.max()];
        v.into_iter().min_by(|a,b | {if a < b {return Ordering::Less;} else {return Ordering::Greater}}).unwrap()

    }

    fn max(&self) -> f64 {
        let v:[f64;4] = [self.source1.min()*self.source2.min(),self.source1.min()*self.source2.max(), self.source1.max()*self.source2.min(),self.source1.max()*self.source2.max()];
        v.into_iter().max_by(|a,b | {if a < b {return Ordering::Less;} else {return Ordering::Greater}}).unwrap()
    }
}

impl <F> Bounded<f64,2> for Clamp<f64,F,2> where F: Bounded<f64,2>{
    fn min(&self) -> f64 {
        self.bounds.0.max(self.source.min())
    }

    fn max(&self) -> f64 {
        self.bounds.1.min(self.source.min())
    }
}




fn bound_accuracy_print<F>(function: F, test_cases: usize) where F: Bounded<f64,2>{
    let (mut min, mut max) = (0.,0.);
    let mut  rng = thread_rng();
    for i in 0..test_cases{
        let val =function.get([rng.gen(),rng.gen()]);
        min = val.min(min);
        max = val.max(max);
    }
    println!("min found by randomness: {}, max found by randomness: {}, trait min: {}, trait max: {}",min,max,function.min(),function.max());
}

#[test]
fn bound_accuracy_test() {
    let mut rng = thread_rng();
    //first let's try with a sum of perlin
    let f = Simplex::new(rng.gen());
    let f1 = Perlin::new(rng.gen());
    //let sum = Add::new(f,f1);
    bound_accuracy_print(f,1000000);
    //bound_accuracy_print(f1,1000000);

}
