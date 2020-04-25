
# Docs

## Currently have:
### Prices
  * yf
  * bloomberg
  * steam trades

### News
  * nyt archive and feed
  * seeking alpha trending
  * wsj videos  
  * JPX news 

## Todo:
  * remove duplicate code with generic typed functions
  * fred data
  * use a db, noria ideally
  * define schema
  * add logging 
  * run on cloud 
  * test stability 
  * integrate with alpaca

## api todos:
  * Guardian
  * FT


### something else
* using https://xueqiu.com/, seems like its providing public realtime data, 

* turns out [yahoo finance data](https://help.yahoo.com/kb/exchanges-data-providers-yahoo-finance-sln2310.html) is mostly 15 mins delayed, which is obviously unacceptable.

* here are two of the nice endpoints they have [last twenty trade prices](https://stock.xueqiu.com/v5/stock/history/trade.json?symbol=FB&count=20) and [realtime price](https://stock.xueqiu.com/v5/stock/realtime/quotec.json?symbol=FB).
* 
* next data to get is interest and reserve rates, and other fred data like cpi and gdp, which the govt has an api for. 

* database https://github.com/mit-pdos/noria running. this is a vid the creator gave at two sigma on noria https://youtu.be/s19G6n0UjsM?t=977.

