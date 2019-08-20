# Store

`Store` 是存储管理服务, `Directory` 收到请求之后, 找到满足要求的 `Store` 之后, 将 `Store` 地址返回给 `Client`. 返回的内容为
``` json
[
    {
        "master": {
            "location": "10.31.21.55",
            "volume_id": 10,
            "mode": "master"
        },
        "replicas": [
            {
                "location": "10.31.22.74",
                "volume_id": 63,
                "mode": "replica"
            }
        ]
    }
]
```
