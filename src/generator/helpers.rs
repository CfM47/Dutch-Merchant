use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueType {
    Integer,
    Float,
}

impl TryFrom<String> for ValueType {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "integer" | "int" => Ok(ValueType::Integer),
            "float" => Ok(ValueType::Float),
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
            Some(seed) => StdRng::seed_from_u64(seed),
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
            ValueType::Integer => self.rng.random_range(0..self.max_value as i64) as f64,
            ValueType::Float => self.rng.random::<f64>() * self.max_value,
        }
    }

    pub fn random_range(&mut self, min: f64, max: f64) -> f64 {
        match self.value_type {
            ValueType::Integer => self.rng.random_range(min as i64..max as i64) as f64,
            ValueType::Float => self.rng.random_range(min..max),
        }
    }

    pub fn random_vector(&mut self, size: usize) -> Vec<f64> {
        (0..size).map(|_| self.random()).collect()
    }

    pub fn random_matrix(&mut self, shape: (usize, usize)) -> Vec<Vec<f64>> {
        (0..shape.0)
            .map(|_| (0..shape.1).map(|_| self.random()).collect())
            .collect()
    }

    pub fn random_symmetric_matrix_diagonal_0(&mut self, shape: (usize, usize)) -> Vec<Vec<f64>> {
        let n = shape.0;
        let mut matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in i + 1..n {
                let val = self.random();
                matrix[i][j] = val;
                matrix[j][i] = val;
            }
        }
        matrix
    }
}
