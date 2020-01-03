import edgar as ed


def links(companies: list) -> list:

    return list(map(ed.search_link, companies))


# example link : https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&company=Renaissance+Technologies&type=13F-HR&output=xml
# https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&company=Renaissance+Technologies&type=13F-HR&count=100&output=xml
# https://www.sec.gov/cgi-bin/browse-edgar?
# action=getcompany
# &company=Renaissance+Technologies
# &type=13F-HR
# &output=xml


if __name__ == "__main__":
    companies = [
        "Renaissance Technologies",
        "Two Sigma Investments",
        "Bridgewater Associates",
        "AQR Capital Management",
        "Millennium Management",
        "Elliott Management",
        "BlackRock",
        "Citadel LLC",
    ]
    ls = links(companies)
    print(ls)
