use crypto_bigint::U256;
use prog_bitcoin::primitives::point_fe_num::PointFeNum;
use prog_bitcoin::fen;
use prog_bitcoin::s256::s256_field::S256Field;
use prog_bitcoin::s256::s256_point::S256Point;
use prog_bitcoin::s256::signature::Signature;

#[test]
fn ex01_01() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);
    let fe105 = fen!(105, 223);
    let fe192 = fen!(192, 223);
    let _ = PointFeNum::new_concrete(fe192, fe105, fe0, fe7);
}

#[test]
fn ex01_02() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);
    let fe17 = fen!(17, 223);
    let fe56 = fen!(56, 223);
    let _ = PointFeNum::new_concrete(fe17, fe56, fe0, fe7);
}

#[test]
#[should_panic(expected = "is not on the curve")]
fn ex01_03() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);
    let fe200 = fen!(200, 223);
    let fe119 = fen!(119, 223);
    let _ = PointFeNum::new_concrete(fe200, fe119, fe0, fe7);
}

#[test]
fn ex01_04() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);
    let fe1 = fen!(1, 223);
    let fe193 = fen!(193, 223);
    let _ = PointFeNum::new_concrete(fe1, fe193, fe0, fe7);
}

#[test]
#[should_panic(expected = "is not on the curve")]
fn ex01_05() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);
    let fe42 = fen!(42, 223);
    let fe99 = fen!(99, 223);
    let _ = PointFeNum::new_concrete(fe42, fe99, fe0, fe7);
}

#[test]
fn ex02_01() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);

    let fe170 = fen!(170, 223);
    let fe142 = fen!(142, 223);
    let p1 = PointFeNum::new_concrete(fe170, fe142, fe0, fe7);

    let fe60 = fen!(60, 223);
    let fe139 = fen!(139, 223);
    let p2 = PointFeNum::new_concrete(fe60, fe139, fe0, fe7);

    let sum = p1 + p2;

    let fe220 = fen!(220, 223);
    let fe181 = fen!(181, 223);
    assert_eq!(sum.x, Some(fe220));
    assert_eq!(sum.y, Some(fe181));
}

#[test]
fn ex02_02() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);

    let fe47 = fen!(47, 223);
    let fe71 = fen!(71, 223);
    let p1 = PointFeNum::new_concrete(fe47, fe71, fe0, fe7);

    let fe17 = fen!(17, 223);
    let fe56 = fen!(56, 223);
    let p2 = PointFeNum::new_concrete(fe17, fe56, fe0, fe7);

    let sum = p1 + p2;

    let fe215 = fen!(215, 223);
    let fe68 = fen!(68, 223);
    assert_eq!(sum.x, Some(fe215));
    assert_eq!(sum.y, Some(fe68));
}

#[test]
fn ex02_03() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);

    let fe143 = fen!(143, 223);
    let fe98 = fen!(98, 223);
    let p1 = PointFeNum::new_concrete(fe143, fe98, fe0, fe7);

    let fe76 = fen!(76, 223);
    let fe66 = fen!(66, 223);
    let p2 = PointFeNum::new_concrete(fe76, fe66, fe0, fe7);

    let sum = p1 + p2;

    let fe47 = fen!(47, 223);
    let fe71 = fen!(71, 223);
    assert_eq!(sum.x, Some(fe47));
    assert_eq!(sum.y, Some(fe71));
}

#[test]
fn ex04_01() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);

    let fe192 = fen!(192, 223);
    let fe105 = fen!(105, 223);
    let p1 = PointFeNum::new_concrete(fe192, fe105, fe0, fe7);

    let res = p1.smul(2);

    let fe49 = fen!(49, 223);
    let fe71 = fen!(71, 223);
    assert_eq!(res.x, Some(fe49));
    assert_eq!(res.y, Some(fe71));
}

#[test]
fn ex04_02() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);

    let fe143 = fen!(143, 223);
    let fe98 = fen!(98, 223);
    let p1 = PointFeNum::new_concrete(fe143, fe98, fe0, fe7);

    let res = p1.smul(2);

    let fe64 = fen!(64, 223);
    let fe168 = fen!(168, 223);
    assert_eq!(res.x, Some(fe64));
    assert_eq!(res.y, Some(fe168));
}

#[test]
fn ex04_03() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);

    let fe47 = fen!(47, 223);
    let fe71 = fen!(71, 223);
    let p1 = PointFeNum::new_concrete(fe47, fe71, fe0, fe7);

    let res = p1.smul(2);

    let fe36 = fen!(36, 223);
    let fe111 = fen!(111, 223);
    assert_eq!(res.x, Some(fe36));
    assert_eq!(res.y, Some(fe111));
}

#[test]
fn ex04_04() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);

    let fe47 = fen!(47, 223);
    let fe71 = fen!(71, 223);
    let p1 = PointFeNum::new_concrete(fe47, fe71, fe0, fe7);

    let res = p1.smul(4);

    let fe194 = fen!(194, 223);
    let fe51 = fen!(51, 223);
    assert_eq!(res.x, Some(fe194));
    assert_eq!(res.y, Some(fe51));
}

#[test]
fn ex04_05() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);

    let fe47 = fen!(47, 223);
    let fe71 = fen!(71, 223);
    let p1 = PointFeNum::new_concrete(fe47, fe71, fe0, fe7);

    let res = p1.smul(8);

    let fe116 = fen!(116, 223);
    let fe55 = fen!(55, 223);
    assert_eq!(res.x, Some(fe116));
    assert_eq!(res.y, Some(fe55));
}

#[test]
fn ex04_06() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);

    let fe47 = fen!(47, 223);
    let fe71 = fen!(71, 223);
    let p1 = PointFeNum::new_concrete(fe47, fe71, fe0, fe7);

    let res = p1.smul(21);

    assert_eq!(res.x, None);
    assert_eq!(res.y, None);
}

#[test]
fn ex05() {
    let fe0 = fen!(0, 223);
    let fe7 = fen!(7, 223);

    let fe15 = fen!(15, 223);
    let fe86 = fen!(86, 223);
    let p1 = PointFeNum::new_concrete(fe15, fe86, fe0, fe7);
    let mut pt = p1;

    let mut order = 1;

    while pt.x != None {
        pt = pt + p1;
        order += 1;
    }
    assert_eq!(order, 7);
}

#[test]
fn ex06_01() {
    let p = S256Point::new_concrete(
        S256Field::new(U256::from_be_hex("887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c")),
        S256Field::new(U256::from_be_hex("61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34")),
    );

    let z = S256Field::new(U256::from_be_hex("ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60"));
    
    let sig: Signature = Signature {
        r:S256Field::new(U256::from_be_hex("ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395")),
        s:S256Field::new(U256::from_be_hex("068342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4")),
    };

    let res = p.verify(z, sig);

    assert!(res);
}

#[test]
fn ex06_02() {
    let p = S256Point::new_concrete(
        S256Field::new(U256::from_be_hex("887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c")),
        S256Field::new(U256::from_be_hex("61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34")),
    );

    let z = S256Field::new(U256::from_be_hex("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d"));
    
    let sig: Signature = Signature {
        r:S256Field::new(U256::from_be_hex("00eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c")),
        s:S256Field::new(U256::from_be_hex("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6")),
    };

    let res = p.verify(z, sig);

    assert!(res);
}