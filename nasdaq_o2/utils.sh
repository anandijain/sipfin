#!/bin/bash

# catem() {
#     IFS=', ' 
#     for NAME in $( ls data/ )

#     do 
#         echo $NAME
#         TICKER=read -r -a array <<< "$NAME"
#         sed -i "1 s/^/t, $(\n/" $NAME
# }

./target/release/nasdaq_o2_rt 
