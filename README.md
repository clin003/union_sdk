开放平台 Rust SDK, 集成简单快捷，也可以快速添加本SDK没有集成的接口。

目前已经实现的平台及SDK如下：

 ### [淘宝开发平台](http://open.taobao.com/)
  * 淘口令生成
  * 官方活动转链
 
TODO:

*   ```taobao``` - 淘宝客
*   ```jingdong``` - 京东
*   ```vip``` - 唯品会

### 支持平台

| 平台                                                 | 是否支持 |
|----------------------------------------------------|------|
| Taobao(淘宝客)                                        | ✅    |
| JD(京东联盟)                                           | ❎    |  

## 使用

```toml
[dependencies]

# The core APIs
union_sdk = { version = "0.1.0"}
```

## 文档

## 示例

### 淘宝开放平台

```rust

use dotenv::dotenv;
use std::env;
use union_sdk::taobaosdk::{SDKClient, TaobaoTbkTpwdCreateRequest};
// cargo run --package union_sdk --bin demo
#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_key = env::var("APP_KEY").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();

    let tkl_url=env::var("TKL_URL").unwrap();
    // https://open.taobao.com/api.htm?docId=31127&docType=2
    let arg=TaobaoTbkTpwdCreateRequest::new(tkl_url.as_str());
    let result_instance = SDKClient::new(app_key.as_str(), app_secret.as_str(), "")
        .send_request(arg)
        .await
        .unwrap();
    println!("{}", result_instance);
}


```

## 未来

我们将逐步完善相应的API
1. 首先非常欢迎和感谢对本项目发起 `Pull Request` 的热心小伙伴们。
1. **特别提示：请务必在 `develop` 分支提交 `PR`，`release` 分支目前仅是正式版的代码，即发布正式版本后才会从 `develop` 分支进行合并。**
1. 本项目代码风格为使用2个空格代表一个Tab，因此在提交代码时请注意一下，否则很容易在IDE格式化代码后与原代码产生大量diff，这样会给其他人阅读代码带来极大的困扰。
1. **提交代码前，请检查代码是否已经格式化，并且保证新增加或者修改的方法都有完整的参数说明，而pub方法必须拥有相应的单元测试并通过测试。**

## 开发

To setup the development envrionment run `cargo run`.

## 贡献者

	白菜林

## Getting help

联盟SDK是个人项目。
我希望这个项目会变得越来越可爱。许多实用的其他函数将
将在将来添加。我希望你能积极帮助这个项目成长并提出建议。
我相信未来会越来越好。


#### License

<sup>
Licensed under either of <a href="LICENSE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>