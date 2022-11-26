use twenty_first::shared_math::b_field_element::BFieldElement;

pub fn felt(v: i32) -> BFieldElement {
    // TODO: implement according to https://github.com/Neptune-Crypto/twenty-first/issues/32
    BFieldElement::new(v as u64)
}
