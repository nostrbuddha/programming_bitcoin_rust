use prog_bitcoin::primitives::field_element_num::FieldElementNum;

#[test]
fn ex02_01() {
    let fe_57_44 = FieldElementNum::new(44, 57);
    let fe_57_33 = FieldElementNum::new(33, 57);

    let sum = fe_57_44 + fe_57_33;

    assert_eq!(sum.num, 20);
    assert_eq!(sum.prime, 57);
}

#[test]
fn ex02_02() {
    let fe_57_9 = FieldElementNum::new(9, 57);
    let fe_57_29 = FieldElementNum::new(29, 57);

    let diff = fe_57_9 - fe_57_29;

    assert_eq!(diff.num, 37);
    assert_eq!(diff.prime, 57);
}

#[test]
fn ex02_03() {
    let fe_57_17 = FieldElementNum::new(17, 57);
    let fe_57_42 = FieldElementNum::new(42, 57);
    let fe_57_49 = FieldElementNum::new(49, 57);

    let sum = fe_57_17 + fe_57_42 + fe_57_49;

    assert_eq!(sum.num, 51);
    assert_eq!(sum.prime, 57);
}

#[test]
fn ex02_04() {
    let fe_57_52 = FieldElementNum::new(52, 57);
    let fe_57_30 = FieldElementNum::new(30, 57);
    let fe_57_38 = FieldElementNum::new(38, 57);

    let diff = fe_57_52 - fe_57_30 - fe_57_38;

    assert_eq!(diff.num, 41);
    assert_eq!(diff.prime, 57);
}

#[test]
fn ex04_01() {
    let fe_97_95 = FieldElementNum::new(95, 97);
    let fe_97_45 = FieldElementNum::new(45, 97);
    let fe_97_31 = FieldElementNum::new(31, 97);

    let mul = fe_97_95 * fe_97_45 * fe_97_31;

    assert_eq!(mul.num, 23);
    assert_eq!(mul.prime, 97);
}

#[test]
fn ex04_02() {
    let fe_97_17 = FieldElementNum::new(17, 97);
    let fe_97_13 = FieldElementNum::new(13, 97);
    let fe_97_19 = FieldElementNum::new(19, 97);
    let fe_97_44 = FieldElementNum::new(44, 97);

    let mul = fe_97_17 * fe_97_13 * fe_97_19 * fe_97_44;

    assert_eq!(mul.num, 68);
    assert_eq!(mul.prime, 97);
}

#[test]
fn ex04_03() {
    let fe_97_12 = FieldElementNum::new(12, 97);
    let fe_97_77 = FieldElementNum::new(77, 97);

    let mul = fe_97_12.pow(7) * fe_97_77.pow(49);

    assert_eq!(mul.num, 63);
    assert_eq!(mul.prime, 97);
}

#[test]
fn ex08_01() {
    let fe_31_3 = FieldElementNum::new(3, 31);
    let fe_31_24 = FieldElementNum::new(24, 31);

    let res = fe_31_3 / fe_31_24;

    assert_eq!(res.num, 4);
    assert_eq!(res.prime, 31);
}

#[test]
fn ex08_02() {
    let fe_31_17 = FieldElementNum::new(17, 31);

    let res = fe_31_17.pow(-3);

    assert_eq!(res.num, 29);
    assert_eq!(res.prime, 31);
}

#[test]
fn ex08_03() {
    let fe_31_4 = FieldElementNum::new(4, 31);
    let fe_31_11 = FieldElementNum::new(11, 31);

    let res = fe_31_4.pow(-4) * fe_31_11;

    assert_eq!(res.num, 13);
    assert_eq!(res.prime, 31);
}

