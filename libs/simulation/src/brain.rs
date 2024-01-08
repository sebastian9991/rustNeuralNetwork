use crate::eye::Eye;
use crate::*;

#[derive(Debug)]
pub struct Brain {
    pub(crate) nn: nn::Network,
}

impl Brain {
    pub fn random(eye: &Eye) -> Self {
        Self {
            nn: nn::Network::random(&Self::topology(eye)),
        }
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        ga::Chromosome::new(self.nn.weights())
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(eye), chromosome),
        }
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },
            nn::LayerTopology { neurons: 2 },
        ]
    }
}
