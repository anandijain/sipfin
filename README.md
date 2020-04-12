# sipfin

The point of this repository is to collect financial data for use in risk calculations and understanding international relations.

## Functionality:

1. 13F-HR parser for Edgar SEC data


## Usages:

1. 13F-HR example:

```python

import edgar

dfs = edgar.company_history('Renaissance Technologies')

```

## TODOs

1. 13F txt file parser (SEC does not have html formatting for filings before 2013)


## Goals 

1. Balance sheet parser
2. Income statement parser
3. Cash flow statements 
4. Shareholders' equity statements 


1. https://www.currency-iso.org/dam/downloads/lists/list_one.xml