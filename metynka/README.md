# metynka

<b>level 1 lesson</b>

- read and parse toml config settings
- measure metric sensors
- prepare data and render influxdb template
- import and backup


```
*/5 * * * * /home/conan/.cargo/bin/cargo run --manifest-path /home/conan/soft/rust/metynka/Cargo.toml BUCKET /home/conan/soft/rust/metynka/src/config.toml false 1>/home/conan/soft/rust/metynka/1_cron.log 2>/home/conan/soft/rust/metynka/2_cron.log
```

```
     1
     2  #COMMAND: Args {
     3      inner: [
     4          "soft/rust/metynka/target/debug/metynka",
     5          "BUCKET",
     6          "/home/conan/soft/rust/metynka/src/config.toml",
     7          "false",
     8      ],
     9  }
    10
    11  #PARSE file_config -> TOML:
    12  /home/conan/soft/rust/metynka/src/config.toml
    13
    14  #METRIC: TemperatureDecimal -> MEASUREMENT: temperature
    15  #METRIC: MemoryDecimal -> MEASUREMENT: memory_float
    16
    17  #URI<default>:
    18  https://ruth:8086/api/v2/write?org=foookin_paavel&bucket=test_rust&precision=ms
    19  https://ruth:8086/api/v2/query?org=foookin_paavel
    20
    21  #URI<backup>:
    22  http://jozefina:8086/api/v2/write?org=foookin_paavel&bucket=backup_test_rust&precision=ms
    23  http://jozefina:8086/api/v2/query?org=foookin_paavel
    24
    25  #CSV_ANNOTATED: /home/conan/soft/rust/metynka/csv/2021_11_30_laptop_temperature.csv
    26  #
    27  #datatype measurement,tag,tag,tag,tag,tag,double,dateTime:number
    28  m,host,Machine,SensorId,SensorCarrier,SensorValid,TemperatureDecimal,time
    29  temperature,spongebob,spongebob,0,cargo,true,63,1638259201608
    30  temperature,spongebob,spongebob,1,cargo,true,55,1638259201608
    31  temperature,spongebob,spongebob,2,cargo,true,47,1638259201608
    32  temperature,spongebob,spongebob,3,cargo,true,56,1638259201608
    33
    34  #CSV_ANNOTATED: /home/conan/soft/rust/metynka/csv/2021_11_30_laptop_memory_float.csv
    35  #
    36  #datatype measurement,tag,tag,tag,tag,tag,double,dateTime:number
    37  m,host,Machine,MemoryId,MemoryCarrier,MemoryValid,MemoryDecimal,time
    38  memory_float,spongebob,spongebob,memory_free,cargo,true,342292,1638259201608
    39  memory_float,spongebob,spongebob,memory_available,cargo,true,2809932,1638259201608
```
    
