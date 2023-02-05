#![feature(decl_macro)]
#[macro_use] extern crate rocket_contrib;

mod exchange;
mod broker;
mod order;

use crate::exchange::*;
use crate::broker::*;
use crate::order::*;

use rocket::{get, post, routes, State};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::sync::{Arc, Mutex};


type BrokerState<'a> = State<'a, Arc<Mutex<Broker>>>;
type ExchangeState<'a> = State<'a, Arc<Mutex<Exchange>>>;


#[derive(Serialize, Deserialize, Debug)]
struct OrderInfo {
    user_id: String,
    order_type: String,
    symbol: String,
    price: i32,
    quantity: i32,
}

/*
    Rocket Routes
*/

// curl -X POST -i http://localhost:8000/user/
#[post("/user")]
fn create_user(broker: BrokerState) -> JsonValue {
    let user_id = Uuid::new_v4().to_string();
    broker.lock().unwrap().add_user(user_id.clone());
    json!({ "user_id": user_id })
}

// curl -X GET -i http://localhost:8000/user/ab2e52ce-b9a2-4fa5-a118-a5349396b82e/exists/
#[get("/user/<user_id>/exists")]
fn user_exists(broker: BrokerState, user_id: String) -> JsonValue {
    let exists = broker.lock().unwrap().get_balance(&user_id).is_some();
    json!({ "exists": exists })
}

/*
    curl -X POST -i http://localhost:8000/order/place/ --data '{
        "user_id": "ab2e52ce-b9a2-4fa5-a118-a5349396b82e",
        "order_type": "Buy",
        "symbol": "AAPL",
        "price": 100,
        "quantity": 1
    }'
*/
#[post("/order/place", data = "<order_info>")]
fn place_order(broker: BrokerState, exchange: ExchangeState, order_info: Json<OrderInfo>) -> JsonValue {

    let order_type = match &order_info.order_type[..] {
        "Buy" => OrderType::Buy,
        _ => OrderType::Sell,
    };

    let new_order = Order {
        id: 0,
        user_id: order_info.user_id.clone(),
        order_type,
        symbol: order_info.symbol.clone(),
        price: order_info.price,
        quantity: order_info.quantity,
    };

    let mut broker_guard = broker.lock().unwrap();
    let order_id = broker_guard.place_new_order(new_order);
    exchange.lock().unwrap().match_orders(&mut broker_guard);

    json!({ "order": order_id })    
}

// curl -X GET -i http://localhost:8000/order/status/0
#[get("/order/status/<order_id>")]
fn order_status(broker: BrokerState, order_id: i32) -> JsonValue {
    let has_order = broker.lock().unwrap().has_order(order_id);
    json!({ "opened": has_order })
}

// curl -X GET -i http://localhost:8000/order/trades/0
#[get("/order/trades/<order_id>")]
fn order_trades(exchange: ExchangeState, order_id: i32) -> JsonValue {
    let trades = exchange.lock().unwrap().find_trades_for_order(order_id);
    json!({ "trades": trades })
}

// curl -X POST -i http://localhost:8000/user/033186e8-3fb4-4814-b170-ad989fe98e59/balance/add
#[post("/user/<user_id>/balance/add")]
fn add_balance(broker: BrokerState, user_id: String ) -> JsonValue {
    let mut broker_guard = broker.lock().unwrap();
    if let Some(balance) = broker_guard.get_balance(&user_id) {
        broker_guard.update_balance(&user_id, 100);
        return json!({ "balance": balance + 100 })
    }
    json!({ "error": "User not found" })
}

// curl -X POST -i http://localhost:8000/user/033186e8-3fb4-4814-b170-ad989fe98e59/balance/remove/68
#[post("/user/<user_id>/balance/remove/<amount>")]
fn remove_balance(broker: BrokerState, user_id: String, amount: u32 ) -> JsonValue {
    let mut broker_guard = broker.lock().unwrap();
    if let Some(balance) = broker_guard.get_balance(&user_id) {
        if balance < (amount as i32) {
            return json!({ "error": "Balance too low" })
        }
        broker_guard.update_balance(&user_id, - (amount as i32));
        return json!({ "balance": balance - (amount as i32) })
    }
    json!({ "error": "User not found" })
}

// curl -X GET -i http://localhost:8000/user/033186e8-3fb4-4814-b170-ad989fe98e59/balance/
#[get("/user/<user_id>/balance")]
fn get_balance(broker: BrokerState, user_id: String ) -> JsonValue {
    if let Some(balance) = broker.lock().unwrap().get_balance(&user_id) {
        return json!({ "balance": balance })
    }
    json!({ "balance": 0 })
}

// curl -X GET -i http://localhost:8000/user/033186e8-3fb4-4814-b170-ad989fe98e59/stocks/
#[get("/user/<user_id>/stocks")]
fn get_stock_balance(exchange: ExchangeState, user_id: String ) -> JsonValue {
    if let Some(stocks) = exchange.lock().unwrap().asset_balances.get(&user_id) {
        return json!({ "stocks": stocks })
    }
    json!({ "stocks": {} })
}

// curl -X POST -i http://localhost:8000/user/033186e8-3fb4-4814-b170-ad989fe98e59/stocks/AAPL/add/10
#[post("/user/<user_id>/stocks/<symbol>/add/<amount>")]
fn add_stock_balance(broker: BrokerState, exchange: ExchangeState, user_id: String, symbol: String, amount: u32 ) -> JsonValue {
    if broker.lock().unwrap().get_balance(&user_id).is_none() {
        return json!({ "error": "User not found" });
    }
    let mut exchange_guard = exchange.lock().unwrap();
    if let Some(balance) = exchange_guard.get_asset_balance(&user_id, &symbol) {
        exchange_guard.update_asset_balance(symbol.clone(), &user_id, amount as i32);
        return json!({ "symbol": symbol, "balance": balance + (amount as i32) })
    }
    exchange_guard.update_asset_balance(symbol.clone(), &user_id, amount as i32);
    json!({ "symbol": symbol, "balance": (amount as i32) })
}

// curl -X POST -i http://localhost:8000/user/033186e8-3fb4-4814-b170-ad989fe98e59/stocks/AAPL/remove/1
#[post("/user/<user_id>/stocks/<symbol>/remove/<amount>")]
fn remove_stock_balance(broker: BrokerState, exchange: ExchangeState, user_id: String, symbol: String, amount: u32 ) -> JsonValue {
    if broker.lock().unwrap().get_balance(&user_id).is_none() {
        return json!({ "error": "User not found" });
    }
    let mut exchange_guard = exchange.lock().unwrap();
    if let Some(balance) = exchange_guard.get_asset_balance(&user_id, &symbol) {
        if balance >= amount as i32 {
            exchange_guard.update_asset_balance(symbol.clone(), &user_id, - (amount as i32));
            return json!({ "symbol": symbol, "balance": balance - (amount as i32) })
        } else {
            return json!({ "error": "Not enough tokens" })
        }
    }
    json!({ "error": "Not enough tokens" })
}

// curl -X GET -i http://localhost:8000/user/033186e8-3fb4-4814-b170-ad989fe98e59/stocks/AAPL/
#[get("/user/<user_id>/stocks/<symbol>")]
fn get_stock_balance_per_symbol(broker: BrokerState, exchange: ExchangeState, user_id: String, symbol: String ) -> JsonValue {
    if broker.lock().unwrap().get_balance(&user_id).is_none() {
        return json!({ "error": "User not found" });
    }
    if let Some(balance) = exchange.lock().unwrap().get_asset_balance(&user_id, &symbol) {
        return json!({ "symbol": symbol, "balance": balance })
    }
    json!({ "symbol": symbol, "balance": 0 })
}


fn main() {

    let broker = Broker::new();
    let exchange = Exchange::new();

    rocket::ignite()
        .manage(Arc::new(Mutex::new(broker)))
        .manage(Arc::new(Mutex::new(exchange)))
        .mount("/", routes![
            create_user, user_exists, order_status, place_order, order_trades, 
            add_balance, remove_balance, get_balance, get_stock_balance, 
            add_stock_balance, remove_stock_balance, get_stock_balance_per_symbol
        ]).launch();

}