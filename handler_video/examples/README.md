<b>batch fill</b>
 - to verify service accessibility durring upload
 - how fast is response for methods, when threads busy uploading

```
$ rm /home/conan/soft/rust/handler_video/html/*html

$ ls -la /home/conan/soft/rust/handler_video/html/
total 32
drwxr-xr-x 2 conan conan 24576 Mar  9 12:04 .
drwxr-xr-x 7 conan conan  4096 Mar  9 10:16 ..


$ cargo run --example main /home/conan/soft/rust/handler_video/examples/example_fill_config.toml
    Finished dev [unoptimized + debuginfo] target(s) in 0.69s
     Running `/home/conan/soft/rust/handler_video/target/debug/examples/main /home/conan/soft/rust/handler_video/examples/example_fill_config.toml`

#CONFIG:
TomlConfig {
    name: "VIDEO_UPLOADER",
    secure: "http",
    host: "spongebob",
    server: "localhost",
    port: 8081,
    curl_limit_rate: "10M",
    video_group: "youtube",
    upload_path: "/video/upload",
    player_path: "/video/play",
    video_dir: "/home/conan/video/youtube",
    sample_limit_start: 0,
    sample_limit_end: -1,
    html_path: "/home/conan/soft/rust/handler_video/html/",
    html_template: "<html>\n\t<body>\n\t<p>handler_video</p>\n\t<p>\n\t\t{all_videos}\n\t</p>\n\t</body>\n</html>",
    video_tag: "\n\t\t<p><i>{name}</i></p>\n\t\t<video width={width} controls autoplay muted>\n\t\t\t<source src={src} type=\"{type}\"/>\n\t\t</video>",
    player_width: "640",
    content_type: "video/mp4",
    flag: Flag {
        debug_config: true,
        debug_template: false,
    },
}
#CMD: "curl" "-X" "PUT" "-H" "Content-type: multipart/form-data" "http://localhost:8081/video/upload" "-F" "rave_party_1997_level_6_dance_or_die_360p_=@/home/conan/video/youtube/rave_party_1997_level_6_dance_or_die_360p_.mp4;type=video/mp4" "-H" "video_id: 4232f777-52e3-4bde-9c22-7ba357fa24bf" "-H" "group: youtube" "--no-buffer" "--limit-rate" "10M"
...
#UPLOADED: /home/conan/video/youtube/da_hool_meet_her_at_the_loveparade_official_video_hq_360p_.mp4
...

$ ls -la *mp4
-rw-r--r-- 1 conan conan  17557991 Mar  5 22:48 da_hool_meet_her_at_the_loveparade_official_video_hq_360p_.mp4
...

$ ls -la /home/conan/soft/rust/handler_video/html/
total 124
drwxr-xr-x 2 conan conan 24576 Mar  9 12:07 .
drwxr-xr-x 7 conan conan  4096 Mar  9 10:16 ..
-rw-r--r-- 1 conan conan   279 Mar  9 12:07 0c99c5e5-425a-426c-9127-7f2aaada57c4_rave_party_1997_level_7_next_gear_360p_.html
...

$ curl http://127.0.0.1:8081/video/all 2>/dev/null| jq
{
  "result": {
    "7a1fa6fe-ea4a-4082-bc9d-04e0a74c4be6": {
      "id": "7a1fa6fe-ea4a-4082-bc9d-04e0a74c4be6",
      "group": "youtube",
      "name": "das_modul_computerliebe_480p_"
    },
    ...
  },
  "status": "some videos found"
}

$ curl "http://127.0.0.1:8081/video/detail/9e3aaad9-f5bc-4cba-b259-3b712e49622f" 2>/dev/null| jq
{
  "result": {
    "id": "9e3aaad9-f5bc-4cba-b259-3b712e49622f",
    "group": "youtube",
    "name": "x_mix_electro_boogie_480p_"
  },
  "status": "video_id found"
}


#now we can play uploaded chunks video
$ cat /home/conan/soft/rust/handler_video/html/9e3aaad9-f5bc-4cba-b259-3b712e49622f_x_mix_electro_boogie_480p_.html 
<html>
        <body>
        <p>handler_video</p>
        <p>
                <p><i>x_mix_electro_boogie_480p_</i></p>
                <video width=640 controls autoplay muted>
                        <source src=http://localhost:8081/video/play/9e3aaad9-f5bc-4cba-b259-3b712e49622f type="video/mp4"/>
                </video>
        </p>
        </body>
</html>
```
