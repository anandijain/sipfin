#!/bin/bash

plot_news() {
    # cargo run
    julia ../jl/utils.jl
    python3.6 ../sipfin/utils.py
}
# time plot_news


options() {
    curl "ftp://ftp.nasdaqtrader.com/symboldirectory/options.txt" -o "./ref_data/options.txt"
}