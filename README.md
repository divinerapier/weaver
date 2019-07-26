# OnyxiaFS

`OnyxiaFS` 是一个基于 `Facebook Inc.` 在 `2010` 发表的[论文](https://www.usenix.org/legacy/event/osdi10/tech/full_papers/Beaver.pdf) 开发的一个分布式对象文件系统。

## 组件

### OnyxiaGateway

作为所有请求的入口。

### OnyxiaDirectory

1. 维护逻辑卷到物理卷的映射关系；
2. 负责对读写数据进行负载均衡；
1. 维护逻辑卷的空闲空间；
1. 维护图片对应的逻辑卷；

### OnyxiaCache

`Cache` 服务是一个分布式缓存，可以使用数据的 `ID` 对内容进行索引。当缓存命中失败时，负责从后端 `Store` 服务获取数据。

### OnyxiaStore

每一个 `Store` 服务维护多个物理卷。每个物理卷存储了百万级别的数据。具体来说，一个物理卷可以被认为是一个超大的文件(比如100G)，被保存在物理磁盘上的某个位置。可以通过图片所在物理卷及偏移量，大小等信息快速读取数据。

## Architecture

![architecture](docs/images/architecture.png)
