use std::collections::HashMap;
use crate::order::{Order};


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Broker {
    /// A HashMap of user IDs to their balances.
    pub users: HashMap<String, i32>,
    /// A HashMap of order IDs to the Order structs.
    pub open_orders: HashMap<i32, Order>,
    /// The maximum order ID that has been assigned.
    max_order_id: i32,
}


impl Broker {

    /// Create new Broker
    pub fn new() -> Broker {
        Broker {
            users: HashMap::new(),
            open_orders: HashMap::new(),
            max_order_id: 0,
        }
    }

    /// Adds a new user to the Broker.
    ///
    /// # Arguments
    /// * `user_id` - A string representing the user's ID.
    pub fn add_user(&mut self, user_id: String) {
        self.users.insert(user_id, 0);
    }

    /// Retrieves the balance of the specified user.
    ///
    /// # Arguments
    /// * `user_id` - A string slice representing the user's ID.
    ///
    /// # Returns
    /// An Option type that either contains the user's balance or None if the user was not found.
    pub fn get_balance(&self, user_id: &str) -> Option<i32> {
        self.users.get(user_id).cloned()
    }

    /// Updates the balance of the specified user.
    ///
    /// # Arguments
    /// * `user_id` - A string slice representing the user's ID.
    /// * `delta` - An integer representing the amount to add (positive) or subtract (negative) from the user's balance.
    pub fn update_balance(&mut self, user_id: &str, delta: i32) {
        let balance = self.get_balance(user_id);
        if let Some(b) = balance {
            self.users.insert(user_id.to_owned(), b + delta);
        }
    }

    /// Adds an Order
    ///
    /// # Arguments
    /// * `order` - The Order struct to be added.
    pub fn place_order(&mut self, order: Order) {
        self.open_orders.insert(self.max_order_id, order);
    }

    /// Adds a new Order, assigning it a unique ID.
    ///
    /// # Arguments
    /// * `order` - The Order struct to be added.
    ///
    /// # Returns
    /// An integer representing the new Order's ID.
    pub fn place_new_order(&mut self, mut order: Order) -> i32 {
        order.id = self.max_order_id;
        self.place_order(order);
        self.max_order_id += 1;
        return self.max_order_id - 1;
    }


    /// Cancels an Order
    ///
    /// # Arguments
    /// * `order_id` - An integer representing the ID of the Order to be cancelled.
    pub fn cancel_order(&mut self, order_id: i32) {
        if self.open_orders.get(&order_id).is_some() {
            self.open_orders.remove(&order_id);
        }
    }

    /// Checks if the Broker has an Order
    ///
    /// # Arguments
    /// * `order_id` - An integer representing the ID of the Order to check for.
    ///
    /// # Returns
    /// A boolean value indicating whether the Order with the specified ID exists in the open_orders HashMap.
    pub fn has_order(&self, order_id: i32) -> bool {
        return self.open_orders.get(&order_id).is_some()
    }

    /// Retrieves all open Orders
    ///
    /// # Returns
    /// A vector of Order structs representing all open Orders.
    pub fn get_open_orders(&self) -> Vec<Order> {
        self.open_orders.values().cloned().collect()
    }
}