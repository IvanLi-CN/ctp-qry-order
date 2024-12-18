# ISSUE for CTP2RS qryOrder

## 复现步骤

首先在外部创建几个报单。

接下来在 linux 下拉取代码，将 `config.toml.example` 复制到 `config.toml` 并配置前置账户和动态库路径。

然后在 Cargo.toml 调整 ctp2rs 的 feature。

最后使用 `cargo run` 执行程序。

理想情况能查询到报单，我遇到的情况是：

  - **simnow 和中信仿真环境** 回调函数触发了和报单数量相同的次数，但 `p_order` 和 `p_rsp_info` 都是 None。
  - **OpenCTP** 正常返回
