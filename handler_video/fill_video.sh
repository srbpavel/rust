#!/bin/sh

###bash -x

#FLUSH
rm tmp/*

curl 'http://localhost:8081/video/' 2>/dev/null| jq


#TEXT
#curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F "text=@now.txt;type=text/plain" 2>/dev/null | jq

#: ' #start comment
#VIDEO
#curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F "stream_001=@/home/conan/video/youtube/smack.mp4;type=video/mp4" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F "name_001=@/home/conan/video/youtube/smack.mp4;type=video/mp4" -H "video_id: 123" -H "group: stream_001" 2>/dev/null | jq

#curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F "stream_001=@/home/conan/video/youtube/love_tonight_extended_mix.mp4;type=video/mp4" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F "stream_001=@/home/conan/video/youtube/love_tonight_extended_mix.mp4;type=video/mp4" -H "video_id: 456" -H "group: stream_001" 2>/dev/null | jq

#curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F "stream_002=@/home/conan/video/youtube/munch_roses_extended_remix.mp4;type=video/mp4" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F "stream_002=@/home/conan/video/youtube/munch_roses_extended_remix.mp4;type=video/mp4" -H "video_id: 789" -H "group: stream_002" 2>/dev/null | jq

#curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F "stream_003=@/home/conan/video/youtube/dmnds.mp4;type=video/mp4" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F "stream_003=@/home/conan/video/youtube/dmnds.mp4;type=video/mp4" -H "video_id: 357" -H "group: stream_003" 2>/dev/null | jq

curl 'http://localhost:8081/video/' 2>/dev/null| jq

curl 'http://localhost:8081/video/detail/123' 2>/dev/null| jq

curl 'http://localhost:8081/video/detail/456' 2>/dev/null| jq
#' #end comment
