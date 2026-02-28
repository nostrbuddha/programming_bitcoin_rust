use crypto_bigint::{I256, U256};
use prog_bitcoin::{algorithms::{base58::encode_base58, endian::{int_to_little_endian, little_endian_to_int}, hash256::hash256}, s256::{private_key::PrivateKey, s256_field::S256Field, signature::Signature}};

#[test]
fn ex01_01() {
    let secret = PrivateKey::new(U256::from_u32(5000u32));
    let pub_key = secret.point.sec(false);
    assert_eq!(hex::encode(pub_key), "04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10");
}

#[test]
fn ex01_02() {
    let n_2018 = S256Field::new(U256::from_u32(2018u32));
    let n_2018_p5 = n_2018.pow(I256::from(5));
    let secret = PrivateKey::new(n_2018_p5.num);
    let pub_key = secret.point.sec(false);
    assert_eq!(hex::encode(pub_key), "04027f3da1918455e03c46f659266a1bb5204e959db7364d2f473bdf8f0a13cc9dff87647fd023c13b4a4994f17691895806e1b40b57f4fd22581a4f46851f3b06");
}

#[test]
fn ex01_03() {
    let secret = PrivateKey::new(U256::from_be_hex("000000000000000000000000000000000000000000000000000deadbeef12345"));
    let pub_key = secret.point.sec(false);
    assert_eq!(hex::encode(pub_key), "04d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd59961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121");
}

#[test]
fn ex02_01() {
    let secret = PrivateKey::new(U256::from_u32(5001u32));
    let pub_key = secret.point.sec(true);
    assert_eq!(hex::encode(pub_key), "0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1");
}

#[test]
fn ex02_02() {
    let n_2018 = S256Field::new(U256::from_u32(2019u32));
    let n_2018_p5 = n_2018.pow(I256::from(5));
    let secret = PrivateKey::new(n_2018_p5.num);
    let pub_key = secret.point.sec(true);
    assert_eq!(hex::encode(pub_key), "02933ec2d2b111b92737ec12f1c5d20f3233a0ad21cd8b36d0bca7a0cfa5cb8701");
}

#[test]
fn ex02_03() {
    let secret = PrivateKey::new(U256::from_be_hex("000000000000000000000000000000000000000000000000000deadbeef54321"));
    let pub_key = secret.point.sec(true);
    assert_eq!(hex::encode(pub_key), "0296be5b1292f6c856b3c5654e886fc13511462059089cdf9c479623bfcbe77690");
}

#[test]
fn ex03_01() {
    let sig= Signature {
        r: S256Field::new(U256::from_be_hex("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6")),
        s: S256Field::new(U256::from_be_hex("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec")),
    };
    let sig_der = sig.der();
    assert_eq!(hex::encode(sig_der), "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
}

#[test]
fn ex04_01() {
    let st = hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap();
    let res = encode_base58(&st);
    assert_eq!(res, "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6");
}

#[test]
fn ex04_02() {
    let st = hex::decode("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c").unwrap();
    let res = encode_base58(&st);
    assert_eq!(res, "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd");
}
#[test]
fn ex04_03() {
    let st = hex::decode("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap();
    let res = encode_base58(&st);
    assert_eq!(res, "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7");
}

#[test]
fn ex05_01() {
    let secret = PrivateKey::new(U256::from_u32(5002u32));
    let pubkey = secret.point;
    let address = pubkey.address(false, true);
    assert_eq!(address, "mmTPbXQFxboEtNRkwfh6K51jvdtHLxGeMA");
}

#[test]
fn ex05_02() {
    let n_2020 = S256Field::new(U256::from_u32(2020u32));
    let n_2020_p5 = n_2020.pow(I256::from(5));
    let secret = PrivateKey::new(n_2020_p5.num);
    let pubkey = secret.point;
    let address = pubkey.address(true, true);
    assert_eq!(address, "mopVkxp8UhXqRYbCYJsbeE1h1fiF64jcoH");
}

#[test]
fn ex05_03() {
    let secret = PrivateKey::new(U256::from_be_hex("00000000000000000000000000000000000000000000000000012345deadbeef"));
    let pubkey = secret.point;
    let address = pubkey.address(true, false);
    assert_eq!(address, "1F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1");
}

#[test]
fn ex06_01() {
    let secret = PrivateKey::new(U256::from_u32(5003u32));
    let wif = secret.wif(true, true);
    assert_eq!(wif, "cMahea7zqjxrtgAbB7LSGbcQUr1uX1ojuat9jZodMN8rFTv2sfUK");
}

#[test]
fn ex06_02() {
    let n_2021 = S256Field::new(U256::from_u32(2021u32));
    let n_2021_p5 = n_2021.pow(I256::from(5));
    let secret = PrivateKey::new(n_2021_p5.num);
    let wif = secret.wif(false, true);
    assert_eq!(wif, "91avARGdfge8E4tZfYLoxeJ5sGBdNJQH4kvjpWAxgzczjbCwxic");
}

#[test]
fn ex06_03() {
    let secret = PrivateKey::new(U256::from_be_hex("00000000000000000000000000000000000000000000000000054321deadbeef"));
    let wif = secret.wif(true, false);
    assert_eq!(wif, "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgiuQJv1h8Ytr2S53a");
}

#[test]
fn ex07() {
    let bytes = [0xB1, 0x07];
    let number = little_endian_to_int(&bytes);
    assert_eq!(number, U256::from_u32(1969u32));
}

#[test]
fn ex08() {
    let bytes = int_to_little_endian(U256::from_u32(1969u32),32);
    assert_eq!(bytes[0], 0xB1);
    assert_eq!(bytes[1], 0x07);
}

#[test]
fn ex09() {
    let my_secret = b"satoshi_rocks_big_time";

    let hash_of_secret = hash256(my_secret);
    let pv_key = PrivateKey::new(little_endian_to_int(&hash_of_secret));
    let pub_key = pv_key.point;

    let address = pub_key.address(true, true);

    assert_eq!(address, "mmvuTBC7D6rWyjHnd3BJRSFvX1jQVvw4SV");
}