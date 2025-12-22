use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

/// Determines what kind of values the generator will produce, with specified maximums
#[derive(Clone, Copy)]
pub enum ValueType {
    Fractional,
    Integer,
}

impl TryFrom<String> for ValueType {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Fractional" => Ok(Self::Fractional),
            "Integer" => Ok(Self::Integer),
            _ => Err(()),
        }
    }
}

pub struct Generator {
    rng: StdRng,
    value_type: ValueType,
    max_value: f64,
}

impl Generator {
    pub fn new(seed: Option<u64>, value_type: ValueType, max_value: f64) -> Self {
        let rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_os_rng(),
        };
        Self {
            rng,
            value_type,
            max_value,
        }
    }

    pub fn random(&mut self) -> f64 {
        match self.value_type {
            ValueType::Fractional => self.rng.random::<f64>() * self.max_value,
            ValueType::Integer => (self.rng.next_u64() % self.max_value as u64) as f64,
        }
    }

    pub fn random_vector(&mut self, size: usize) -> Vec<f64> {
        (0..size).map(|_| self.random()).collect()
    }

    pub fn random_matrix(&mut self, shape: (usize, usize)) -> Vec<Vec<f64>> {
        (0..shape.0).map(|_| self.random_vector(shape.1)).collect()
    }

    pub fn random_symmetric_matrix_diagonal_0(&mut self, shape: (usize, usize)) -> Vec<Vec<f64>> {
        let mut v = vec![vec![0.0; shape.1]; shape.0];

        for i in 0..shape.0 {
            for j in i..shape.1 {
                if i == j {
                    v[i][j] = 0_f64;
                    continue;
                }
                v[i][j] = self.random();
                v[j][i] = self.random();
            }
        }

        v
    }
}
