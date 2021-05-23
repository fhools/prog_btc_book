use std::error;
use std::fmt;
use std::ops;
use num_bigint::BigInt;
use num_integer::Integer;

mod field_element;
pub use field_element::*;

mod ecc;

#[test]
fn add_fieldelement() {
    let fe1 = FieldElement::new(3, 5).unwrap();
    let fe2 = FieldElement::new(3, 5).unwrap();
    assert_eq!(&fe1 + &fe2, FieldElement::new(1,5).unwrap());
}
#[test]
fn add_fieldelement_with_ref() {
    let fe1 = FieldElement::new(3, 5).unwrap();
    let fe2 = FieldElement::new(3, 5).unwrap();
    assert_eq!(&fe1 + &fe2, FieldElement::new(1,5).unwrap());
}
#[test]
fn add_fieldelement_moved_ref() {

    // Test T + &U
    let fe1 = FieldElement::new(3, 7).unwrap();
    let fe2 = FieldElement::new(3, 7).unwrap();
    let fe3 = fe1 + &fe2;
    println!("fe2: {}", fe2);
    println!("fe3: {}", fe3);
}

#[test]
#[should_panic]
fn add_fieldelement_panic() {
    let fe1 = FieldElement::new(3, 5).unwrap();
    let fe2 = FieldElement::new(3, 7).unwrap();
    let fe3 = fe1 + &fe2;
    println!("fe2: {}", fe2);
}

#[test]
fn sub_fieldelement() {
    let fe1 = FieldElement::new(3, 5).unwrap();
    let fe2 = FieldElement::new(3, 5).unwrap();
    assert_eq!(&fe1 - &fe2, FieldElement::new(0,5).unwrap());

    assert_eq!(FieldElement::new(1,7).unwrap() - &FieldElement::new(5,7).unwrap(), FieldElement::new(3,7).unwrap());
}

#[test]
fn sub_fieldelement_with_ref() {
    let fe1 = FieldElement::new(4, 5).unwrap();
    let fe2 = FieldElement::new(2, 5).unwrap();
    assert_eq!(&fe1 - &fe2, FieldElement::new(2,5).unwrap());
}

#[test]
fn sub_fieldelement_moved_ref() {

    // Test T + &U
    let fe1 = FieldElement::new(6, 7).unwrap();
    let fe2 = FieldElement::new(1, 7).unwrap();
    let fe3 = fe1 - &fe2;
    assert_eq!(fe3, FieldElement::new(5, 7).unwrap())
}

#[test]
fn field_element_pow() {
    let a = FieldElement::new(3,7).unwrap();
    let b = a.pow(2);
    assert_eq!(b, FieldElement::new(2,7).unwrap());


    let c = a.pow(-3);
    assert_eq!(c, FieldElement::new(6,7).unwrap());
    eprintln!("pow c: {:?}", c);
}

#[test]
fn field_element_div() {
    let a = FieldElement::new(1,7).unwrap();
    let b = a.div(3);
    assert_eq!(b, FieldElement::new(5, 7).unwrap());
}


