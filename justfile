default:
  just --list

build: 
    go build -o build/od_sim ./...

run: build
    build/od_sim generate_log -sim local/sim.xlsm -result local/sim.txt

test:
    go test ./...
