use std::error;
use std::fmt;
use std::ops;
use num_bigint::BigInt;
use num_integer::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    pub num: BigInt,
    pub prime: BigInt
}

impl FieldElement {
    pub fn pow<T: Into<BigInt>>(&self, exp: T) -> FieldElement {
        let mut e: BigInt = exp.into().clone();

        // Handle negative exponent
        // modpow does not let us have negative exponent
        // We can use fermat's little theorem here.
        // x^-n = x^-n * 1 =  x^-n * x^(p-1) = x^(p-n-1) 
        if e < BigInt::from(0) {
            e += &self.prime - &1;
        }
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
    pub fn div_field(&self, divisor: &FieldElement) -> FieldElement {
        // Since a/b = a * b^-1 
        // and b^(p-1) = 1 (Fermat's Little Theorem
        // and b^-1 = b^-1*1 = b^-1 * b^(p-1) = b^(p-2) 
        // then
        // a / b = a * b^(p-2)
        let divisor_bi: BigInt = divisor.num.clone();
        let divisor_inv_exp: BigInt = divisor_bi.modpow(&(&self.prime - 2), &self.prime);
        let mult = &self.num * &divisor_inv_exp;
        FieldElement::new(mult.mod_floor(&self.prime), self.prime.clone()).unwrap()
    } 
}

impl From<(i64, i64)> for FieldElement {
    fn from(tupl: (i64,i64)) -> FieldElement {
        FieldElement::new(tupl.0, tupl.1).unwrap()
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

impl<'a> ops::Mul<&'a BigInt> for &FieldElement {
    type Output = FieldElement;
    fn mul(self, other: &BigInt) -> FieldElement {
        let operand : BigInt  = other.clone();
        let fother = FieldElement::new(operand, self.prime.clone()).unwrap();
        return self * &fother;
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

