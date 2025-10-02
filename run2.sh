#!/bin/bash
export WINEDEBUG=-all
export SC2PATH="/home/dax/Games/battlenet/drive_c/Program Files (x86)/StarCraft II/"
cd /home/dax/Repositories/Nikorust
cargo run --features wine_sc2
