import requests as r
import bs4
import pandas as pd


"""
list companies
get company ids
"""


def search_link(name: str, form_type='13F-HR') -> str:
    company = name.replace(' ', '+')
    return 'https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&company=' + company + '&type=' + form_type + '&count=100&output=xml'


def get_page(link: str) -> bs4.BeautifulSoup:
    return bs4.BeautifulSoup(r.get(link).text, 'html.parser')


def next_pages(page: bs4.BeautifulSoup) -> list:
    pages = [page]
    next_page = page.find('span', {'id': 'next'})
    while next_page is not None:
        page = get_page(next_page.a['href'])
        pages.append(page)
        next_page = page.find('span', {'id': 'next'})

    return pages


def grab_docs_links(page: bs4.BeautifulSoup) -> list:
    all_links = []
    all_pages = next_pages(page)
    for page in all_pages:
        t = page.find('table', {'class': 'results'})
        links = [l['href'] for l in t.find_all('a')]
        all_links += links

    return all_links


def get_holdings(page: bs4.BeautifulSoup, output='df') -> pd.DataFrame:
    table = page.find('table', {'summary': 'Form 13F-NT Header Information'})

    if output == 'df':
        ret = clean_holdings(pd.read_html(table.prettify())[0])

    return ret


def clean_holdings(df: pd.DataFrame) -> pd.DataFrame:
    df.columns = df.iloc[2]
    df.drop([0, 1, 2], inplace=True)
    return df


def get_CIKs():
    cik = 'https://www.sec.gov/cgi-bin/cik_lookup'
    return cik


def main():
    """
    1. given list of companies, gather all of the CIKs using cik_lookup
    """

    companies = ['Renaissance Technologies', 'Two Sigma Investments', 'Bridgewater Associates',
                    'AQR Capital Management', 'Millennium Management', 'Elliott Management', 'BlackRock', 'Citadel LLC']
    links = list(map(search_link, companies))
    pages = list(map(get_page, links))
    document_links = list(map(grab_docs_links, pages))
    doc_pages = list(map(get_page, document_links))

    dfs = list(map(get_holdings, doc_pages))

    print(dfs)
    return dfs

if __name__ == "__main__":

    dfs = main()
    print(dfs)
