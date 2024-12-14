pub struct Assert<const p: bool>;

pub trait IsTrue {}

impl IsTrue for Assert<true> {}