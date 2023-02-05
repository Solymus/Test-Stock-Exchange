use std::collections::HashMap;
use crate::order::{Order};


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Broker {
    pub users: HashMap<String, i32>,
    pub open_orders: HashMap<i32, Order>,
    max_order_id: i32,
}

impl Broker {
    pub fn new() -> Broker {
        Broker {
            users: HashMap::new(),
            open_orders: HashMap::new(),
            max_order_id: 0,
        }
    }

    pub fn add_user(&mut self, user_id: String) {
        self.users.insert(user_id, 0);
    }

    pub fn get_balance(&self, user_id: &str) -> Option<i32> {
        self.users.get(user_id).cloned()
    }

    pub fn update_balance(&mut self, user_id: &str, delta: i32) {
        let balance = self.get_balance(user_id);
        if let Some(b) = balance {
            self.users.insert(user_id.to_owned(), b + delta);
        }
    }

    pub fn place_order(&mut self, order: Order) {
        self.open_orders.insert(self.max_order_id, order);
    }

    pub fn place_new_order(&mut self, mut order: Order) -> i32 {
        order.id = self.max_order_id;
        self.place_order(order);
        self.max_order_id += 1;
        return self.max_order_id - 1;
    }

    pub fn cancel_order(&mut self, order_id: i32, user_id: &str) {
        if let Some(order) = self.open_orders.get(&order_id) {
            self.open_orders.remove(&order_id);
        }
    }

    pub fn has_order(&self, order_id: i32) -> bool {
        return self.open_orders.get(&order_id).is_some()
    }

    pub fn get_open_orders(&self) -> Vec<Order> {
        self.open_orders.values().cloned().collect()
    }
}