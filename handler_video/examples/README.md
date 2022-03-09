*batch fill*
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

$ ls -la *mp4
-rw-r--r-- 1 conan conan  17557991 Mar  5 22:48 da_hool_meet_her_at_the_loveparade_official_video_hq_360p_.mp4
-rw-r--r-- 1 conan conan  22889687 Mar  5 22:52 das_modul_computerliebe_480p_.mp4
-rw-r--r-- 1 conan conan  83352731 Mar  5 22:57 dj_mag_presents_mark_spoon_rip_love_parade_1998_480p_.mp4
-rw-r--r-- 1 conan conan  58462389 Oct 24 17:47 dmnds.mp4
-rw-r--r-- 1 conan conan  17866333 Mar  5 22:44 dr_motte_you_cant_stop_us_loveparade_2001_anthem_360p_.mp4
-rw-r--r-- 1 conan conan  65170716 Mar  5 22:43 love_parade_1997_2010_hymny_anthems_2015_hq_480p_.mp4
-rw-r--r-- 1 conan conan  24128474 Mar  5 22:49 love_parade_1997_sunshine_480p_.mp4
-rw-r--r-- 1 conan conan  64179630 Oct 24 17:42 love_tonight_1_hour.mp4
-rw-r--r-- 1 conan conan   8320394 Oct 24 18:09 love_tonight_extended_mix.mp4
-rw-r--r-- 1 conan conan  99354662 Oct 24 17:46 love_tonight.mp4
-rw-r--r-- 1 conan conan  26952961 Oct 24 18:12 munch_roses_extended_remix.mp4
-rw-r--r-- 1 conan conan   9381506 Mar  5 22:31 photek_ni_ten_ichi_ryu_360p_.mp4
-rw-r--r-- 1 conan conan 114652063 Mar  5 22:25 rave_1997_levels_3_4_5_6_7_8_360p_.mp4
-rw-r--r-- 1 conan conan  23944577 Mar  5 22:28 rave_party_1997_level_4_360p_.mp4
-rw-r--r-- 1 conan conan  49936781 Mar  5 22:28 rave_party_1997_level_5_360p_.mp4
-rw-r--r-- 1 conan conan  23901069 Mar  5 22:26 rave_party_1997_level_6_dance_or_die_360p_.mp4
-rw-r--r-- 1 conan conan  18959267 Mar  5 22:27 rave_party_1997_level_7_next_gear_360p_.mp4
-rw-r--r-- 1 conan conan  20715589 Oct 24 17:49 smack.mp4
-rw-r--r-- 1 conan conan 273074254 Mar  5 22:38 x_mix_electro_boogie_480p_.mp4
-rw-r--r-- 1 conan conan  30467586 Oct 24 18:10 zhu_in_the_morning_bassboost_extended_remix.mp4
fill_video conan@spongebob:~/video/youtube$ ls -la /home/conan/soft/rust/handler_video/html/
total 124
drwxr-xr-x 2 conan conan 24576 Mar  9 12:07 .
drwxr-xr-x 7 conan conan  4096 Mar  9 10:16 ..
-rw-r--r-- 1 conan conan   279 Mar  9 12:07 0c99c5e5-425a-426c-9127-7f2aaada57c4_rave_party_1997_level_7_next_gear_360p_.html
-rw-r--r-- 1 conan conan   298 Mar  9 12:07 102ef6c3-364b-4f9e-8da7-d4cdb08e7ea8_da_hool_meet_her_at_the_loveparade_official_video_hq_360p_.html
-rw-r--r-- 1 conan conan   293 Mar  9 12:07 18f1e3bd-e4fd-4084-8bd2-6e09e5cec2ff_dj_mag_presents_mark_spoon_rip_love_parade_1998_480p_.html
-rw-r--r-- 1 conan conan   289 Mar  9 12:07 1d9cc479-1757-41d4-9b2f-e4397a9b96b8_love_parade_1997_2010_hymny_anthems_2015_hq_480p_.html
-rw-r--r-- 1 conan conan   274 Mar  9 12:07 422a66b0-ed34-4746-9595-4f49a3feccbb_rave_1997_levels_3_4_5_6_7_8_360p_.html
-rw-r--r-- 1 conan conan   282 Mar  9 12:07 4232f777-52e3-4bde-9c22-7ba357fa24bf_rave_party_1997_level_6_dance_or_die_360p_.html
-rw-r--r-- 1 conan conan   245 Mar  9 12:07 49eef39a-1211-48e3-b40b-7b48c1fd6857_smack.html
-rw-r--r-- 1 conan conan   259 Mar  9 12:07 4be361af-5ac8-45e9-a38f-a75780c666e8_love_tonight_1_hour.html
-rw-r--r-- 1 conan conan   269 Mar  9 12:07 521b2d54-9131-43c9-bfaa-83ab451cffd9_rave_party_1997_level_5_360p_.html
-rw-r--r-- 1 conan conan   283 Mar  9 12:07 5d7d09c5-9a13-43b0-a0a5-970788574c9f_zhu_in_the_morning_bassboost_extended_remix.html
-rw-r--r-- 1 conan conan   245 Mar  9 12:07 5f0d2996-6906-4531-86df-b5ac16d1bd55_dmnds.html
-rw-r--r-- 1 conan conan   265 Mar  9 12:07 71222c11-5446-4e00-8702-68836460766f_love_tonight_extended_mix.html
-rw-r--r-- 1 conan conan   252 Mar  9 12:07 723871b5-0c6a-4d84-90b0-fe0ffc06ca5c_love_tonight.html
-rw-r--r-- 1 conan conan   269 Mar  9 12:07 72aa8b22-7f98-496c-b6b0-59ac9d22d9ed_rave_party_1997_level_4_360p_.html
-rw-r--r-- 1 conan conan   269 Mar  9 12:07 7a1fa6fe-ea4a-4082-bc9d-04e0a74c4be6_das_modul_computerliebe_480p_.html
-rw-r--r-- 1 conan conan   298 Mar  9 12:07 846e65bc-340c-4527-8800-4893e0ffa291_lines_twenty_thousand_leagues_under_the_sea_by_jules_verne.html
-rw-r--r-- 1 conan conan   266 Mar  9 12:07 882ba615-1d1c-40ce-967b-9926a355b21a_munch_roses_extended_remix.html
-rw-r--r-- 1 conan conan   268 Mar  9 12:07 8b17030f-bc67-4358-ab82-9adc70899bbd_photek_ni_ten_ichi_ryu_360p_.html
-rw-r--r-- 1 conan conan   292 Mar  9 12:07 8d5b0de2-94c4-4de6-a169-c204b4aea72d_twenty_thousand_leagues_under_the_sea_by_jules_verne.html
-rw-r--r-- 1 conan conan   271 Mar  9 12:07 8f831e9e-5d8a-4480-a0ac-a3812239be03_love_parade_1997_sunshine_480p_.html
-rw-r--r-- 1 conan conan   266 Mar  9 12:07 9e3aaad9-f5bc-4cba-b259-3b712e49622f_x_mix_electro_boogie_480p_.html
-rw-r--r-- 1 conan conan   249 Mar  9 12:07 ba7c8b0f-6672-48ee-bba8-a54b87304432_timestamp.html
-rw-r--r-- 1 conan conan   294 Mar  9 12:07 d7c01c7e-4f26-424a-b41e-6ffe45be98a4_dr_motte_you_cant_stop_us_loveparade_2001_anthem_360p_.html
fill_video conan@spongebob:~/video/youtube$ curl http://127.0.0.1:8081/video/all 2>/dev/null| jq
{
  "result": {
    "7a1fa6fe-ea4a-4082-bc9d-04e0a74c4be6": {
      "id": "7a1fa6fe-ea4a-4082-bc9d-04e0a74c4be6",
      "group": "youtube",
      "name": "das_modul_computerliebe_480p_"
    },
    "4be361af-5ac8-45e9-a38f-a75780c666e8": {
      "id": "4be361af-5ac8-45e9-a38f-a75780c666e8",
      "group": "youtube",
      "name": "love_tonight_1_hour"
    },
    "422a66b0-ed34-4746-9595-4f49a3feccbb": {
      "id": "422a66b0-ed34-4746-9595-4f49a3feccbb",
      "group": "youtube",
      "name": "rave_1997_levels_3_4_5_6_7_8_360p_"
    },
    "72aa8b22-7f98-496c-b6b0-59ac9d22d9ed": {
      "id": "72aa8b22-7f98-496c-b6b0-59ac9d22d9ed",
      "group": "youtube",
      "name": "rave_party_1997_level_4_360p_"
    },
    "ba7c8b0f-6672-48ee-bba8-a54b87304432": {
      "id": "ba7c8b0f-6672-48ee-bba8-a54b87304432",
      "group": "youtube",
      "name": "timestamp"
    },
    "5f0d2996-6906-4531-86df-b5ac16d1bd55": {
      "id": "5f0d2996-6906-4531-86df-b5ac16d1bd55",
      "group": "youtube",
      "name": "dmnds"
    },
    "5d7d09c5-9a13-43b0-a0a5-970788574c9f": {
      "id": "5d7d09c5-9a13-43b0-a0a5-970788574c9f",
      "group": "youtube",
      "name": "zhu_in_the_morning_bassboost_extended_remix"
    },
    "723871b5-0c6a-4d84-90b0-fe0ffc06ca5c": {
      "id": "723871b5-0c6a-4d84-90b0-fe0ffc06ca5c",
      "group": "youtube",
      "name": "love_tonight"
    },
    "8f831e9e-5d8a-4480-a0ac-a3812239be03": {
      "id": "8f831e9e-5d8a-4480-a0ac-a3812239be03",
      "group": "youtube",
      "name": "love_parade_1997_sunshine_480p_"
    },
    "49eef39a-1211-48e3-b40b-7b48c1fd6857": {
      "id": "49eef39a-1211-48e3-b40b-7b48c1fd6857",
      "group": "youtube",
      "name": "smack"
    },
    "882ba615-1d1c-40ce-967b-9926a355b21a": {
      "id": "882ba615-1d1c-40ce-967b-9926a355b21a",
      "group": "youtube",
      "name": "munch_roses_extended_remix"
    },
    "d7c01c7e-4f26-424a-b41e-6ffe45be98a4": {
      "id": "d7c01c7e-4f26-424a-b41e-6ffe45be98a4",
      "group": "youtube",
      "name": "dr_motte_you_cant_stop_us_loveparade_2001_anthem_360p_"
    },
    "1d9cc479-1757-41d4-9b2f-e4397a9b96b8": {
      "id": "1d9cc479-1757-41d4-9b2f-e4397a9b96b8",
      "group": "youtube",
      "name": "love_parade_1997_2010_hymny_anthems_2015_hq_480p_"
    },
    "4232f777-52e3-4bde-9c22-7ba357fa24bf": {
      "id": "4232f777-52e3-4bde-9c22-7ba357fa24bf",
      "group": "youtube",
      "name": "rave_party_1997_level_6_dance_or_die_360p_"
    },
    "102ef6c3-364b-4f9e-8da7-d4cdb08e7ea8": {
      "id": "102ef6c3-364b-4f9e-8da7-d4cdb08e7ea8",
      "group": "youtube",
      "name": "da_hool_meet_her_at_the_loveparade_official_video_hq_360p_"
    },
    "0c99c5e5-425a-426c-9127-7f2aaada57c4": {
      "id": "0c99c5e5-425a-426c-9127-7f2aaada57c4",
      "group": "youtube",
      "name": "rave_party_1997_level_7_next_gear_360p_"
    },
    "8b17030f-bc67-4358-ab82-9adc70899bbd": {
      "id": "8b17030f-bc67-4358-ab82-9adc70899bbd",
      "group": "youtube",
      "name": "photek_ni_ten_ichi_ryu_360p_"
    },
    "18f1e3bd-e4fd-4084-8bd2-6e09e5cec2ff": {
      "id": "18f1e3bd-e4fd-4084-8bd2-6e09e5cec2ff",
      "group": "youtube",
      "name": "dj_mag_presents_mark_spoon_rip_love_parade_1998_480p_"
    },
    "71222c11-5446-4e00-8702-68836460766f": {
      "id": "71222c11-5446-4e00-8702-68836460766f",
      "group": "youtube",
      "name": "love_tonight_extended_mix"
    },
    "521b2d54-9131-43c9-bfaa-83ab451cffd9": {
      "id": "521b2d54-9131-43c9-bfaa-83ab451cffd9",
      "group": "youtube",
      "name": "rave_party_1997_level_5_360p_"
    },
    "846e65bc-340c-4527-8800-4893e0ffa291": {
      "id": "846e65bc-340c-4527-8800-4893e0ffa291",
      "group": "youtube",
      "name": "lines_twenty_thousand_leagues_under_the_sea_by_jules_verne"
    },
    "9e3aaad9-f5bc-4cba-b259-3b712e49622f": {
      "id": "9e3aaad9-f5bc-4cba-b259-3b712e49622f",
      "group": "youtube",
      "name": "x_mix_electro_boogie_480p_"
    },
    "8d5b0de2-94c4-4de6-a169-c204b4aea72d": {
      "id": "8d5b0de2-94c4-4de6-a169-c204b4aea72d",
      "group": "youtube",
      "name": "twenty_thousand_leagues_under_the_sea_by_jules_verne"
    }
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
