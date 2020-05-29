import utils

ROOT = 'http://services.runescape.com/m=itemdb_oldschool/'

def item_links(link: str) -> list:
    """

    """
    p = utils.page(link)
    t = p.find('tbody')
    rows = t.find_all('tr')
    links = []
    # a_tags = map()
    for row in rows:
        link = row.find('a')
        links.append(link['href'])
    return links

if __name__ == "__main__":
    links = item_links(
        'http://services.runescape.com/m=itemdb_oldschool/top100?list=2&scale=3')
    print(links)
