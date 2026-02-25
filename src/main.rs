use prog_bitcoin::primitives::field_element_u32::FieldElementU32;

fn main() {
    let fe1 = FieldElementU32::new(2, 7);
    let fe2 = FieldElementU32::new(2, 7);
    let fe3 = FieldElementU32::new(3, 7);
    // let fe4 = FieldElement::new(4, 7);
    // let fe5 = FieldElement::new(5, 7);
    let fe6 = FieldElementU32::new(6, 7);
    let fea = FieldElementU32::new(2, 8);
    println!("{fe1:?}");
    println!("{fe2:?}");
    println!("{fe3:?}");
    println!("{fea:?}");
    
    println!("fe1 and fe2: {}", fe1 == fe2);
    println!("fe1 and fe3: {}", fe1 == fe3);
    println!("fe1 and fea: {}", fe1 == fea);

    println!("fe1 + fe2: {:?}", fe1 + fe2);
    println!("fe1 + fe2 + fe2: {:?}", fe1 + fe2 + fe2);
    println!("fe1 + fe2 + fe2 + fe2: {:?}", fe1 + fe2 + fe2 + fe2);

    println!("fe6 - fe2: {:?}", fe6 - fe2);
    // println!("fe1 + fea: {:?}", fe1 + fea);

    let fe_31_3 = FieldElementU32::new(3, 31);
    let fe_31_24 = FieldElementU32::new(24, 31);

    println!("fe_3 / fe_24: {:?}", fe_31_3 / fe_31_24);
}

