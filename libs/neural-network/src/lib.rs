use rand::Rng;

pub struct Network {
    layers: Vec<Layer>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Network {
    pub fn random(layers: &[LayerTopology]) -> Self {
        //Ensure that the layer is more than one layer
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
            .collect();
        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

impl Layer {
    pub fn random(input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(input_size))
            .collect();

        Self { neurons }
    }
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

impl Neuron {
    pub fn random(input_size: usize) -> Self {
        let mut rng = rand::thread_rng();

        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();

        Self { bias, weights }
    }

    #[warn(dead_code)]
    fn random_test(rng: &mut dyn rand::RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();

        Self { bias, weights }
    }
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (output + self.bias).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use super::*;

        #[test]
        fn test_random() {
            use approx::assert_relative_eq;
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;
            // Because we always use the same seed, our `rng` in here will
            // always return the same set of values
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random_test(&mut rng, 4);

            assert_relative_eq!(neuron.bias, -0.6255188);
            assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
            );
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn test_propagate_neuron() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            //Ensure that our .max() works as intended:
            approx::assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0,);

            // 0.5 and 1.0 test it
            approx::assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
            );
        }

        #[test]
        fn test_propagate_Layer() {
            let layer = Layer {
                neurons: vec![
                    Neuron {
                        bias: 0.5,
                        weights: vec![-0.3, 1.0],
                    },
                    Neuron {
                        bias: 0.2,
                        weights: vec![-0.3, 0.8],
                    },
                    Neuron {
                        bias: 0.1,
                        weights: vec![0.3, 0.2],
                    },
                ],
            };

            let inputs = vec![-0.3, 0.5];

            //Ensure that the .max() works (ReLu)
            assert_eq!(&layer.propagate(inputs), &vec![1.09, 0.69, 0.11]);
        }
        #[test]
        fn test_propagate_layer() {
            let layer = Layer {
                neurons: vec![
                    Neuron {
                        bias: 0.5,
                        weights: vec![-0.3, 1.0],
                    },
                    Neuron {
                        bias: 0.2,
                        weights: vec![-0.3, 0.8],
                    },
                    Neuron {
                        bias: 0.1,
                        weights: vec![0.3, 0.2],
                    },
                ],
            };

            let inputs = vec![-0.3, 0.5];

            //Ensure that the .max() works (ReLu)
            assert_eq!(&layer.propagate(inputs), &vec![1.09, 0.69, 0.11]);
        }
        #[test]
        fn test_propagate_network() {}
    }
}
