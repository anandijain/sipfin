#!/bin/bash

options() {
    curl "ftp://ftp.nasdaqtrader.com/symboldirectory/options.txt" -o "./ref_data/options.txt"
}

