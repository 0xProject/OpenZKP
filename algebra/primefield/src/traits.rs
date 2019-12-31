/// Primitive roots of unity
pub trait Root<Order>: Sized {
    fn root(order: Order) -> Option<Self>;
}
