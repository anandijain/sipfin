import bs4
import pandas as pd
import requests
import glob
import matplotlib.pyplot as plt


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


def commodities():
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

def col_to_txt(df, col:str, fn:str):
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

if __name__ == "__main__":
    write_plots()
