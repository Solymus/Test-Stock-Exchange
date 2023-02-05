#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Order {
    pub id: i32,
    pub user_id: String,
    pub order_type: OrderType,
    pub symbol: String,
    pub price: i32,
    pub quantity: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum OrderType {
    Buy,
    Sell,
}