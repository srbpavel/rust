#!/bin/sh

###bash -x

#SERVER=192.168.0.103 # via dlink
#SERVER=192.168.88.253 # via mikrotik
SERVER=127.0.0.1
#SERVER=localhost
PORT=8081


curl "http://${SERVER}:${PORT}/video/"

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/put" -F "name_001=@/home/conan/video/youtube/smack.mp4;type=video/mp4" -H "video_id: 123" -H "group: stream_001" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/put" -F "stream_001=@/home/conan/video/youtube/love_tonight_extended_mix.mp4;type=video/mp4" -H "video_id: 456" -H "group: stream_001" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/put" -F "stream_002=@/home/conan/video/youtube/munch_roses_extended_remix.mp4;type=video/mp4" -H "video_id: 789" -H "group: stream_002" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/put" -F "stream_003=@/home/conan/video/youtube/dmnds.mp4;type=video/mp4" -H "video_id: 357" -H "group: stream_003" 2>/dev/null | jq

curl "http://${SERVER}:${PORT}/video/" 2>/dev/null | jq

curl "http://${SERVER}:${PORT}/video/detail/123" 2>/dev/null | jq

curl "http://${SERVER}:${PORT}/video/detail/456" 2>/dev/null | jq


echo "LIST: curl http://${SERVER}:${PORT}/video/ 2>/dev/null| jq"

echo "INSERT: curl -X PUT -H \"Content-type: multipart/form-data\" \"http://${SERVER}:${PORT}/video/put\" -F \"name_001=@/home/conan/video/youtube/smack.mp4;type=video/mp4\" -H \"video_id: 123\" -H \"group: stream_001\" 2>/dev/null | jq"

echo "DETAIL: curl \"http://${SERVER}:${PORT}/video/detail/123\" 2>/dev/null| jq"

echo "UPDATE: curl -X POST \"http://${SERVER}:${PORT}/video/update/group\" -H \"Content-Type: application/json\" -d '{\"video_id\": \"123\", \"group_id\": \"video_on_demand\"}' 2>/dev/null | jq"

echo "DELETE: curl -X DELETE \"http://${SERVER}:${PORT}/video/delete/123\" 2>/dev/null| jq"

echo "LIST GROUPS: curl \"http://${SERVER}:${PORT}/video/groups\" 2>/dev/null| jq"

echo "LIST GROUP MEMBERS: curl http://${SERVER}:${PORT}/video/list/stream_001 2>/dev/null | jq"

echo "CLEAR: curl -X POST \"http://${SERVER}:${PORT}/video/clear\" 2>/dev/null| jq"

echo "DOWNLOAD: vlc \"http://${SERVER}:${PORT}/video/download/357\""
