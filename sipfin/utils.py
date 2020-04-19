import plotly.graph_objects as go
import bs4
import pandas as pd
import requests
import glob
import datetime
import matplotlib.pyplot as plt
import sentiment

from functools import reduce


def page(link: str) -> bs4.BeautifulSoup:
    """

    """
    p = bs4.BeautifulSoup(requests.get(link).text, 'html.parser')
    return p


def get_dfs(link: str) -> list:
    """

    """
    dfs = [pd.read_html(p.prettify()) for p in page(link).find_all('table')]
    return dfs


def sp500_df() -> pd.DataFrame:
    return get_dfs('https://en.wikipedia.org/wiki/List_of_S%26P_500_companies')[0][0]


def bloomberg_commodities():
    root = 'https://www.bloomberg.com/markets/commodities'
    p = page(root)
    group = p.find('ul', {'class': 'group'})
    urls = [a['href'] for a in group.find_all('a')]
    dfs = []
    for url in urls:
        dfs.append(get_dfs(url))
    return dfs


def write_plots():
    dfs = getem()
    for df in dfs:
        df.plot(x='date_time', y=df.columns[1])
        print(df.columns[1])
        plt.savefig(f'{df.columns[1]}_intraday.png', dpi=300)


def convert_dts(dfs, colname):
    for df in dfs:
        df[colname] = pd.to_datetime(df[colname])
    return dfs


def col_to_txt(df, col: str, fn: str):
    l = df[[col]].to_csv(index=False, sep='\n')


def getem():
    fns = glob.glob(
        '/home/sippycups/programming/repos/sipfin/finox/data/**stock_intraday*.csv')
    print(fns)
    dfs = [pd.read_csv(fn) for fn in fns]
    dfs = convert_dts(dfs, 'date_time')
    return dfs


def merge_em():
    dfs = getem()
    df_merged = reduce(lambda left, right: pd.merge(left, right, on=['date_time'],
                                                    how='outer'), dfs)
    return df_merged


def yf_com() -> pd.DataFrame:
    root = 'https://finance.yahoo.com/commodities'
    df = get_dfs(root)[0][0]
    return df


def candle_plot(tick: str):
    dt = datetime.date.today()
    root = "~/sipfin/finox/"
    path = f"{root}data/{tick}:US_2020_4_18.csv"
    print(path)
    df = pd.read_csv(path)
    df2 = pd.read_csv(root + "sa.csv")
    df2 = sentiment.add_sentiments(df2, "title")
    df2 = df2[df2.slug == tick.lower()]

    fig = go.Figure(data=[go.Candlestick(x=df['t'],
                                         open=df[f'o_{tick}:US'], high=df[f'h_{tick}:US'],
                                         low=df[f'o_{tick}:US'], close=df[f'c_{tick}:US'])
                          ])
    for index, row in df2.iterrows():
        print(row)
        fig.update_layout(
            title=f"{tick}: {row['title']}, {row['sentiment_label']}, {row['sentiment_score']}",
            yaxis_title=f'{tick} candlestick',
            shapes=[dict(
                x0=row['publish_on'], x1=row['publish_on'], y0=0, y1=1, xref='x', yref='paper',
                line_width=2)],
            annotations=[dict(
                x=row['publish_on'], y=0.05, xref='x', yref='paper',
                showarrow=False, xanchor='left', text=row['title'])],
        )

    fig.show()


if __name__ == "__main__":
    # df = merge_em()
    # df.to_csv('intraday_merged.csv')
    # df = yf_com()
    # print(df.Symbol.to_list())
    intersects = [
        "atvi",
        "amzn",
        "aos",
        "ba",
        "cah",
        "cop",
        "dvn",
        "dfs",
        "etr",
        "eog",
        "gild",
        "ibm",
        "iqv",
        "ksu",
        "key",
        "kr",
        "ms",
        "nflx",
        "nke",
        "rtx",
        "o",
        "rf",
        "stt",
        "syy",
        "vlo",
        "wba",
        "wfc"
    ]

    # todo covariance of 5-10 mins out with sentiment score
    for elt in intersects:
        candle_plot(elt.upper())
