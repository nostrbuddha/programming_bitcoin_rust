use prog_bitcoin::p32;

#[test]
fn ex01_01() {
    let p_m1_m1 = p32!(-1, -1, 5, 7);
    assert_eq!(p_m1_m1.x, Some(-1));
    assert_eq!(p_m1_m1.y, Some(-1));
    let p_18_77 = p32!(18, 77, 5, 7);
    assert_eq!(p_18_77.x, Some(18));
    assert_eq!(p_18_77.y, Some(77));
}

#[test]
#[should_panic(expected = "is not on the curve")]
fn ex01_02() {
    let _ = p32!(2, 4, 5, 7);
}

#[test]
#[should_panic(expected = "is not on the curve")]
fn ex01_03() {
    let _ = p32!(5, 7, 5, 7);
}

#[test]
fn ex04() {
    let pt1 = p32!(2, 5, 5, 7);
    let pt2 = p32!(-1, -1, 5, 7);
    
    let res = pt1 + pt2;
    assert_eq!(res.x, Some(3));
    assert_eq!(res.y, Some(-7));
}

#[test]
fn ex06() {
    let pt1 = p32!(-1, -1, 5, 7);
    let pt2 = p32!(-1, -1, 5, 7);
    
    let res = pt1 + pt2;
    assert_eq!(res.x, Some(18));
    assert_eq!(res.y, Some(77));
}
