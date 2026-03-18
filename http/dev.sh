clc() {
  curl localhost:$1
}

c_test() {
  test $(echo $1) || (echo "Usage: $0 <port to be forever curling>" && exit 1)
  seq 1 99999999999999999999999999999999999999999999999999999999999999999999999|iter dash -c 'timeout 0.1s curl $1; echo {}' dash $1
}
clc_test() {
  c_test localhost:$1
}

clct() {clc_test $@}

clp() {
  curl -X POST localhost:$1 -H "Content-Type: application/json" -d $2
}

clu() {
  curl -X PUT localhost:$1 -H "Content-Type: application/json" -d $2
}

tldrify_and_install() {
  randomzus=$RANDOM$RANDOM$RANDOM
  echo '
    tldrify() {
      cp $1.md ~/.cache/tldr/pages/common/$1.md
      cp $1.md ~/.cache/tlrc/pages.en/common/$1.md
    }
    tldrify $1
    read -sp "Enter sudo pass: " sudo_pass 
    cargo build --release
    echo "$sudo_pass" | sudo -S install target/release/$1 /usr/bin/
  ' > /tmp/$randomzus
  chmod +x /tmp/$randomzus
  /tmp/$randomzus $1
  rm /tmp/$randomzus
}
