*PUT*
```
$ ./chunks.sh 
sending '2022-03-23 11:01:42.617909546+00:00'
*   Trying 127.0.0.1:8081...
* Connected to localhost (127.0.0.1) port 8081 (#0)
> PUT /foo/bar HTTP/1.1
> Host: localhost:8081
> User-Agent: curl/7.74.0
> Accept: */*
> Transfer-Encoding: chunked
> Expect: 100-continue
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 100 Continue
sending '2022-03-23 11:01:43.628144711+00:00'
sending '2022-03-23 11:01:44.636684440+00:00'
sending '2022-03-23 11:01:45.645499602+00:00'
sending '2022-03-23 11:01:46.655412179+00:00'
sending '2022-03-23 11:01:47.663454189+00:00'
sending '2022-03-23 11:01:48.668420300+00:00'
sending '2022-03-23 11:01:49.673739773+00:00'
sending '2022-03-23 11:01:50.681432117+00:00'
sending '2022-03-23 11:01:51.689818652+00:00'
* Signaling end of chunked upload via terminating chunk.
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 16
< date: Wed, 23 Mar 2022 11:01:52 GMT
< 
Status::UploadOk* Connection #0 to host localhost left intact
```

*GET*
```
$ curl --silent --verbose --no-buffer http://localhost:8081/foo/bar/
*   Trying 127.0.0.1:8081...
* Connected to localhost (127.0.0.1) port 8081 (#0)
> GET /foo/bar/ HTTP/1.1
> Host: localhost:8081
> User-Agent: curl/7.74.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 360
< content-type: application/octet-stream
< content-encoding: chunked
< date: Wed, 23 Mar 2022 11:02:22 GMT
< 
2022-03-23 11:01:42.617909546+00:00
2022-03-23 11:01:43.628144711+00:00
2022-03-23 11:01:44.636684440+00:00
2022-03-23 11:01:45.645499602+00:00
2022-03-23 11:01:46.655412179+00:00
2022-03-23 11:01:47.663454189+00:00
2022-03-23 11:01:48.668420300+00:00
2022-03-23 11:01:49.673739773+00:00
2022-03-23 11:01:50.681432117+00:00
2022-03-23 11:01:51.689818652+00:00
* Connection #0 to host localhost left intact
```
*DELETE*
```
$ curl -X DELETE http://localhost:8081/foo/bar
Status::DeleteOk

$ curl -X DELETE http://localhost:8081/foo/bar
Status::DeleteBinaryError
```

*PUT* via pipe
```
cat /home/conan/video/youtube/lines_twenty_thousand_leagues_under_the_sea_by_jules_verne.txt | curl -v -X PUT "http://127.0.0.1:8081/jules/verne/twenty" --no-buffer --limit-rate 100K -T -
*   Trying 127.0.0.1:8081...
* Connected to 127.0.0.1 (127.0.0.1) port 8081 (#0)
> PUT /jules/verne/twenty HTTP/1.1
> Host: 127.0.0.1:8081
> User-Agent: curl/7.74.0
> Accept: */*
> Transfer-Encoding: chunked
> Expect: 100-continue
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 100 Continue
* Signaling end of chunked upload via terminating chunk.
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 16
< date: Wed, 23 Mar 2022 11:04:57 GMT
< 
Status::UploadOk* Connection #0 to host 127.0.0.1 left intact
```

*GET*
```
watch "curl --silent --verbose --no-buffer http://localhost:8081/jules/verne/twenty|tail"
*   Trying 127.0.0.1:8081...
* Connected to localhost (127.0.0.1) port 8081 (#0)
> GET /jules/verne/twenty HTTP/1.1
> Host: localhost:8081
> User-Agent: curl/7.74.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 709440
< content-type: application/octet-stream
< content-encoding: chunked
< date: Wed, 23 Mar 2022 11:08:03 GMT
< 
{ [32621 bytes data]
* Connection #0 to host localhost left intact
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
```
