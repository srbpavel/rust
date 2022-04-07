#!/bin/sh

curl -X PUT http://localhost:8081/tonda/pepa -d "1234567890a"
curl -X PUT http://localhost:8081/pepa/tonda/franta -d "1234567890b"
curl -X PUT http://localhost:8081/pepa/tonda/lojza -d "1234567890c"

curl -X PUT http://localhost:8081/pepa/tonda/ -d "1234567890d"
#curl -X PUT http://localhost:8081/pepa/tonda -d "1234567890"

curl -X PUT http://localhost:8081/foo -d "1234567890e"
curl -X PUT http://localhost:8081/foo/pepa -d "1234567890f"
