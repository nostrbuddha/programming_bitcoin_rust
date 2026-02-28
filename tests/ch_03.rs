use crypto_bigint::U256;
use prog_bitcoin::primitives::point_fe_u32::PointFEU32;
use prog_bitcoin::{fe32, p_fe32};
use prog_bitcoin::algorithms::hash256::hash256;
use prog_bitcoin::s256::private_key::PrivateKey;
use prog_bitcoin::s256::s256_field::S256Field;
use prog_bitcoin::s256::s256_point::S256Point;
use prog_bitcoin::s256::signature::Signature;

#[test]
fn ex01_01() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);
    let fe105 = fe32!(105, 223);
    let fe192 = fe32!(192, 223);
    let _ = p_fe32!(fe192, fe105, fe0, fe7);
}

#[test]
fn ex01_02() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);
    let fe17 = fe32!(17, 223);
    let fe56 = fe32!(56, 223);
    let _ = PointFEU32::new_concrete(fe17, fe56, fe0, fe7);
}

#[test]
#[should_panic(expected = "is not on the curve")]
fn ex01_03() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);
    let fe200 = fe32!(200, 223);
    let fe119 = fe32!(119, 223);
    let _ = p_fe32!(fe200, fe119, fe0, fe7);
}

#[test]
fn ex01_04() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);
    let fe1 = fe32!(1, 223);
    let fe193 = fe32!(193, 223);
    let _ = p_fe32!(fe1, fe193, fe0, fe7);
}

#[test]
#[should_panic(expected = "is not on the curve")]
fn ex01_05() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);
    let fe42 = fe32!(42, 223);
    let fe99 = fe32!(99, 223);
    let _ = p_fe32!(fe42, fe99, fe0, fe7);
}

#[test]
fn ex02_01() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);

    let fe170 = fe32!(170, 223);
    let fe142 = fe32!(142, 223);
    let p1 = p_fe32!(fe170, fe142, fe0, fe7);

    let fe60 = fe32!(60, 223);
    let fe139 = fe32!(139, 223);
    let p2 = p_fe32!(fe60, fe139, fe0, fe7);

    let sum = p1 + p2;

    let fe220 = fe32!(220, 223);
    let fe181 = fe32!(181, 223);
    assert_eq!(sum.x, Some(fe220));
    assert_eq!(sum.y, Some(fe181));
}

#[test]
fn ex02_02() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);

    let fe47 = fe32!(47, 223);
    let fe71 = fe32!(71, 223);
    let p1 = p_fe32!(fe47, fe71, fe0, fe7);

    let fe17 = fe32!(17, 223);
    let fe56 = fe32!(56, 223);
    let p2 = p_fe32!(fe17, fe56, fe0, fe7);

    let sum = p1 + p2;

    let fe215 = fe32!(215, 223);
    let fe68 = fe32!(68, 223);
    assert_eq!(sum.x, Some(fe215));
    assert_eq!(sum.y, Some(fe68));
}

#[test]
fn ex02_03() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);

    let fe143 = fe32!(143, 223);
    let fe98 = fe32!(98, 223);
    let p1 = p_fe32!(fe143, fe98, fe0, fe7);

    let fe76 = fe32!(76, 223);
    let fe66 = fe32!(66, 223);
    let p2 = p_fe32!(fe76, fe66, fe0, fe7);

    let sum = p1 + p2;

    let fe47 = fe32!(47, 223);
    let fe71 = fe32!(71, 223);
    assert_eq!(sum.x, Some(fe47));
    assert_eq!(sum.y, Some(fe71));
}

#[test]
fn ex04_01() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);

    let fe192 = fe32!(192, 223);
    let fe105 = fe32!(105, 223);
    let p1 = p_fe32!(fe192, fe105, fe0, fe7);

    let res = p1.smul(2);

    let fe49 = fe32!(49, 223);
    let fe71 = fe32!(71, 223);
    assert_eq!(res.x, Some(fe49));
    assert_eq!(res.y, Some(fe71));
}

#[test]
fn ex04_02() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);

    let fe143 = fe32!(143, 223);
    let fe98 = fe32!(98, 223);
    let p1 = p_fe32!(fe143, fe98, fe0, fe7);

    let res = p1.smul(2);

    let fe64 = fe32!(64, 223);
    let fe168 = fe32!(168, 223);
    assert_eq!(res.x, Some(fe64));
    assert_eq!(res.y, Some(fe168));
}

#[test]
fn ex04_03() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);

    let fe47 = fe32!(47, 223);
    let fe71 = fe32!(71, 223);
    let p1 = p_fe32!(fe47, fe71, fe0, fe7);

    let res = p1.smul(2);

    let fe36 = fe32!(36, 223);
    let fe111 = fe32!(111, 223);
    assert_eq!(res.x, Some(fe36));
    assert_eq!(res.y, Some(fe111));
}

#[test]
fn ex04_04() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);

    let fe47 = fe32!(47, 223);
    let fe71 = fe32!(71, 223);
    let p1 = p_fe32!(fe47, fe71, fe0, fe7);

    let res = p1.smul(4);

    let fe194 = fe32!(194, 223);
    let fe51 = fe32!(51, 223);
    assert_eq!(res.x, Some(fe194));
    assert_eq!(res.y, Some(fe51));
}

#[test]
fn ex04_05() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);

    let fe47 = fe32!(47, 223);
    let fe71 = fe32!(71, 223);
    let p1 = p_fe32!(fe47, fe71, fe0, fe7);

    let res = p1.smul(8);

    let fe116 = fe32!(116, 223);
    let fe55 = fe32!(55, 223);
    assert_eq!(res.x, Some(fe116));
    assert_eq!(res.y, Some(fe55));
}

#[test]
fn ex04_06() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);

    let fe47 = fe32!(47, 223);
    let fe71 = fe32!(71, 223);
    let p1 = p_fe32!(fe47, fe71, fe0, fe7);

    let res = p1.smul(21);

    assert_eq!(res.x, None);
    assert_eq!(res.y, None);
}

#[test]
fn ex05() {
    let fe0 = fe32!(0, 223);
    let fe7 = fe32!(7, 223);

    let fe15 = fe32!(15, 223);
    let fe86 = fe32!(86, 223);
    let p1 = p_fe32!(fe15, fe86, fe0, fe7);
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

#[test]
fn ex07() {
    let e = PrivateKey::new(U256::from_u32(12345u32));
    let z = hash256(b"Programming Bitcoin!");
    let z_u256 = U256::from_be_slice(&z);

    let sig = e.sign(z_u256);

    assert_eq!(hex::encode(z), "969f6056aa26f7d2795fd013fe88868d09c9f6aed96965016e1936ae47060d48");
    assert_eq!(hex::encode(sig.r.num.to_be_bytes()), "2b698a0f0a4041b77e63488ad48c23e8e8838dd1fb7520408b121697b782ef22");
    assert_eq!(hex::encode(sig.s.num.to_be_bytes()), "1dbc63bfef4416705e602a7b564161167076d8b20990a0f26f316cff2cb0bc1a");
}