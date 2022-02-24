#!/bin/sh

###bash -x

#flush old data
curl -X POST 'http://localhost:8081/clear' | jq

#initial read
curl 'http://localhost:8081/' | jq

#fill vec with samples
curl -X POST "http://localhost:8081/send" -H "Content-Type: application/json" -d '{"message": "fazole"}' | jq

curl -X POST "http://localhost:8081/send" -H "Content-Type: application/json" -d '{"message": "puerh"}' | jq

curl -X POST "http://localhost:8081/send" -H "Content-Type: application/json" -d '{"message": "siipkovej"}' | jq

curl -X POST "http://localhost:8081/send" -H "Content-Type: application/json" -d '{"message": "loituma"}' | jq

# VEC obsolete !!!
#get by absolute position
#curl 'http://localhost:8081/lookup/0' | jq
#curl 'http://localhost:8081/lookup/1' | jq
#curl 'http://localhost:8081/lookup/2' | jq

#relative position
#curl 'http://localhost:8081/lookup/-1' | jq

#error msg
#curl 'http://localhost:8081/lookup/last' | jq

curl 'http://localhost:8081/search/0' 2>/dev/null | jq

#all msg
curl 'http://localhost:8081/' | jq

echo "LIST: curl 'http://localhost:8081/' 2>/dev/null | jq"
echo "SEND: curl -X POST \"http://localhost:8081/send\" -H \"Content-Type: application/json\" -d '{\"message\": \"rambo\"}' 2>/dev/null | jq"
# VEC obsolete !!!
#echo "LAST: curl 'http://localhost:8081/lookup/-1' 2>/dev/null | jq"
#echo "FIRST: curl 'http://localhost:8081/lookup/0' 2>/dev/null | jq"
# HASH
echo "ID: curl 'http://localhost:8081/search/0' 2>/dev/null | jq"
echo "CLEAR >>> curl -X POST 'http://localhost:8081/clear' 2>/dev/null | jq"
