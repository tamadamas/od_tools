default:
  just --list

build:
    cargo build

run:
    cargo run generate-log --sim local/sim.xlsx --result local/sim.txt

run_hour:
    cargo run generate-log --sim local/sim.xlsx --result local/sim.txt --hour 2
