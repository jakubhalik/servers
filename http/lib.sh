host_web_text_that_will_stop_being_hosted_after_being_fetched_once() {
  (
    echo "HTTP/1.1 200 OK"
    echo "Content-Length: $(echo $1|wc -c)"
    echo ""
    echo "$1"
  ) | nc -l -p $2
}
