import bs4
import pandas as pd
import requests as r

headers = {
    'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/56.0.2924.87 Safari/537.36',
}


def page(link: str, parser: str = 'html.parser') -> bs4.BeautifulSoup:
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
