extern crate rand;
extern crate nalgebra as na;

use self::rand::thread_rng;
use self::rand::Rng;
use self::rand::distributions::{Normal, IndependentSample};
use self::na::DMat;

pub struct Network {
    num_layers: u32,
    sizes: Vec<u32>,
    biases: Vec<DMat<f64>>,
    weights: Vec<DMat<f64>>,
}

impl Network {
    
    pub fn new(num_layers: u32, sizes: Vec<u32>) -> Network {

        let biases = sizes.iter()
            .skip(1)
            .map(|&size| Network::random_uniform_sample_matrix(size, 1))
            .collect::<Vec<DMat<f64>>>(); 

        let weights = sizes.iter()
            .take(2)
            .zip(sizes.iter().skip(1))
            .map(|(&x, &y)| Network::random_uniform_sample_matrix(y, x))
            .collect::<Vec<DMat<f64>>>();
        Network { num_layers: num_layers, sizes: sizes, biases: biases, weights: weights }
    }

    fn random_uniform_sample_matrix(rows: u32, cols: u32) -> DMat<f64> {
        let n = Normal::new(0.0, 1.0);
        DMat::from_fn(rows as usize, cols as usize, |x: usize, y: usize | {
            n.ind_sample(&mut rand::thread_rng())
        })
    }
    
    fn sigmoid(z: f64) -> f64 {
        1.0/(1.0+(-z).exp())
    }

    fn feedforward(&self, a: &mut DMat<f64>) {
       let mut result = a.clone();
       let w_ite = self.weights.iter();
       let b_ite = self.biases.iter();
       let r = b_ite.zip(w_ite)
           .fold(result, |acc, (b, w) | {
               b * acc + w 
           });
    }

    pub fn sgd(&mut self, mut training_data: Vec<(f64, f64)>, epochs: usize, mini_batch_size: usize, eta: f32) {
        let n = training_data.len();
        for j in 0..n {
            let mut slice = &mut training_data[..];
            thread_rng().shuffle(slice);
            for mini_batch in slice.chunks(mini_batch_size){
                println!("Working on mini batch: {:?}", mini_batch);
                self.update_mini_batch(mini_batch, eta);
            }
            println!("Epoch {} complete", j);
        }
    }

    fn update_mini_batch(&mut self, batch: &[(f64, f64)], eta: f32) {
        let nabla_b = self.biases
            .iter()
            .map(|ref biais| DMat::new_zeros(biais.nrows(), biais.ncols()))
            .collect::<Vec<DMat<f64>>>();   
        let nabla_w = self.weights
            .iter()
            .map(|ref weight| DMat::new_zeros(weight.nrows(), weight.ncols()))
            .collect::<Vec<DMat<f64>>>();
        for &(a, b) in batch {
            println!("{} {}", a, b);
        }
    }

    fn backprop(&mut self, x: f64, y: f64) {
        let nabla_b = self.biases
            .iter()
            .map(|ref biais| DMat::new_zeros(biais.nrows(), biais.ncols()))
            .collect::<Vec<DMat<f64>>>();   
        let nabla_w = self.weights
            .iter()
            .map(|ref weight| DMat::new_zeros(weight.nrows(), weight.ncols()))
            .collect::<Vec<DMat<f64>>>();

        let mut activation = x;
        let mut activations = vec![x];
        let mut zs : Vec<DMat<f64>> = vec![];
        
        let w_ite = self.weights.iter();
        let b_ite = self.biases.iter();
        let r = b_ite.zip(w_ite);

        for (w,b) in r {
//            let z = w * DMat::from_row_vec(1, activations.len(), &activations[..]) + b;
//            zs.push(z);
//            activation = z.map(|x| self.sigmoid(x));
//            activations.push(activation);
        }

    }
}

