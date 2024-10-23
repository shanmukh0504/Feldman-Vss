use num_bigint::BigInt;

pub fn bigint_to_string(value: &BigInt) -> String {
    value.to_str_radix(10)
}

pub fn string_to_bigint(value: &str) -> BigInt {
    BigInt::parse_bytes(value.as_bytes(), 10).unwrap()
}