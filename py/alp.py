import requests as r
import json
from config import *
import alpaca_trade_api as tradeapi


api = tradeapi.REST()
def test_get():
    barset = api.get_barset('AAPL', 'day', limit=5)
    aapl_bars = barset['AAPL']
    return aapl_bars

ACC_URL = f'{BASE_URL}/v2/account'
ORD_URL = f'{BASE_URL}/v2/orders'

HEADERS = {'APCA-API-KEY-ID' : API_KEY, 'APCA-API-SECRET-KEY': SECRET_KEY}

def get_acc():
    req = r.get(ACC_URL, headers=HEADERS)
    return json.loads(req.content)


if __name__ == "__main__":
    # /ret = get_acc()
    # print(f'ret: {ret}')
    aapl = test_get() 
    print(f'aapl: {aapl}')
