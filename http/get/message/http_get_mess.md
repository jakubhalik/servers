# http_get_mess

> Run a simple multi-threaded super performant (at least 300k RPS on a 2025 ok cpu) http server that just serves one text to get reqs

- U can or not run in a debug mode, pick a port and message

`http_get_mess --debug 1024 message`

- can run with quite huge messages if u're into that

`http_get_mess 8080 $(head -c 10000 /dev/urandom | tr -dc 'A-Za-z')`
