#!/bin/bash
init_dirs() {
	mkdir -p data/yf data/sec/10q data/sec/13f data/nasdaq/realtime-trades ref_data/sec
}

# run as root in home
init_main() {
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	source $HOME/.cargo/env
	wget "https://julialang-s3.julialang.org/bin/linux/x64/1.4/julia-1.4.2-linux-x86_64.tar.gz" 
	tar -xvf "julia-1.4.2-linux-x86_64.tar.gz"
	git clone "https://github.com/anandijain/sipfin.git"
}

init_conf() {
	apt-get install build-essential curl wget libssl-dev pkg-config 
	apt-get install neovim fish exa 
	chsh -s /usr/bin/fish
	cd ~
	git clone "https://github.com/anandijain/lilconfig.git" 
	mkdir -p ~/.config/nvim/autoload	
	wget -O ~/.config/nvim/autoload/plug.vim "https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim" 
	mv lilconfig/init.vim ~/.config/nvim/
	mv config.fish ~/.config/fish
	source ~/.config/fish/config.fish
}

options() {
    curl "ftp://ftp.nasdaqtrader.com/symboldirectory/options.txt" -o "./ref_data/options.txt"
}

get_nasdaq_listings() {

    curl "ftp://ftp.nasdaqtrader.com/symboldirectory/nasdaqlisted.txt" -o "./ref_data/nasdaqlisted.txt"
	julia run.jl
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




