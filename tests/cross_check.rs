//! Integration tests for cross-checking backend implementations.

#[cfg(all(feature = "backend-dashu", feature = "backend-rug"))]
#[test]
fn test_backends_match_exactly() {
    use ni_number::backend::NiFloat;
    use ni_number::backend::dashu::DashuBackend;
    use ni_number::backend::rug::RugBackend;
    use ni_number::compute;

    // Вычисляем 5000 десятичных знаков (~ 3386 бит)
    let digits = 5000;
    let bits = compute::digits_to_bits(digits);

    let val_dashu = compute::compute::<DashuBackend>(bits);
    let val_rug = compute::compute::<RugBackend>(bits);

    let str_dashu = val_dashu.to_decimal_string(digits);
    let str_rug = val_rug.to_decimal_string(digits);

    assert_eq!(
        str_dashu, str_rug,
        "Dashu and Rug backends produced different results at {} bits!",
        bits
    );
}

#[cfg(feature = "backend-f64")]
#[test]
fn test_f64_backend_prefix() {
    use ni_number::backend::NiFloat;
    use ni_number::backend::f64::F64Backend;
    use ni_number::compute;
    let val = compute::compute::<F64Backend>(64);
    let s = val.to_decimal_string(10);
    assert!(s.starts_with("1.8893766604"), "wrong prefix: {}", s);
}
