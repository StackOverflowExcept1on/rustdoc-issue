pub trait Trait {}

/// link1 [impl<G1, G2> Trait for Type<G1, G2>](#impl-Trait-for-Type<G1, G2>)
///
/// link2 [impl<G1, G2> Trait for Type<G1, G2>](#impl-Trait-for-Type<G1%2C%20G2>)
pub struct Type<G1, G2>(G1, G2);

impl<G1, G2> Trait for Type<G1, G2> {}
