# Store

`Store` 是存储管理服务, `Directory` 负责为 `Client` 找到满足要求的 `Volume`

``` json
[
    {
        "master": {
            "location": "10.31.21.55", // store service address
            "volume_id": 10,
            "chunk_id": 32,
            "mode": "master"
        },
        "replicas": [
            {
                "location": "10.31.22.74",
                "volume_id": 63,
                "chunk_id": 17,
                "mode": "replica"
            }
        ]
    }
]
```

``` rust
// 代表一个 Store Service 服务节点
struct StoreService {
    // 每一个 Store Service 节点可以有若干各 volume(暂定每一个volume有一个数据文件)
    volumes: Vec<Volume>,
}

struct Volume {
    // 
    data_file:  std::fs::File,
    index_file: std::fs::File,
}
```


## Upload


`Client` 将数据发送给 `Master`, 由 `Master` -> `Replica` -> `Other Replicas` 传递发送。
