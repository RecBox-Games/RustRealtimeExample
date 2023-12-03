#!/bin/bash

# kill function
function kill_process_by_name() {
    pattern=$1
    pids=$(ps aux | grep "$pattern" | grep -v grep | awk '{print $2}')
    for pid in $pids; do
        kill -9 $pid
        echo "Killed process $pid"
    done
}


# kill any previous instances of the game running
kill_process_by_name rust_cards_example

# don't continue if there's build errors
set -e
cargo check
set +e

# start the controlpad server and web server
cd controlpad_server
./start.sh &
cd ..

# build and run the game (getting ControlpadServer dependency from private repo)
export CARGO_NET_GIT_FETCH_WITH_CLI=true
$(
    cargo run 2>/dev/null
    cd controlpad_server
    ./start.sh -x
) &

# print out qr code to connect phones to web server at this computer's IP
ifconfig | grep 'inet ' | tail -1 | sed 's/.*inet \([^ ]*\).*/http:\/\/\1:3000/' | qrencode -t utf8 -m 2

