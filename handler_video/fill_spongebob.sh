#!/bin/sh

###bash -x

#SERVER=192.168.0.103 # via dlink
#SERVER=192.168.88.253 # via mikrotik
SERVER=127.0.0.1
#SERVER=localhost
PORT=8081


#curl "http://${SERVER}:${PORT}/video/all"

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/upload" -F "smack=@/home/conan/video/youtube/smack.mp4;type=video/mp4" -H "video_id: 123" -H "group: stream_001" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/upload" -F "extended_mix=@/home/conan/video/youtube/love_tonight_extended_mix.mp4;type=video/mp4" -H "video_id: 456" -H "group: stream_001" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/upload" -F "roses=@/home/conan/video/youtube/munch_roses_extended_remix.mp4;type=video/mp4" -H "video_id: 789" -H "group: stream_001" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/upload" -F "dmnds=@/home/conan/video/youtube/dmnds.mp4;type=video/mp4" -H "video_id: 357" -H "group: stream_001" 2>/dev/null | jq

###
curl -X PUT -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/upload" -F "zhu=@/home/conan/video/youtube/zhu_in_the_morning_bassboost_extended_remix.mp4;type=video/mp4" -H "video_id: 666" -H "group: stream_001" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/upload" -F "da_hool_meet=@/home/conan/video/youtube/da_hool_meet_her_at_the_loveparade_official_video_hq_360p_.mp4" -H "video_id: da_hool" -H "group: youtube" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/upload" -F "das_modul=@/home/conan/video/youtube/das_modul_computerliebe_480p_.mp4" -H "video_id: das_modul_computer" -H "group: youtube" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/upload" -F "dj_mag=@/home/conan/video/youtube/dj_mag_presents_mark_spoon_rip_love_parade_1998_480p_.mp4" -H "video_id: dj_mag_spoon" -H "group: youtube" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/upload" -F "parade_2010=@/home/conan/video/youtube/love_parade_1997_2010_hymny_anthems_2015_hq_480p_.mp4" -H "video_id: love_parade_1997_2010" -H "group: youtube" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/upload" -F "sunshine=@/home/conan/video/youtube/love_parade_1997_sunshine_480p_.mp4" -H "video_id: love_parade_sunshine" -H "group: youtube" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/upload" -F "rave_party_level_4=@/home/conan/video/youtube/rave_party_1997_level_4_360p_.mp4" -H "video_id: level_4" -H "group: youtube" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/upload" -F "x_mix=@/home/conan/video/youtube/x_mix_electro_boogie_480p_.mp4" -H "video_id: electro_boogie" -H "group: youtube" 2>/dev/null | jq

curl -X PUT -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/upload" -F "level_5=@/home/conan/video/youtube/rave_party_1997_level_5_360p_.mp4;type=video/mp4" -H "video_id: rave_level_5" -H "group: stream_001" 2>/dev/null | jq

# EMPTY
#curl -X PUT -H "Content-type: multipart/form-data" "http://${SERVER}:${PORT}/video/upload" -F "s=@/home/conan/video/youtube/" -H "video_id: " -H "group: youtube" 2>/dev/null | jq

#curl "http://${SERVER}:${PORT}/video/all" 2>/dev/null | jq
#curl "http://${SERVER}:${PORT}/video/detail/123" 2>/dev/null | jq

echo "LIST: curl http://${SERVER}:${PORT}/video/all 2>/dev/null| jq"

echo "INSERT: curl -X PUT -H \"Content-type: multipart/form-data\" \"http://${SERVER}:${PORT}/video/upload\" -F \"smack=@/home/conan/video/youtube/smack.mp4;type=video/mp4\" -H \"video_id: 123\" -H \"group: stream_001\" 2>/dev/null | jq"

echo "DETAIL: curl \"http://${SERVER}:${PORT}/video/detail/123\" 2>/dev/null| jq"

echo "DELETE: curl -X DELETE \"http://${SERVER}:${PORT}/video/delete/123\" 2>/dev/null| jq"

echo "LIST GROUP MEMBERS: curl http://${SERVER}:${PORT}/video/list/stream_001 2>/dev/null | jq"

echo "CLEAR: curl -X POST \"http://${SERVER}:${PORT}/video/clear\" 2>/dev/null| jq"

echo "DOWNLOAD: vlc \"http://${SERVER}:${PORT}/video/download/357\""
