import bs4
import pandas as pd
import requests

def page(link: str) -> bs4.BeautifulSoup:
    """

    """
    p = bs4.BeautifulSoup(requests.get(link).text, 'html.parser')
    return p


def df_from_link(link: str) -> pd.DataFrame:
    """

    """
    df = pd.read_html(page(link).find('table').prettify())
    return df
