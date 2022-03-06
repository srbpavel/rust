```
$ cargo run --example main /home/conan/soft/rust/handler_video/examples/example_fill_config.toml
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/examples/main /home/conan/soft/rust/handler_video/examples/example_fill_config.toml`
UPLOAD_URL: http://localhost:8081/video/upload
PLAYER_URL: http://localhost:8081/video/play
HTMLL: player.html
SAMPLE_LIMIT: 0..-1


#CMD: "curl" "-X" "PUT" "-H" "Content-type: multipart/form-data" "http://localhost:8081/video/upload" "-F" "rave_party_1997_level_6_dance_or_die_360p_=@/home/conan/video/youtube/rave_party_1997_level_6_dance_or_die_360p_.mp4;type=video/mp4" "-H" "video_id: 89712de5-4777-4cc8-983d-4b17b3616aeb" "-H" "group: youtube"
#OUTPUT: Ok(
    Output {
        status: ExitStatus(
            unix_wait_status(
                0,
            ),
        ),
        stdout: "{\"result\":{\"video\":{\"id\":\"89712de5-4777-4cc8-983d-4b17b3616aeb\",\"group\":\"youtube\",\"name\":\"rave_party_1997_level_6_dance_or_die_360p_\"}},\"status\":\"upload finished\"}",
        stderr: "  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current\n                                 Dload  Upload   Total   Spent    Left  Speed\n\r  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0\r100 22.7M  100   163  100 22.7M    423  59.2M --:--:-- --:--:-- --:--:-- 59.3M\n",
    },
)
```
