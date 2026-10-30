# 代码覆盖率指南

本项目使用 `cargo-llvm-cov` 在本地和 CI 中采集覆盖率并生成报告。项目入口为
`./.infra/bin/coverage.sh`；该脚本用于贡献者检查，不会随 crate 发布。

## 依赖

请安装 `cargo-llvm-cov`、当前 Rust toolchain 对应的 LLVM 工具组件以及 `jq`：

```bash
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
```

采集工具使用 `cargo-llvm-cov` 采集覆盖率并检查阈值。采集成功后，共享报告脚本负责导出报告，
并使用 `jq` 读取配置和生成摘要文件。

## 快速开始

```bash
./.infra/bin/coverage.sh --help
./.infra/bin/coverage.sh
```

帮助输出列出底层 `rs-infra-coverage collect` 命令接受的采集选项，包括 `--output` 和
`--ignore-thresholds`。包装脚本在采集后还会运行报告导出；如果帮助命令成功但没有生成覆盖率数据，
导出阶段仍可能因找不到 `.profraw` 文件而失败。

工具按选中源码的覆盖/总数聚合值检查阈值，不会分别对每个文件执行阈值检查。项目显式配置
`.infra/coverage/coverage.json` 优先；缺失的阈值使用工具默认值：

- 行覆盖率：`> 90%`
- 函数覆盖率：`>= 95%`
- 区域覆盖率：`> 85%`

本项目当前没有显式覆盖率配置，因此使用上述工具默认值。请在
`.infra/coverage/coverage.json` 中配置项目策略，不要使用没有依据的 `MIN_*` 环境变量。

## 采集与报告

包装脚本首先运行 `rs-infra-coverage collect`。默认将原始 LLVM JSON 报告写入
`target/infra/coverage/raw.json`，并检查阈值。可以通过支持的 `--output <PATH>` 选项指定其他原始
报告路径。如果采集或阈值检查失败，包装脚本会在报告导出前停止。

采集成功后，共享报告脚本使用 `cargo llvm-cov report` 导出以下文件：

- JSON：`coverage.json`
- LCOV：`lcov.info`
- 文本：`coverage.txt`
- 聚合 JSON 摘要：`ci-summary.json`
- 覆盖率徽章数据：`coverage-badge.json`
- Markdown 摘要：`coverage-summary.md`
- HTML：`target/llvm-cov/html/index.html`
- Cobertura XML：`target/llvm-cov/cobertura.xml`

HTML 报告会写入磁盘；项目包装脚本不会要求自动打开浏览器。

## 直接使用 `cargo llvm-cov`

直接命令适合临时检查，但不会运行项目采集工具或其配置的阈值检查：

```bash
cargo llvm-cov clean
cargo llvm-cov --html --open
cargo llvm-cov --lcov --output-path target/llvm-cov/lcov.info
cargo llvm-cov --json --output-path target/llvm-cov/coverage.json
cargo llvm-cov --cobertura --output-path target/llvm-cov/cobertura.xml
```

如需筛选测试，请在 `--` 后传入普通测试过滤参数：

```bash
cargo llvm-cov --all-features --html --open --test integration_tests -- testers::tester_2_tests::tests::test_always_true
```

`integration_tests` 需要 `full` feature，`--all-features` 会启用该 feature。`--` 后的参数是
libtest 用于匹配测试路径的过滤串，不是 Cargo 测试目标名。

## 编译契约缓存

编译契约测试将 Cargo 构建缓存固定在
`<checkout>/target/compile-contract/<feature_key>`。source fixture 保存在临时目录中，并在每次调用后
清理；按 feature 区分的 target 目录则是持久、可复用的缓存。为 checkout 设置 `CARGO_TARGET_DIR`
不会改变此辅助缓存的位置。正常测试不会清理共享缓存。如需清理，只针对当前 checkout 内已确认的
编译契约缓存；辅助逻辑不会扫描历史 `/tmp` 残留目录。

## 排除规则

`.llvm-cov.toml` 会从报告中排除测试、benchmark 和示例文件：

- `tests/*`
- `benches/*`
- `examples/*`

采集工具还会过滤 Cargo registry、rustup 和同级 workspace crate，使报告只覆盖本 crate 的源码。

## CI

仓库工作流委托给 `main` 上可复用的 `rs-infra-ci` 工作流。该工作流不传额外采集参数，直接运行
`./.infra/bin/coverage.sh`。采集工具会按项目配置或工具默认值检查阈值；由于报告导出在采集之后，
采集或阈值检查失败会阻止报告导出和 artifact 上传。成功时，CI 将生成的报告发布为
`coverage-reports` artifact，并在任务中添加摘要。可复用工作流和覆盖率工具跟随各自上游 `main`
分支，因此具体行为可能随时间变化。

## 常见问题

如果缺少 `cargo-llvm-cov`，请安装该工具，并为当前 toolchain 添加 `llvm-tools-preview`。还需安装
`jq`：报告导出和摘要阶段会用到它，即使阈值检查本身由采集阶段执行。

如果覆盖率数据看起来过期，可直接使用支持的清理子命令：

```bash
cargo llvm-cov clean
```

该命令清理当前 checkout 的 cargo-llvm-cov 数据；不会删除独立的编译契约缓存。

## 参考资料

- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [LLVM Coverage Mapping](https://llvm.org/docs/CoverageMappingFormat.html)
