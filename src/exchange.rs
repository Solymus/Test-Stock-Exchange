use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::broker::*;
use crate::order::*;

pub struct Exchange {
    pub asset_balances: HashMap<String, HashMap<String, i32>>,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Trade {
    pub symbol: String,
    pub price: i32,
    pub quantity: i32,
    pub buy_order_id: i32,
    pub sell_order_id: i32,
}

impl Exchange {

    pub fn new() -> Self {
        Self { asset_balances: HashMap::new(), trades: Vec::new() }
    }

    pub fn get_asset_price(&self, symbol: &str) -> Option<i32> {
        let asset_trades = self.get_asset_trades(symbol);
        if asset_trades.is_empty() {
            None
        } else {
            Some(asset_trades.last().unwrap().price)
        }
    }

    pub fn get_asset_trades(&self, symbol: &str) -> Vec<Trade> {
        self.trades
            .iter()
            .filter(|trade| trade.symbol == symbol)
            .cloned()
            .collect()
    }

    pub fn find_trades_for_order(&self, order_id: i32) -> Vec<Trade> {
        self.trades
            .iter()
            .filter(|trade| trade.buy_order_id == order_id || trade.sell_order_id == order_id)
            .cloned()
            .collect()
    }

    fn execute_trade(&mut self, buy_order: &Order, sell_order: &Order, broker: &mut Broker) -> bool {

        if buy_order.symbol != sell_order.symbol {
            return false;
        }

        let executed_quantity = std::cmp::min(buy_order.quantity, sell_order.quantity);

        let remaining_quantity_buy = buy_order.quantity - executed_quantity;
        let remaining_quantity_sell = sell_order.quantity - executed_quantity;

        if let Some(buyer_balance) = broker.get_balance(&buy_order.user_id) {
            if buyer_balance < executed_quantity * buy_order.price {
                broker.open_orders.remove(&buy_order.id);
                return false;
            }
        } else {
            broker.open_orders.remove(&buy_order.id);
            return false;
        }

        if let Some(seller_balance) = self.get_asset_balance(&sell_order.user_id, &sell_order.symbol) {
            if seller_balance < sell_order.quantity {
                broker.open_orders.remove(&sell_order.id);
                return false;
            }
        } else {
            broker.open_orders.remove(&sell_order.id);
            return false;
        }

        broker.update_balance(&buy_order.user_id, -executed_quantity * buy_order.price);
        broker.update_balance(&sell_order.user_id, executed_quantity * buy_order.price);

        self.update_asset_balance(buy_order.symbol.clone(), &buy_order.user_id, executed_quantity);
        self.update_asset_balance(buy_order.symbol.clone(), &sell_order.user_id, -executed_quantity);


        let trade = Trade {
            symbol: buy_order.symbol.clone(),
            price: buy_order.price,
            quantity: executed_quantity,
            buy_order_id: buy_order.id,
            sell_order_id: sell_order.id,
        };
        self.trades.push(trade);

        broker.open_orders.remove(&buy_order.id);
        broker.open_orders.remove(&sell_order.id);

        if remaining_quantity_buy > 0 {
            let new_buy_order = Order {
                id: buy_order.id,
                user_id: buy_order.user_id.clone(),
                order_type: OrderType::Buy,
                symbol: buy_order.symbol.clone(),
                price: buy_order.price,
                quantity: remaining_quantity_buy,
            };
            broker.place_order(new_buy_order);
        }

        if remaining_quantity_sell > 0 {
            let new_sell_order = Order {
                id: sell_order.id,
                user_id: sell_order.user_id.clone(),
                order_type: OrderType::Sell,
                symbol: sell_order.symbol.clone(),
                price: sell_order.price,
                quantity: remaining_quantity_sell,
            };
            broker.place_order(new_sell_order);
        }

        executed_quantity > 0
    }

    pub fn match_orders(&mut self, broker: &mut Broker) {
        let mut buy_orders = Vec::new();
        let mut sell_orders = Vec::new();

        for (_, order) in &broker.open_orders {
            match order.order_type {
                OrderType::Buy => buy_orders.push(order.clone()),
                OrderType::Sell => sell_orders.push(order.clone()),
            }
        }

        buy_orders.sort_by_key(|o| o.price);
        sell_orders.sort_by_key(|o| -o.price);

        let mut i = 0;
        let mut j = 0;
        while i < buy_orders.len() && j < sell_orders.len() {
            let buy_order = &buy_orders[i];
            let sell_order = &sell_orders[j];
            if buy_order.price >= sell_order.price {
                if self.execute_trade(buy_order, sell_order, broker) {
                    i += 1;
                    j += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }


    pub fn get_asset_balance(&self, user_id: &String, symbol: &str) -> Option<i32> {
        if let Some(user_balance) = self.asset_balances.get(user_id) {
            return user_balance.get(symbol).cloned();
        } else {
            return None;
        }
    }

    pub fn update_asset_balance(&mut self, symbol: String, user_id: &String, delta: i32) {
        let balance = self.get_asset_balance(user_id, &symbol);

        if self.asset_balances.get_mut(user_id).is_none() {
            self.asset_balances.insert(user_id.into(), HashMap::new());
        }

        let user_balance = self.asset_balances.get_mut(user_id).unwrap();

        if let Some(b) = balance {
            user_balance.insert(symbol, b + delta);
        } else {
            user_balance.insert(symbol, delta);
        }
    }
    

}
