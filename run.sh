#!/bin/bash
export WINEDEBUG=-all
export SC2PATH="/home/dax/Bob/Battle.net/StarcraftII/"
cd /home/dax/Bob/Repositories/Nikorust
cargo run --features wine_sc2
