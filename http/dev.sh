clc() {
  curl localhost:$1
}

clc_test() {
  test $(echo $1) || (echo "Usage: $0 <port to be forever curling>" && exit 1)
  seq 1 99999999999999999999999999999999999999999999999999999999999999999999999|iter dash -c 'timeout 0.01s curl localhost:$1; echo {}' dash $1
}

clct() {clc_test $@}

clp() {
    curl -X POST localhost:$1 -H "Content-Type: application/json" -d $2
}

clu() {
    curl -X PUT localhost:$1 -H "Content-Type: application/json" -d $2
}
