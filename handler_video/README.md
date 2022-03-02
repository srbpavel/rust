# handler_video

<b>actix lesson</b>

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

$ curl -X DELETE "http://127.0.0.1:8081/video/delete/123" 2>/dev/null | jq
{
  "result": "video_id not found"
}
```
