enum DenominatorsExclusiveInclusive {
    Exclusive,
    Inclusive
}

enum DenominatorsMaximalExact {
    Maximal,
    Exact
}

struct DenominatorsOutput {
    p: bool, 
    q: bool,
    p_q: bool,
    p_q_order: bool,
    number_of_p_q_occurences: bool,
    occurences: i64

}

struct DenominatorsConfig {
    exclusive_inclusive: DenominatorsExclusiveInclusive,
    maximal_exact: DenominatorsMaximalExact,
    output: DenominatorsOutput
}