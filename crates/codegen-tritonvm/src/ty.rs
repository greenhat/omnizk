use twenty_first::shared_math::b_field_element::BFieldElement;

pub fn felt_i32(v: i32) -> BFieldElement {
    if v < 0 {
        BFieldElement::new(BFieldElement::P - (v.unsigned_abs() as u64))
    } else {
        BFieldElement::new(v as u64)
    }
}

pub fn felt_i64(v: i64) -> BFieldElement {
    if v < 0 {
        BFieldElement::new(BFieldElement::P - (v.unsigned_abs() as u64))
    } else {
        BFieldElement::new(v as u64)
    }
}
