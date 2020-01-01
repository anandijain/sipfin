from selenium import webdriver

import csv
import json
import edgar
"""
# todo benchmark selenium driver to bs4 cik grab vs just selenium 

"""

CIK_URL = 'https://www.sec.gov/cgi-bin/cik_lookup'


def base_driver(screen=None):
    """
    screen is either None, 'full', or 'min'
    """
    chrome_options = webdriver.ChromeOptions()
    chrome_options.add_argument("--incognito")
    chrome_options.add_argument("--disable-dev-shm-usage")

    driver = webdriver.Chrome(chrome_options=chrome_options)

    if not screen:
        pass
    elif screen == "full":
        driver.maximize_window()
    elif screen == "min":
        driver.minimize_window()

    return driver



def get_CIKs(driver, search_term:str)-> list:
    driver.get(CIK_URL)
    input_fields = driver.find_element_by_tag_name("input")
    input_fields[0].send_keys(search_term)
    input_fields[1].send_keys("\n")

    # to bs4 ?  p.find_all('a')

    links = driver.find_elements_by_tag_name("a")
    ciks = [l.text for l in links]
    return ciks


def main():
    """
    1. open browser to cik_lookup url
    2. create dictionary for each search term where value is list of CIKs
    3. write to csv 
        csv: each column is search term row elements are 
    """
    companies = ['Renaissance Technologies', 'Two Sigma Investments', 'Bridgewater Associates',
                 'AQR Capital Management', 'Millennium Management', 'Elliott Management', 'BlackRock', 'Citadel LLC']

    driver = base_driver()
    # need to async/map by opening new tab or something 
    ret = {c : get_CIKs(driver, c) for c in companies}
    return ret


if __name__ == "__main__":
    ciks = main()
    print(ciks)
