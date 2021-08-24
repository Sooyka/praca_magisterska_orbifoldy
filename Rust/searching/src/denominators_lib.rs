use serde::{Deserialize, Serialize};

use crate::common_lib::*;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DenominatorsMaximalExact {
    Maximal,
    Exact,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DenominatorsOutput {
    pub provided_p_q: bool,
    pub p_q: bool,
    pub p_q_order: bool,
    pub number_of_p_q_occurences: bool,
    pub order_and_occurences: bool,
    pub yes_no_counting: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DenominatorsConfig {
    pub base_manifolds: Vec<TwoDimentionalManifold>,
    pub only_relatively_prime_numerators: bool,
    pub maximal_exact: DenominatorsMaximalExact,
    pub order_and_occurences: i64,
    pub yes_no_counting: bool,
    pub output: DenominatorsOutput,
}
