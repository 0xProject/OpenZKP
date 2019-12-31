/// Primitive roots of unity
// TODO: Rename primitive_root ?
pub trait Root<Order>: Sized {
    fn root(order: Order) -> Option<Self>;
}

/// Square roots
pub trait SquareRoot: Sized {
    fn is_quadratic_residue(&self) -> bool;

    fn square_root(&self) -> Option<Self>;
}
