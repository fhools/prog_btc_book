use super::FieldElement;
use num_bigint::BigInt;
use std::ops::Add;

#[derive(Debug, Clone, PartialEq, Eq)]
struct FieldPoint {
    x:  FieldElement,
    y:  FieldElement,
    a:  FieldElement,
    b:  FieldElement,
    inf: bool,
}


impl FieldPoint {
    fn new(x: &FieldElement, y: &FieldElement, a: &FieldElement, b: &FieldElement) -> Result<FieldPoint, String> {

        if y.pow(2) != x.pow(3)  + &(a * x) + b {
            Err(format!("{}, {} is not on curve (a: {}, b: {})", x, y, a, b))
        } else {
            Ok(FieldPoint {
                x: x.clone(), 
                y: y.clone(),
                a: a.clone(),
                b: b.clone(),
                inf: false,
            })
        }
    }


    fn new_inf(a: &FieldElement, b: &FieldElement) -> Result<FieldPoint, String> {
        Ok(FieldPoint {
            x: FieldElement::new(0,1).unwrap(),
            y: FieldElement::new(0,1).unwrap(),
            a: a.clone(),
            b: b.clone(),
            inf: true,
        })
    }
}

impl Add<&FieldPoint> for FieldPoint {
    type Output = FieldPoint;
    fn add(self, other: &FieldPoint) -> FieldPoint {
        if self.a != other.a || self.b != other.b {
            panic!(format!("cannot add 2 field points not on the same curve \
point 1: (a: {:?} b: {:?})\n\
point2: ({:?}. {:?})", self.a, self.b, other.a, other.b));
        } 

        if self.inf {
            return other.clone();
        }

        if other.inf {
            return self.clone();
        }

        if  self.x != other.x {
            let slope = &(&other.y - &self.y).div_field(&(&other.x - &self.x));
            let x3 = &(slope.pow(2) - &self.x) - &other.x;
            let y3 = &(slope * &(self.x - &x3)) - &self.y;
            return FieldPoint::new(&x3, &y3, &self.a, &self.b).unwrap();
        } else if self.y.num == BigInt::from(0) { 
            return FieldPoint::new_inf(&self.a, &self.b).unwrap();
        } else {
        // Point 1 = Point 2
            println!("same point: {:?}", self);
            let slope = &(&self.x.pow(2) * &BigInt::from(3) + &self.a).div_field(&(&self.y * &BigInt::from(2)));
            let x3 = &slope.pow(2) - &(&self.x * &BigInt::from(2));
            let y3 = &(slope * &(&self.x - &x3)) - &self.y;
            return FieldPoint::new(&x3, &y3, &self.a, &self.b).unwrap();
        }
    }
}

#[test]
fn point_new() {
    let x = FieldElement::new(1, 7).unwrap();
    let y = FieldElement::new(2,7).unwrap();
    let a = FieldElement::new(3,7).unwrap();
    let b = FieldElement::new(4,7).unwrap();
    let point = FieldPoint::new(&x, &y, &a, &b);
    println!("{:?}", point);
    assert!(point.is_err());

    let x = FieldElement::new(192, 223).unwrap();
    let y = FieldElement::new(105,223).unwrap();
    let a = FieldElement::new(0,223).unwrap();
    let b = FieldElement::new(7,223).unwrap();
    let point = FieldPoint::new(&x, &y, &a, &b);
    println!("{:?}", point);
    assert!(point.is_ok());
}

#[test]
fn point_add_point_to_inf() {
    let x = FieldElement::new(192, 223).unwrap();
    let y = FieldElement::new(105,223).unwrap();
    let a = FieldElement::new(0,223).unwrap();
    let b = FieldElement::new(7,223).unwrap();
    let point = FieldPoint::new(&x, &y, &a, &b).unwrap();

    let point2 = FieldPoint::new_inf(&a, &b).unwrap();

    let point3 = point.clone() + &point2; 
    assert_eq!(point, point3);
}
