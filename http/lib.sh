host_web_text_that_will_stop_being_hosted_after_being_fetched_once() {
  test $(echo $2) || (echo "Usage: $0 <plaintext to put on the website> <port to host on>" && exit 1)
  (
    echo "HTTP/1.1 200 OK"
    echo "Content-Length: $(echo $1|wc -c)"
    echo ""
    echo "$1"
  ) | nc -l -p $2
}

get_server () {
  test $(echo $2) || (echo "Usage: $0 <plaintext to put on the website> <port to host on>" && exit 1)
  dash -c '
    msg="$1"
    port="$2"
    while true; do
      {
        printf "HTTP/1.1 200 OK\r\n"
        printf "Content-Type: text/plain\r\n"
        printf "Content-Length: %s\r\n" "$(printf "%s" "$msg" | wc -c)"
        printf "\r\n"
        printf "%s" "$msg"
      } | nc -l -p "$port"
    done
  ' dash "$1" "$2"
}

