host_web_text_that_will_stop_being_hosted_after_being_fetched_once() {
  test $(echo $2) || (echo "Usage: $0 <plaintext to put on the website> <port to host on>" && exit 1)
  (
    echo "HTTP/1.1 200 OK"
    echo "Content-Length: $(echo $1|wc -c)"
    echo ""
    echo "$1"
  ) | nc -l -p $2
}

get_server() {
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

post_server() {

}

post_server_test() {
  test $(echo $1) || (echo "Usage: $0 <port to host on>" && exit 1)
    port="$1"
    while true; do
      req=$(nc -l -p "$port" | tee /tmp/req$$)
      body=$(printf "%s" "$req" | sed -n '/^$/,/^$/p' | tail -n +2)
      
      content_len=$(printf "%s" "$req" | grep -i "^Content-Length:" | awk "{print \$2}" | tr -d "\r")
      [ -n "$content_len" ] && body=$(printf "%s" "$body" | head -c "$content_len")
      
      body=$(printf "%s" "$body" | tr -d "\r\n" | xargs)
      
      case "$body" in
        "hi") response="helloo!!" ;;
        "how r u?"|"how are you?") response="well, u girlie?" ;;
        *) response="go away" ;;
      esac
      
      {
        printf "HTTP/1.1 200 OK\r\n"
        printf "Content-Type: text/plain\r\n"
        printf "Content-Length: %s\r\n" "$(printf "%s" "$response" | wc -c)"
        printf "\r\n"
        printf "%s" "$response"
      }
      
      rm -f /tmp/req$$
    done
}
