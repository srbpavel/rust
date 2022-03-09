# handler_video

<b>actix lesson</b>

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustup update
 stable-x86_64-unknown-linux-gnu unchanged - rustc 1.59.0 (9d1b2106e 2022-02-23)

$ echo "export PATH=$PATH\:$HOME/.cargo/env" >> ~/.bashrc
```

*CONFIG and RUN*
```
$ cat src/handler_video_config.toml
name = 'HANDLER_VIDEO'
host = 'spongebob'
server = "localhost"
port = 8081
workers = -1 # 2 # 64 # -1 as default cpu number
log_format = "\"%r\" %s %b \"%{User-Agent}i\" %D"

[flag]
debug_config = false

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/handler_video`

EXIT: Problem parsing cmd arguments
REASON >>> we want exactly one argument
 example:
  $ cargo run /home/conan/soft/rust/handler_video/src/handler_video_config.toml
  $ /home/conan/soft/rust/handler_video/target/debug/handler_video /home/conan/soft/rust/handler_video/src/handler_video_config.toml

$ cargo run /home/conan/soft/rust/handler_video/src/handler_video_config.toml
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/handler_video /home/conan/soft/rust/handler_video/src/handler_video_config.toml`
[2022-03-09T10:29:26Z INFO  handler_video::handler] start -> HANDLER_VIDEO at spongebob / localhost
[2022-03-09T10:29:26Z INFO  actix_server::builder] Starting 2 workers
[2022-03-09T10:29:26Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
```

*EMPTY START*
```
$ curl http://localhost:8081/video/all 2>/dev/null | jq
{
  "result": null,
  "status": "none videos found"
}

$ curl http://localhost:8081/video/list/chunk_tester 2>/dev/null | jq
{
  "result": null,
  "status": "group not found"
}

$ curl "http://127.0.0.1:8081/video/detail/verne_piped" 2>/dev/null | jq
{
  "result": null,
  "status": "video_id not found"
}

$ curl -X DELETE "http://127.0.0.1:8081/video/delete/verne_piped" 2>/dev/null | jq
{
  "status": "video_id not found"
}

$ curl "http://127.0.0.1:8081/video/play/verne_piped" 2>/dev/null | tail
{"status": "player binary_id not found"}
```

*IMPORT*
```
#no header video_id
$ cat /home/conan/video/youtube/lines_twenty_thousand_leagues_under_the_sea_by_jules_verne.txt | curl -X PUT -H "Transfer-Encoding: chunked" -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/upload" -F "ts=@-;type=text/plain" -H "video_id: verne_piped" -H "###group: chunk_tester" --no-buffer --limit-rate 10K
{"result":null,"status":"header 'group' not provided"}

#no header group
$ cat /home/conan/video/youtube/lines_twenty_thousand_leagues_under_the_sea_by_jules_verne.txt | curl -X PUT -H "Transfer-Encoding: chunked" -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/upload" -F "ts=@-;type=text/plain" -H "###video_id: verne_piped" -H "group: chunk_tester" --no-buffer --limit-rate 10K
{"result":null,"status":"header 'video_id' not provided"}

#missing form filename
$ cat /home/conan/video/youtube/lines_twenty_thousand_leagues_under_the_sea_by_jules_verne.txt | curl -X PUT -H "Transfer-Encoding: chunked" -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/upload" -F "ts=;type=text/plain" -H "video_id: verne_piped" -H "group: chunk_tester" --no-buffer --limit-rate 10K
{"result":null,"status":"form 'filename' not provided"}

#missing form name
$ cat /home/conan/video/youtube/lines_twenty_thousand_leagues_under_the_sea_by_jules_verne.txt | curl -v -X PUT -H "Transfer-Encoding: chunked" -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/upload" -F "=@-;type=text/plain" -H "video_id: verne_piped" -H "group: chunk_tester" --no-buffer --limit-rate 10K
No Content-Disposition `form-data` header* Closing connection 0

$ cat /home/conan/video/youtube/lines_twenty_thousand_leagues_under_the_sea_by_jules_verne.txt | curl -v -X PUT -H "Transfer-Encoding: chunked" -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/upload" -F "ts=@-;type=text/plain" -H "video_id: verne_piped" -H "group: chunk_tester" --no-buffer --limit-rate 10K

#first chunk
$ curl "http://127.0.0.1:8081/video/play/verne_piped" 2>/dev/null | tail
  1181
  1182  They heaved the log a second time.
  1183
  1184  "Well?" asked the captain of the man at the wheel.
  1185
  1186  "Nineteen miles and three-tenths, sir."
  1187
  1188  "Clap on more steam."
  1189

$ curl "http://127.0.0.1:8081/video/detail/verne_piped" 2>/dev/null| jq
{
  "result": {
    "id": "verne_piped",
    "group": "chunk_tester",
    "name": "ts"
  },
  "status": "video_id found"
}

$ curl http://127.0.0.1:8081/video/all 2>/dev/null| jq
{
  "result": {
    "verne_piped": {
      "id": "verne_piped",
      "group": "chunk_tester",
      "name": "ts"
    }
  },
  "status": "some videos found"
}

$ curl http://127.0.0.1:8081/video/list/chunk_tester 2>/dev/null | jq
{
  "result": {
    "verne_piped": {
      "id": "verne_piped",
      "group": "chunk_tester",
      "name": "ts"
    }
  },
  "status": "group found"
}

#upload done
$ curl "http://127.0.0.1:8081/video/play/verne_piped" 2>/dev/null | tail
 12530
 12531
 12532  Most people start at our Web site which has the main PG search facility:
 12533
 12534       https://www.gutenberg.org
 12535
 12536  This Web site includes information about Project Gutenberg-tm,
 12537  including how to make donations to the Project Gutenberg Literary
 12538  Archive Foundation, how to help produce our new eBooks, and how to
 12539  subscribe to our email newsletter to hear about new eBooks.
 
 $ curl -X DELETE "http://127.0.0.1:8081/video/delete/verne_piped" 2>/dev/null| jq
{
  "status": "delete ok"
}

$ curl -X DELETE "http://127.0.0.1:8081/video/delete/verne_piped" 2>/dev/null| jq
{
  "status": "video_id not found"
}

$ curl -X POST "http://127.0.0.1:8081/video/clear" 2>/dev/null| jq
{
  "result": null,
  "status": "clear ok"
}

$ curl http://127.0.0.1:8081/video/all 2>/dev/null| jq
{
  "result": null,
  "status": "none videos found"
}
```

#############################


*BATCH FILL*
[examples](examples)
