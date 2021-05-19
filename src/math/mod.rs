use std::error;
use std::fmt;
use std::ops;
use num_bigint::BigInt;
use num_integer::Integer;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    num: BigInt,
    prime: BigInt
}

impl FieldElement {
    pub fn pow<T: Into<BigInt>>(&self, exp: T) -> FieldElement {
        let e: BigInt = exp.into().clone();
        FieldElement::new(self.num.modpow(&e, &self.prime), self.prime.clone()).unwrap()
    }

    
    pub fn div<T: Into<BigInt>>(&self, divisor: T) -> FieldElement {
        // Since a/b = a * b^-1 
        // and b^(p-1) = 1 (Fermat's Little Theorem
        // and b^-1 = b^-1*1 = b^-1 * b^(p-1) = b^(p-2) 
        // then
        // a / b = a * b^(p-2)
        let divisor_bi: BigInt = divisor.into().clone();
        let divisor_inv_exp: BigInt = divisor_bi.modpow(&(&self.prime - 2), &self.prime);
        let mult = &self.num * &divisor_inv_exp;
        FieldElement::new(mult.mod_floor(&self.prime), self.prime.clone()).unwrap()
    }
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Regular(ErrorKind)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Regular(ref err) => write!(f, "regular error occured {:?}", err)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Regular(ref err) => err.as_str()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    OutOfRange
}

impl ErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            ErrorKind::OutOfRange  => 
                "Parameters out of range"
        }
    }
}


impl FieldElement {
    pub fn new<T: Into<BigInt> + Clone>(num: T, prime: T) -> Result<FieldElement>  {
        let n : BigInt = num.clone().into();
        let p : BigInt = prime.clone().into();
        if n >= p {
            Err(Error::Regular(ErrorKind::OutOfRange))
        } else {
            Ok(FieldElement{ num: n, prime: p })
        }
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement({},{})", self.num, self.prime)

    }
}

/// Math operations

// &T + U
// see rust/src/libcore/ops.rs forward_ref_binop and add_impl macros for a better but advanced way
/*
impl<'a> ops::Add<FieldElement> for &'a FieldElement {
    type Output = <FieldElement as ops::Add<FieldElement>>::Output;
    fn add(self, other: FieldElement) -> <FieldElement as ops::Add<FieldElement>>::Output {
        ops::Add::add(self.clone(), other.clone())
    }
}
*/

// T + &U
/*
impl<'a> ops::Add<&'a FieldElement> for FieldElement {
    type Output = <FieldElement as ops::Add<FieldElement>>::Output;
    fn add(self, other: &'a FieldElement) -> <FieldElement as ops::Add<FieldElement>>::Output {
        ops::Add::add(self.clone(), other.clone())
    }
}
*/

impl<'a> ops::Add<&'a FieldElement> for FieldElement {
    type Output = FieldElement;
    fn add(self, other: &'a FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("FieldElement add not equal order self: {} other: {}", self.prime, other.prime);
        }
        FieldElement::new((self.num + &other.num).mod_floor(&self.prime), self.prime).unwrap()
    }
}

// &T + &U
impl<'a> ops::Add<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn add(self, other: &'a FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("FieldElement add not equal order self: {} other: {}", self.prime, other.prime);
        }
        FieldElement::new((&self.num + &other.num).mod_floor(&self.prime), self.prime.clone()).unwrap()
    }
}

// T - &U
impl<'a> ops::Sub<&'a FieldElement> for FieldElement {
    type Output = FieldElement;
    fn sub(self, other: &'a FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("FieldElement add not equal order self: {} other: {}", self.prime, other.prime);
        }
        FieldElement::new((self.num - &other.num).mod_floor(&self.prime), self.prime).unwrap()
    }
}

// &T - &U
impl<'a> ops::Sub<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn sub(self, other: &'a FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("FieldElement add not equal order self: {} other: {}", self.prime, other.prime);
        }
        FieldElement::new((&self.num - &other.num).mod_floor(&self.prime), self.prime.clone()).unwrap()
    }
}

// T * &U
impl<'a> ops::Mul<&'a FieldElement> for FieldElement {
    type Output = FieldElement;
    fn mul(self, other: &'a FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("FieldElement add not equal order self: {} other: {}", self.prime, other.prime);
        }
        FieldElement::new((self.num * &other.num).mod_floor(&self.prime), self.prime).unwrap()
    }
}

// &T * &U
impl<'a> ops::Mul<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn mul(self, other: &'a FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("FieldElement add not equal order self: {} other: {}", self.prime, other.prime);
        }
        FieldElement::new((&self.num * &other.num).mod_floor(&self.prime), self.prime.clone()).unwrap()
    }
}


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
}

#[test]
fn field_element_div() {
    let a = FieldElement::new(1,7).unwrap();
    let b = a.div(3);
    assert_eq!(b, FieldElement::new(5, 7).unwrap());
}


