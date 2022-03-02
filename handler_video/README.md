# handler_video

<b>actix lesson</b>

*CONFIG and RUN*
```
$ cat src/handler_video_config.toml
name = 'HANDLER_VIDEO'
host = 'spongebob'
#server = "127.0.0.1"
server = "localhost"
#server = "192.168.0.103"
#server = "192.168.88.253"
port = 8081
workers = 4
static_dir = "/home/conan/soft/rust/handler_video/storage/"
log_format = "\"%r\" %s %b \"%{User-Agent}i\" %D"

[flag]
debug_config = true # false
#verify dir is available and we can write before each video safe             
verify_dir_per_video = true # false

$ cargo run
EXIT: Problem parsing cmd arguments
REASON >>> we want exactly one argument
 example:
  $ cargo run /home/conan/soft/rust/handler_video/src/handler_video_config.toml
  $ /home/conan/soft/rust/handler_video/target/debug/handler_video /home/conan/soft/rust/handler_video/src/handler_video_config.toml

$ cargo run /home/conan/soft/rust/handler_video/src/handler_video_config.toml
VERIFY STORAGE: Error: video_storage directory does not exists: "\"/home/conan/soft/rust/handler_video/storage/\""

$ mkdir storage
$ /home/conan/soft/rust/handler_video/target/debug/handler_video /home/conan/soft/rust/handler_video/src/handler_video_config.toml

#CONFIG:
TomlConfig {
    name: "HANDLER_VIDEO",
    host: "spongebob",
    server: "localhost",
    port: 8081,
    workers: 4,
    static_dir: "/home/conan/soft/rust/handler_video/storage/",
    log_format: "\"%r\" %s %b \"%{User-Agent}i\" %D",
    flag: Flag {
        debug_config: true,
        verify_dir_per_video: true,
    },
}
start -> HANDLER_VIDEO at spongebob / localhost
[2022-03-02T18:13:15Z INFO  actix_server::builder] Starting 4 workers
[2022-03-02T18:13:15Z INFO  actix_server::builder] Starting "actix-web-service-127.0.0.1:8081" service on 127.0.0.1:8081
```

*EMPTY START*
```
$ curl "http://127.0.0.1:8081/video/" 2>/dev/null| jq
{
  "server_id": 0,
  "request_count": 1,
  "video_map": {},
  "status": "ok"
}

$ curl "http://127.0.0.1:8081/video/groups" 2>/dev/null| jq
{
  "server_id": 2,
  "request_count": 1,
  "result": null,
  "status": "no groups found"
}

$ curl http://localhost:8081/video/list/stream_001 2>/dev/null | jq
{
  "server_id": 1,
  "request_count": 1,
  "result": null,
  "status": "group not found"
}

curl "http://127.0.0.1:8081/video/detail/123" 2>/dev/null| jq
{
  "server_id": 3,
  "request_count": 1,
  "result": null,
  "url": "/video/detail/123",
  "status": "video_id not found"
}
```

*IMPORT*
```
$ curl -X PUT -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/put" -F "auticko=/home/conan/video/youtube/dmnds.mp4;type=video/mp4" -H "video_id: 123" -H "group: stream_001" 2>/dev/null | jq
{
  "result": null,
  "status": "form 'filename' not provided"
}

$ curl -X PUT -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/put" -F "=@/home/conan/video/youtube/dmnds.mp4;type=video/mp4" -H "video_id: 123" -H "group: stream_001" 2>/dev/null | jq
{
  "result": null,
  "status": "'name' not provided for form"
}

$ curl -X PUT -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/put" -F "=@/home/conan/video/youtube/dmnds.mp4;type=video/mp4" -H "video_id: 123" -H "Xgroup: stream_001" 2>/dev/null | jq
{
  "result": null,
  "status": "header 'group' not provided"
}

$ curl -X PUT -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/put" -F "remix=@/home/conan/video/youtube/dmnds.mp4;type=video/mp4" -H "Xvideo_id: 123" -H "group: stream_001" 2>/dev/null | jq
{
  "result": null,
  "status": "header 'video_id' not provided"
}

$ curl -X PUT -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/put" -F "remix=@/home/conan/video/youtube/dmnds.mp4;type=video/mp4" -H "video_id: 123" -H "group: stream_001" 2>/dev/null | jq
{
  "result": {
    "server_id": 2,
    "request_count": 3,
    "video": {
      "id": "123",
      "group": "stream_001",
      "name": "remix",
      "path": "/home/conan/soft/rust/handler_video/storage/123_dmnds.mp4"
    }
  },
  "status": "ok"
}

$ ls -la storage/
total 57104
drwxr-xr-x 2 conan conan     4096 Mar  2 18:59 .
drwxr-xr-x 6 conan conan     4096 Mar  2 18:34 ..
-rw-r--r-- 1 conan conan 58462389 Mar  2 18:59 123_dmnds.mp4
-rw-r--r-- 1 conan conan        0 Mar  2 18:59 touch.verify

```

```
$ curl "http://127.0.0.1:8081/video/detail/123" 2>/dev/null| jq{
  "server_id": 1,
  "request_count": 3,
  "result": {
    "id": "123",
    "group": "stream_001",
    "name": "remix",
    "path": "/home/conan/soft/rust/handler_video/storage/123_dmnds.mp4"
  },
  "url": "/video/detail/123",
  "status": "video found"
}

$ curl http://localhost:8081/video/list/stream_001 2>/dev/null | jq
{
  "server_id": 3,
  "request_count": 3,
  "result": {
    "123": {
      "id": "123",
      "group": "stream_001",
      "name": "remix",
      "path": "/home/conan/soft/rust/handler_video/storage/123_dmnds.mp4"
    }
  },
  "status": "group found"
}

$ curl "http://127.0.0.1:8081/video/groups" 2>/dev/null| jq{
  "server_id": 0,
  "request_count": 4,
  "result": [
    "stream_001"
  ],
  "status": "some groups found"
}
```

```
$ curl -X POST "http://127.0.0.1:8081/video/update/group" -H "Content-Type: application/json" -d '{"video_id": "123", "group_id": "video_on_demand"}' 2>/dev/null | jq
{
  "server_id": 2,
  "request_count": 4,
  "result": {
    "id": "123",
    "group": "video_on_demand",
    "name": "remix",
    "path": "/home/conan/soft/rust/handler_video/storage/123_dmnds.mp4"
  },
  "url": "/video/update/group",
  "status": "update ok"
}

$ curl "http://127.0.0.1:8081/video/detail/123" 2>/dev/null| jq
{
  "server_id": 1,
  "request_count": 4,
  "result": {
    "id": "123",
    "group": "video_on_demand",
    "name": "remix",
    "path": "/home/conan/soft/rust/handler_video/storage/123_dmnds.mp4"
  },
  "url": "/video/detail/123",
  "status": "video found"
}

$ curl -X DELETE "http://127.0.0.1:8081/video/delete/123" 2>/dev/null | jq
{
  "result":"delete ok"
}

$ curl "http://127.0.0.1:8081/video/detail/123" 2>/dev/null| jq
{
  "server_id": 0,
  "request_count": 7,
  "result": null,
  "url": "/video/detail/123",
  "status": "video_id not found"
}

$ ls -la storage/
total 8
drwxr-xr-x 2 conan conan 4096 Mar  2 18:56 .
drwxr-xr-x 6 conan conan 4096 Mar  2 18:34 ..
-rw-r--r-- 1 conan conan    0 Mar  2 18:56 touch.verify

$ curl -X DELETE "http://127.0.0.1:8081/video/delete/123" 2>/dev/null | jq
{
  "result": "video_id not found"
}
```

*BATCH FILL*
```
$ ./fill_spongebob.sh 

$ curl "http://127.0.0.1:8081/video/" 2>/dev/null| jq
{
  "server_id": 0,
  "request_count": 3,
  "video_map": {
    "456": {
      "id": "456",
      "group": "stream_001",
      "name": "stream_001",
      "path": "/home/conan/soft/rust/handler_video/storage/456_love_tonight_extended_mix.mp4"
    },
    "357": {
      "id": "357",
      "group": "stream_003",
      "name": "stream_003",
      "path": "/home/conan/soft/rust/handler_video/storage/357_dmnds.mp4"
    },
    "789": {
      "id": "789",
      "group": "stream_002",
      "name": "stream_002",
      "path": "/home/conan/soft/rust/handler_video/storage/789_munch_roses_extended_remix.mp4"
    },
    "123": {
      "id": "123",
      "group": "stream_001",
      "name": "name_001",
      "path": "/home/conan/soft/rust/handler_video/storage/123_smack.mp4"
    }
  },
  "status": "ok"
}

$ curl -X POST "http://127.0.0.1:8081/video/clear" 2>/dev/null| jq
{
  "server_id": 1,
  "request_count": 3,
  "video_map": {},
  "status": "ok"
}
fill conan@spongebob:~/soft/rust/handler_video$ curl "http://127.0.0.1:8081/video/" 2>/dev/null| jq
{
  "server_id": 3,
  "request_count": 3,
  "video_map": {},
  "status": "ok"
}
```

*DOWNLOAD* 
```
vlc 'http://ruth:8081/video/download/abc123'
```
