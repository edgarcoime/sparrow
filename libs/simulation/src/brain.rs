use crate::*;

#[derive(Debug)]
pub struct Brain {
    crate speed_accel: f32,
    crate rotation_accel: f32,
    crate nn: nn::Network,
}

impl Brain {
    crate fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        let nn = nn::Network::random(rng, &Self::topology(config));

        Self::new(config, nn)
    }

    crate fn from_chromosome(config: &Config, chromosome: ga::Chromosome) -> Self {
        let nn = nn::Network::from_weights(&Self::topology(config), chromosome);

        Self::new(config, nn)
    }

    crate fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    crate fn propagate(&self, vision: Vec<f32>) -> (f32, f32) {
        let response = self.nn.propagate(vision);

        let r0 = response[0].clamp(0.0, 1.0) - 0.5;
        let r1 = response[1].clamp(0.0, 1.0) - 0.5;
        let speed = (r0 + r1).clamp(-self.speed_accel, self.speed_accel);
        let rotation = (r0 + r1).clamp(-self.rotation_accel, self.rotation_accel);

        (speed, rotation)
    }
}

impl Brain {
    fn new(config: &Config, nn: nn::Network) -> Self {
        Self {
            speed_accel: config.sim_speed_accel,
            rotation_accel: config.sim_rotation_accel,
            nn,
        }
    }

    fn topology(config: &Config) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: config.eye_cells,
            },
            nn::LayerTopology {
                neurons: 2 * config.eye_cells,
            },
            nn::LayerTopology {
                neurons: 2
            },
        ]
    }
}
