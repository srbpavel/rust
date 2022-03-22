for i in $(seq 5)
do date +%s%N
#do date ; echo "\n"
   sleep 1
done |
    #dd conv=block cbs=512 |
    dd conv=block cbs=64 |
    #strace -t -e sendto,read -o /home/conan/soft/rust/handler_video/timestamp.txt curl -X PUT -H "Transfer-Encoding: chunked" -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/upload" -F "ts=@-;type=text/plain" -H "video_id: now" -H "group: timestamp" --no-buffer
    #strace -t -e sendto,read -o /home/conan/soft/rust/handler_video/timestamp.txt curl --trace-ascii - -X PUT -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/video/upload" -F "ts=@-;type=text/plain" --no-buffer --limit-rate 100K
    strace -t -e sendto,read -o /home/conan/soft/rust/handler_video/timestamp.txt curl --trace-ascii - -X PUT "http://127.0.0.1:8081/video/upload" -F "ts=@-;type=text/plain" --no-buffer --limit-rate 100K

