import plotly.graph_objects as go
import bs4
import pandas as pd
import requests as r
import glob
import datetime
import matplotlib.pyplot as plt

from functools import reduce
from transformers import pipeline
import pandas as pd

headers = {
    'User-Agent': 'My User Agent 1.0',
}

def page(link: str, parser:str = 'html.parser') -> bs4.BeautifulSoup:
    """

    """
    p = bs4.BeautifulSoup(r.get(link, headers=headers).text, parser)
    return p


def get_dfs(link: str, fn=None) -> list:
    """
    // fn only writes first, (most common)
    """
    dfs = [pd.read_html(p.prettify()) for p in page(link).find_all('table')]
    if fn:
        dfs[0][0].to_csv(fn, index=False)
    return dfs


def sec_cik_listings(cik:int) -> pd.DataFrame:
    return get_dfs(f'https://www.sec.gov/Archives/edgar/data/{cik}/')


def sec_xml(cik:int, form='13'):
    link = f"https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK={cik}&type={form}%25&output=atom"
    return [href.text for href in page(link, 'lxml').find_all('filing-href')]


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
    df[[col]].to_csv(fn, index=False, sep='\n', header=False)


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
    pat = f"{root}data/{tick}_7d*.csv"
    path = glob.glob(pat)
    print(path)
    df = pd.read_csv(path)

    df2 = pd.read_csv(root + "ref_data/sa.csv")
    df2 = add_sentiments(df2, "title")
    union_rows = df2[df2.slug == tick.lower()]
    print(tick, union_rows)

    fig = go.Figure(data=[go.Candlestick(x=df['t'],
                                         open=df[f'o_{tick}:US'], high=df[f'h_{tick}:US'],
                                         low=df[f'o_{tick}:US'], close=df[f'c_{tick}:US'])
                          ])
    shapes = []
    annotes = []
    i = 1
    for index, row in union_rows.iterrows():
        shapes.append(dict(
            x0=row['publish_on'], x1=row['publish_on'], y0=0, y1=1, xref='x', yref='paper',
            line_width=2))
        annotes.append(dict(
            x=row['publish_on'], y=i*0.1, xref='x', yref='paper',
            showarrow=False, xanchor='left', text=row['title'])
        )
        i += 1
    row = union_rows.iloc[0]
    fig.update_layout(
        title=f"{tick}: {row.title}, {row['sentiment_label']}, {row['sentiment_score']}",
        yaxis_title=f'{tick} candlestick',
        shapes=shapes,
        annotations=annotes,
    )

    fig.show()


def add_sentiments(df: pd.DataFrame, col: str, label_col="sentiment_label", score_col="sentiment_score") -> pd.DataFrame:
    nlp = pipeline('sentiment-analysis')
    sentiments = [nlp(text)[0] for text in df[col]]

    labels = [s['label'] for s in sentiments]
    labels = pd.Series(labels, name=label_col)

    scores = [s['score'] for s in sentiments]
    scores = pd.Series(scores, name=score_col)

    return pd.concat([df, labels, scores], axis=1)


if __name__ == "__main__":
    max_plots = 5

    f = open("../finox/ref_data/intersect_sa_yf.txt", "r")
    intersects = f.read().splitlines()

    for i, elt in enumerate(intersects):
        # try: 
        candle_plot(elt)
        # except:
        #     continue
        # if i >= max_plots:
        #     break
