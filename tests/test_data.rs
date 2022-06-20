use yamini::memory::InnerData;

#[test]
fn test_inner_data_equality() {
    assert_eq!(InnerData::INT(3), InnerData::INT(3));
    assert_eq!(InnerData::INT16(200), InnerData::INT16(200));
    assert_eq!(InnerData::INT32(1000), InnerData::INT32(1000));
    assert_eq!(InnerData::STR("ab".to_string()), InnerData::STR("ab".to_string()));
}

#[test]
fn test_get_u8() {
    assert_eq!(InnerData::INT(3).get_u8(), 3);
    assert_eq!(InnerData::INT(-3).get_u8(), 253);
}

#[test]
fn test_get_i8() {
    assert_eq!(InnerData::INT(3).get_i8(), 3);
    assert_eq!(InnerData::INT(-3).get_i8(), -3);
}

#[test]
fn test_get_u16() {
    assert_eq!(InnerData::INT16(3).get_u16(), 3);
    assert_eq!(InnerData::INT16(-3).get_u16(), 65533);
}

#[test]
fn test_get_i16() {
    assert_eq!(InnerData::INT16(3).get_i16(), 3);
    assert_eq!(InnerData::INT16(-3).get_i16(), -3);
}

#[test]
fn test_clone() {
    assert_eq!(InnerData::INT(3).clone(), InnerData::INT(3));
    assert_eq!(InnerData::INT16(3).clone(), InnerData::INT16(3));
    assert_eq!(InnerData::INT32(3).clone(), InnerData::INT32(3));
    assert_eq!(InnerData::STR("ab".to_string()).clone(), InnerData::STR("ab".to_string()));
}

#[test]
fn test_variant_eq() {
    assert_eq!(InnerData::variant_eq(&InnerData::INT(3), &InnerData::INT(3)), true);
}

#[test]
fn test_add_overflow_i8() {
    let a = InnerData::INT(90);
    let b = InnerData::INT(80);

    let result = a + b;
    assert_eq!(result, InnerData::INT16(170));
    assert_eq!(InnerData::variant_eq(&result, &InnerData::INT16(170)), true);
}

#[test]
fn test_sub_overflow_i8() {
    let a = InnerData::INT(-80);
    let b = InnerData::INT(90);

    let result = a - b;

    assert_eq!(result, InnerData::INT16(-170));
    assert_eq!(InnerData::variant_eq(&result, &InnerData::INT16(-170)), true);
}

#[test]
fn test_mul_overflow_i8() {
    let a = InnerData::INT(10);
    let b = InnerData::INT(13);

    let result = a * b;

    assert_eq!(result, InnerData::INT16(130));
    assert_eq!(InnerData::variant_eq(&result, &InnerData::INT16(130)), true);
}

#[test]
fn test_add_overflow_i16() {
    let a = InnerData::INT16(32767);
    let b = InnerData::INT16(10);

    let result = a + b;

    assert_eq!(result, InnerData::INT32(32777));
    assert_eq!(InnerData::variant_eq(&result, &InnerData::INT32(32777)), true);
}

#[test]
fn test_sub_overflow_i16() {
    let a = InnerData::INT16(-10);
    let b = InnerData::INT16(32767);

    let result = a - b;

    assert_eq!(result, InnerData::INT32(-32777));
    assert_eq!(InnerData::variant_eq(&result, &InnerData::INT32(-32757)), true);
}

#[test]
fn test_mul_overflow_i16() {
    let a = InnerData::INT16(10);
    let b = InnerData::INT16(32767);

    let result = a * b;

    assert_eq!(result, InnerData::INT32(327670));
    assert_eq!(InnerData::variant_eq(&result, &InnerData::INT32(327670)), true);
}