use crate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_orders() {

        let mut exchange = Exchange::new();
        let mut broker = Broker::new();


        broker.add_user("user1".to_owned());
        broker.add_user("user2".to_owned());

        broker.update_balance("user1", 1000);
        broker.update_balance("user2", 0);

        exchange.update_asset_balance("AAPL".into(), &"user2".into(), 10);


        let buy_order = Order {
            id: 0,
            user_id: "user1".into(),
            order_type: OrderType::Buy,
            symbol: "AAPL".to_owned(),
            price: 100,
            quantity: 10,
        };
        broker.place_new_order(buy_order);

        let sell_order = Order {
            id: 0,
            user_id: "user2".into(),
            order_type: OrderType::Sell,
            symbol: "AAPL".to_owned(),
            price: 100,
            quantity: 10,
        };
        broker.place_new_order(sell_order);

        exchange.match_orders(&mut broker);

        assert_eq!(exchange.trades.len(), 1);
        assert_eq!(exchange.trades[0].symbol, "AAPL");
        assert_eq!(exchange.trades[0].price, 100);
        assert_eq!(exchange.trades[0].quantity, 10);

        assert_eq!(broker.open_orders.len(), 0);

        assert_eq!(exchange.get_asset_balance(&"user1".into(), "AAPL"), Some(10));
        assert_eq!(exchange.get_asset_balance(&"user2".into(), "AAPL"), Some(0));

        assert_eq!(broker.get_balance("user1"), Some(0));
        assert_eq!(broker.get_balance("user2"), Some(1000));

    }

}