#!/bin/bash
init() {
	mkdir -p data/yf data/sec data/nasdaq/realtime-trades ref_data/sec
}
options() {
    curl "ftp://ftp.nasdaqtrader.com/symboldirectory/options.txt" -o "./ref_data/options.txt"
}

sec_idxs() {
for YEAR in {1993..2020}
do 
	for QTR in {1..4}
	do 
		URL="https://www.sec.gov/Archives/edgar/full-index/$YEAR/QTR$QTR/master.idx" 
		OPATH="./ref_data/sec/master_$YEAR$QTR.idx"
		echo "$URL, to $OPATH"
		curl $URL -o $OPATH
	done
done


for f in ./ref_data/sec/*.idx
do 
	sed -i -e 1,9d $f
	sed -i -e 2d $f
done
}

