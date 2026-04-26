# 代码覆盖率指南

本项目使用 `cargo-llvm-cov` 生成本地和 CI 覆盖率报告。覆盖率脚本用于贡献者检查，
不会随 crate 发布。

## 依赖

先安装覆盖率工具和对应 LLVM 组件：

```bash
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
```

`json` 和 `all` 格式还需要 `jq`，因为脚本会从 JSON 报告中检查逐文件覆盖率阈值。

## 快速开始

日常检查优先使用 `coverage.sh`：

```bash
./coverage.sh              # 生成 HTML 并在浏览器中打开
./coverage.sh text         # 在终端输出文本报告
./coverage.sh lcov         # 生成 LCOV
./coverage.sh json         # 生成 JSON 并检查阈值
./coverage.sh cobertura    # 生成 Cobertura XML
./coverage.sh all          # 只运行一次测试并生成所有报告格式
./coverage.sh all --clean  # 先清理旧覆盖率数据
./coverage.sh help         # 查看所有选项
```

`json` 和 `all` 会对每个源码文件执行当前 CI 阈值：

- 函数覆盖率：`100%`
- 行覆盖率：`> 98%`
- 区域覆盖率：`> 98%`

临时实验时可以通过环境变量覆盖阈值：

```bash
MIN_FUNCTION_COVERAGE=100 MIN_LINE_COVERAGE=98 MIN_REGION_COVERAGE=98 ./coverage.sh json
```

## 报告位置

生成的报告位于 `target/llvm-cov`：

- HTML: `target/llvm-cov/html/index.html`
- LCOV: `target/llvm-cov/lcov.info`
- JSON: `target/llvm-cov/coverage.json`
- Cobertura: `target/llvm-cov/cobertura.xml`
- Text: `target/llvm-cov/coverage.txt`（仅 `all` 生成）

## `all` 的工作方式

`./coverage.sh all` 先用 `cargo llvm-cov --no-report` 运行一次测试并收集覆盖率数据，
再用 `cargo llvm-cov report` 基于同一份数据生成 HTML、LCOV、JSON、Cobertura 和
文本报告。这样可以和 CircleCI 保持一致，并避免重复执行测试。

## 直接使用 `cargo llvm-cov`

直接命令适合临时查看，但不会执行本项目的逐文件阈值检查：

```bash
cargo llvm-cov clean
cargo llvm-cov --html --open
cargo llvm-cov --lcov --output-path target/llvm-cov/lcov.info
cargo llvm-cov --json --output-path target/llvm-cov/coverage.json
cargo llvm-cov --cobertura --output-path target/llvm-cov/cobertura.xml
```

如果只想筛选部分测试，把普通测试过滤参数放在 `--` 后面：

```bash
cargo llvm-cov --html --open --test tester_tests -- test_always_true
```

## 排除规则

`.llvm-cov.toml` 会从报告中排除测试、benchmark 和示例文件：

- `tests/*`
- `benches/*`
- `examples/*`

`coverage.sh` 还会过滤 Cargo registry、rustup 和同级 workspace crate，确保报告只覆盖
当前 crate 的源码文件。

## CI

CircleCI 调用 `./coverage.sh all`，保存 JSON、LCOV 和文本报告，并在配置了
`COVERALLS_REPO_TOKEN` 时把 LCOV 上传到 Coveralls。`./ci-check.sh` 也会通过
`./coverage.sh json` 执行相同的阈值检查。

## 常见问题

如果缺少 `cargo-llvm-cov`，请安装它，并给当前 toolchain 添加 `llvm-tools-preview`。

如果 `json` 或 `all` 在阈值检查前失败，请安装 `jq`。

如果覆盖率数据看起来过期，可以运行：

```bash
./coverage.sh json --clean
```

## 参考资料

- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [LLVM Coverage Mapping](https://llvm.org/docs/CoverageMappingFormat.html)
