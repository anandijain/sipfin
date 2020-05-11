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


looperz() {
	i=0
	while :
	do
		echo $i
		((i++))	
		./target/release/finox
		sleep 600 
	done
}

letsgetit() {

	./target/release/finox
	julia ../jl/smol.jl

}

letsgetit

# looperz
