import requests as r
import bs4
import pandas as pd

EXAMPLE_TABLE = "https://www.sec.gov/Archives/edgar/data/1037389/000103738919000272/xslForm13F_X01/renaissance13Fq32019_holding.xml"

ROOT = 'https://www.sec.gov'


def search_link(name: str, form_type:str='13F-HR', verbose:bool=False) -> str:
    """
    constructs a link to the filings page for a company name and form type
    """
    company = name.replace(' ', '+')
    ret = ROOT + '/cgi-bin/browse-edgar?action=getcompany' + '&company=' + company + '&type=' + form_type + '&count=100'
    if verbose: 
        print(f'Grabbing: {ret}')
    return ret


def get_page(link: str, parser:str='html.parser') -> bs4.BeautifulSoup:
    return bs4.BeautifulSoup(r.get(link).text, 'html.parser')


def next_pages(page: bs4.BeautifulSoup) -> list:
    """
    TODO: not being used as most companies do not have >100 13F filings, however, this needs to get fixed!

    """
    pages = [page]
    next_page = page.find('span', {'id': 'next'})
    while next_page is not None:
        page = get_page(next_page.a['href'])
        pages.append(page)
        next_page = page.find('span', {'id': 'next'})

    return pages


def grab_docs_links(page: bs4.BeautifulSoup, output:str='dict'):
    """
    given a bs4 page, find all of the links to filings and return either a list or dictionary.

    if output is dict, the keys are the filing dates and the values are links to the html data

    """

    if output == 'dict':
        docs = {}
    elif output == 'list':
        docs = []

    all_links = page.find_all('a', {'id': 'documentsbutton'})
    # print(f'getting document links for {company_full_name} at {}')

    for l in all_links:
        cur_page = get_page(ROOT + l['href'])
        date = cur_page.find('div', {'class' : 'info'}).text
        cur_table = cur_page.find('table', {'class': 'tableFile'})
        links = cur_table.find_all('a')

        if len(links) < 4:
            continue
        
        html_link = ROOT + links[2]['href']

        if output == 'dict':
            docs[date] = html_link
        elif output == 'list':
            docs.append(html_link)
    return docs


def get_holding(link: str)-> pd.DataFrame:
    """
    link is html formatted 13F-HR form link as in EXAMPLE_TABLE at start of this file
    
    """
    p = get_page(link)
    df = get_holding_from_page(p)
    return df


def get_holding_from_page(page: bs4.BeautifulSoup, output='df') -> pd.DataFrame:
    """
    given the bs4 page, uses pandas to parse the holdings table

    """
    table = page.find('table', {'summary': 'Form 13F-NT Header Information'})

    if output == 'df':
        ret = clean_holding(pd.read_html(table.prettify())[0])

    return ret


def clean_holding(df: pd.DataFrame, convert:bool=False) -> pd.DataFrame:
    """
    parses an html holdings page 

    columns to convert: ['(x$1000)', 'PRN AMT', 'MANAGER', 'SOLE', 'SHARED', 'NONE']

    """
    df.columns = df.iloc[2]
    df.drop([0, 1, 2], inplace=True)

    if convert:
        to_convert = ['(x$1000)', 'PRN AMT', 'MANAGER', 'SOLE', 'SHARED', 'NONE']
        df[to_convert] = df[to_convert].apply(pd.to_numeric, errors='ignore')

    return df


def company_history(name:str, form_type:str='13F-HR', verbose:bool=False)-> dict:
    """
    date : df
    """
    history = {}
    link = search_link(name, form_type=form_type)
    page = get_page(link)
    doc_links = grab_docs_links(page, output='dict')
    for date, doc_link in doc_links.items():
        print(f'date: {date}')
        df = get_holding(doc_link)
        history[date] = df
    return history


def main():
    """
    1. given list of companies, gather all of the CIKs using cik_lookup (might need to use selenium :(  ))
    2. for each CIK, gather links to all 13F-HR forms html formatting, txt link if html nonexistent
    3. for each html page, create dataframe
    4. dictionary of companies:
        "search_term" : [
            cik : [
                date : dataframe
            ]
        ]

    """

    companies = ['Renaissance Technologies']
    # companies = ['Renaissance Technologies', 'Two Sigma Investments', 'Bridgewater Associates',
    #              'AQR Capital Management', 'Millennium Management', 'Elliott Management', 'BlackRock', 'Citadel LLC']
    data = {name: company_history(name) for name in companies}
    return data


if __name__ == "__main__":

    data = main()
    print(data)
