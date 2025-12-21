mod interval;
mod profit_calculator;

use interval::Interval;
pub use profit_calculator::IntervalEvaluator;

#[cfg(test)]
mod profit_calculator_test;
