use prog_bitcoin::primitives::point_num::PointNum;

#[test]
fn ex01_01() {
    let p_m1_m1 = PointNum::new_concrete(-1, -1, 5, 7);
    assert_eq!(p_m1_m1.x, Some(-1));
    assert_eq!(p_m1_m1.y, Some(-1));
    let p_18_77 = PointNum::new_concrete(18, 77, 5, 7);
    assert_eq!(p_18_77.x, Some(18));
    assert_eq!(p_18_77.y, Some(77));
}

#[test]
#[should_panic(expected = "is not on the curve")]
fn ex01_02() {
    let _ = PointNum::new_concrete(2, 4, 5, 7);
}

#[test]
#[should_panic(expected = "is not on the curve")]
fn ex01_03() {
    let _ = PointNum::new_concrete(5, 7, 5, 7);
}

#[test]
fn ex04() {
    let pt1 = PointNum::new_concrete(2, 5, 5, 7);
    let pt2 = PointNum::new_concrete(-1, -1, 5, 7);
    
    let res = pt1 + pt2;
    assert_eq!(res.x, Some(3));
    assert_eq!(res.y, Some(-7));
}

/*
TODO: To Check
#[test]
fn ex06() {
    let pt1 = PointNum::new_concrete(-1, -1, 5, 7);
    let pt2 = PointNum::new_concrete(-1, -1, 5, 7);
    
    let res = pt1 + pt2;
    assert_eq!(res.x, Some(18));
    assert_eq!(res.y, Some(-77));
}
*/
