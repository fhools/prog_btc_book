mod math;

use math::{FieldElement};

fn main() {
    let field_el = FieldElement::new(2, 11);

    match field_el {
        Ok(field_el) => println!("Created {}", field_el),
        Err(err) => println!("Got err: {:?}", err)
    }
}

