# PodHelper

> (第一个的练手`rust`项目)
> 
> `Cocoapods`的一些帮助工具

### 编译

> 前提已安装`rust`

```shell
cargo install # 编译、安装
# or
cargo build --release # 仅编译
```

### 功能

- `pod_helper check [FLAGS] [OPTIONS] <FILE_PATH>`
	- 检查`podfile.lock`中安装的库有哪些有最新版本(非本地`pod repo`数据对比, 访问相应的`pod index`查找)

结果以表格输出(检查时还会有个进度条)

```
$ pod_helper check Podfile.lock
request done.                                                                                                                                                                                  +-----------------+-------+-----------------------------+
| name            | ver   | new ver                     |
+=======================================================+
| UMCCommon       | 7.1.3 | not found or request failed |
|-----------------+-------+-----------------------------|
| SimpleStoreData | 0.1.2 | not found or request failed |
|-----------------+-------+-----------------------------|
| swiftScan       | 1.2.0 | not found or request failed |
|-----------------+-------+-----------------------------|
| MBProgressHUD   | 1.1.0 | 1.2.0                       |
+-----------------+-------+-----------------------------+
```

### 开源协议

MIT