for i in {1..10}; do
#for i in {1..20}; do
    DATE="$(date --rfc-3339=ns --utc)"
    >&2 echo "sending '$DATE'"
    echo $DATE
    sleep 1
done | curl --silent --verbose --no-buffer --raw -X PUT -T - http://localhost:8081/foo/bar

#done | curl --silent --verbose --no-buffer --raw -X PUT -H "Content-type: application/x-www-form-urlencoded" -T - http://localhost:8081/foo/bar

#enctype="multipart/form-data"
#done | curl --silent --verbose --no-buffer --raw -X PUT -H "Transfer-Encoding: chunked" -H "Content-type: multipart/form-data" -T - http://localhost:8081/foo/bar

# -T --upload-file

#cat /home/conan/video/youtube/lines_twenty_thousand_leagues_under_the_sea_by_jules_verne.txt | curl -v -X PUT -H "Transfer-Encoding: chunked" -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/upload" -F "ts=@-;type=text/plain" -H "video_id: verne_piped" -H "group: chunk_tester" --no-buffer --limit-rate 10K
