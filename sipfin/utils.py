import bs4
import pandas as pd
import requests

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


def col_to_txt(df, col:str, fn:str):
    l = df[[col]].to_csv(index=False, sep='\n')
