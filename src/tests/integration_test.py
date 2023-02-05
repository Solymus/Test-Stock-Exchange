import requests
import json

def create_user():
    url = "http://localhost:8000/user/"
    headers = {
        'Content-Type': 'application/json'
    }
    response = requests.post(url, headers=headers)
    if response.status_code == 200:
        return json.loads(response.text)["user_id"]
    return None

def user_exists(user_id):
    url = f"http://localhost:8000/user/{user_id}/exists/"
    headers = {
        'Content-Type': 'application/json'
    }
    response = requests.get(url, headers=headers)
    if response.status_code == 200:
        return json.loads(response.text)["exists"]
    return None

def place_order(user_id, order_type, symbol, price, quantity):
    url = "http://localhost:8000/order/place/"
    headers = {
        'Content-Type': 'application/json'
    }
    payload = {
        "user_id": user_id,
        "order_type": order_type,
        "symbol": symbol,
        "price": price,
        "quantity": quantity
    }
    response = requests.post(url, headers=headers, data=json.dumps(payload))
    if response.status_code == 200:
        return json.loads(response.text)["order"]
    return None

def order_status(order_id):
    url = f"http://localhost:8000/order/status/{order_id}"
    headers = {
        'Content-Type': 'application/json'
    }
    response = requests.get(url, headers=headers)
    if response.status_code == 200:
        return json.loads(response.text)["opened"]
    return None

def order_trades(order_id):
    url = f"http://localhost:8000/order/trades/{order_id}"
    headers = {
        'Content-Type': 'application/json'
    }
    response = requests.get(url, headers=headers)
    if response.status_code == 200:
        return json.loads(response.text)["trades"]
    return None

def add_balance(user_id):
    url = f"http://localhost:8000/user/{user_id}/balance/add"
    headers = {
        'Content-Type': 'application/json'
    }
    response = requests.post(url, headers=headers)
    if response.status_code == 200:
        return json.loads(response.text)["balance"]
    return None

def remove_balance(user_id, amount):
    url = f"http://localhost:8000/user/{user_id}/balance/remove/{amount}"
    headers = {
        'Content-Type': 'application/json'
    }
    response = requests.post(url, headers=headers)
    if response.status_code == 200:
        return json.loads(response.text)["balance"]
    return None


def get_balance(user_id):
    url = f"http://localhost:8000/user/{user_id}/balance"
    headers = {
        'Content-Type': 'application/json'
    }
    response = requests.get(url, headers=headers)
    if response.status_code == 200:
        return json.loads(response.text)["balance"]
    return None

def get_stock_balance(user_id):
    url = f"http://localhost:8000/user/{user_id}/stocks/"
    response = requests.get(url)
    if response.status_code == 200:
        return json.loads(response.text)["stocks"]
    return None

def add_stock_balance(user_id, symbol, amount):
    url = f"http://localhost:8000/user/{user_id}/stocks/{symbol}/add/{amount}"
    response = requests.post(url)
    if response.status_code == 200:
        return json.loads(response.text)["balance"]
    return None

def remove_stock_balance(user_id, symbol, amount):
    url = f"http://localhost:8000/user/{user_id}/stocks/{symbol}/remove/{amount}"
    response = requests.post(url)
    if response.status_code == 200:
        return json.loads(response.text)["balance"]
    return None


def get_stock_balance_per_symbol(user_id, symbol):
    url = f"http://localhost:8000/user/{user_id}/stocks/{symbol}"
    response = requests.get(url)
    if response.status_code == 200:
        return json.loads(response.text)["balance"]
    return None

def test_place_orders_and_check_balances():
    # Create 2 users
    user1 = create_user()
    user2 = create_user()

    # Add their initial balances
    for i in range(5):
        add_balance(user1)
    for i in range(10):
        add_balance(user2)

    add_stock_balance(user2, "AAPL", 20)

    # Place buy and sell orders
    buy_order = place_order(user1, "Buy", "AAPL", 10, 10)
    sell_order = place_order(user2, "Sell", "AAPL", 9, 10)

    # Confirm trades were done
    trades = order_trades(buy_order)
    assert len(trades) == 1
    trade = trades[0]
    assert trade["price"] == 10
    assert trade["quantity"] == 10

    # Confirm balances are correct
    assert get_balance(user1) == 400
    assert get_balance(user2) == 1100

    assert get_stock_balance_per_symbol(user2, "AAPL") == 10
    assert get_stock_balance_per_symbol(user1, "AAPL") == 10

    print("Test passed!")

def main():
    test_place_orders_and_check_balances()

if __name__ == "__main__":
    main()