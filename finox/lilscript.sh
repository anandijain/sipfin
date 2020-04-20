#!/bin/bash

plot_news() {
    # cargo run
    julia intersect.jl
    python3.6 ../sipfin/utils.py
}

time plot_news
