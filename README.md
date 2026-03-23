### commands to start hosting simple http servers on <port>, parallel async with actix-web -> tested proc 300k get reqs/s, helpers for curl testing

individual commands to run will be usually functions, not files, since I would otherwise have crazy amount of small files here - not a js project lol - that would be stupid - but that is the true only when commands will be just pipelines of different commands, not when written programs to be compiled

dev.sh will be full of funcs that are only helpful for development on the lib.sh and other sources of commands

# http_get_mess

> Run a simple multi-threaded super performant (at least 300k RPS on a 2025 ok cpu) http server that just serves one text to get reqs

- U can or not run in a debug mode, pick a port and message

`http_get_mess --debug 1024 message`

- can run with quite huge messages if u're into that

`http_get_mess 8080 $(head -c 10000 /dev/urandom | tr -dc 'A-Za-z')`

- defaultly will run on localhost
- run on 0.0.0.0 to be publicly available as long as firewall allows access to that port
`http_get_mess --public`
`http_get_mess --public helo`
`http_get_mess -p helo -d 1024`


lib.sh or dev.sh must be sourced if u wanna run some of below


> This one is a little fun:
`host_web_text_that_will_stop_being_hosted_after_being_fetched_once "hello helloo :)" 1024`

> send a fat stack of curls single-threaded:
`clct 1024`


It is real interesting to compare just how much brutally faster is http_get_mess then get_server or classic flask server or python3 -m http.server

Curl testing is not enough for actix-web, I test with `oha`

`oha -n 300k http://localhost:1024`

on my 16 core cpu oha actually fetches 300k reqs in a second for http_get_mess, where other lang http servers were able to max out at about 20k/s, actix-web limit should be 600k/s, so idk if the bottleneck of 300k/s is on the side of my cpu or oha

