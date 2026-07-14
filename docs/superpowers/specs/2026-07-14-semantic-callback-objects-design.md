# rs-function 语义回调对象重构设计

## 1. 背景

`rs-function` 的核心价值不是替代 Rust 闭包，而是把闭包和自定义对象统一为可存储、可命名、可共享、可组合的语义回调对象。

当前工作区中的实际调用证明了这一定位：

- `rs-batch` 和 `rs-executor` 使用 `Runnable`、`Callable` 作为执行器的领域约束；
- `rs-cas` 允许自定义结构体实现 `Consumer`，并使用 `ArcConsumer` 保存可克隆 hook；
- Box/Rc/Arc 包装器为对象字段、异构任务集合和跨线程共享提供稳定类型；
- concrete wrapper 上的链式组合是裸闭包不直接提供的主要能力。

当前设计的主要问题不是这些抽象本身，而是闭包扩展层、公共导出、内部重复和文档契约：

- `Fn` 同时满足 `FnMut` 和 `FnOnce`，多个 `Fn*Ops` trait 同时进入作用域后，`when`、`and_then` 等方法产生歧义；
- `combinators` feature 把 crate 的核心能力变成了可选能力；
- 大量子模块从父模块间接导入 `Arc`、`Mutex`、trait 和宏，隐藏真实依赖；
- README 和 rustdoc 对纯度、Box 开销、类型保持、转换方法等行为存在不准确描述；
- CI 通过带写权限和继承 secrets 的调用方引用可变的远端 workflow `@main`。

本次允许破坏性变更，不保留旧 API 兼容性。

## 2. 目标

本次重构必须达到以下目标：

1. 保留语义 trait，使闭包和自定义结构体都能满足领域接口约束。
2. 保留 Box/Rc/Arc 包装器，使回调可以方便地保存在对象中并表达所有权、共享和线程语义。
3. 保留具体包装器上的链式组合，并保证组合后的所有权模型清晰可预测。
4. 删除闭包扩展 trait 的方法歧义。
5. 让链式组合成为默认能力，不再由 `combinators` feature 控制。
6. 统一名称元数据、Clone、Debug/Display 和共享可变存储的内部实现。
7. 让每个具体源码文件直接导入自己使用的依赖。
8. 修正文档，使文档只承诺代码和类型系统实际保证的行为。
9. 固定 CI reusable workflow 到不可变版本，并收紧权限和 secrets 传递。
10. 保持现有语义行为、短路规则、状态共享规则和锁重入规则有自动化测试覆盖。

## 3. 非目标

- 不把 crate 收缩成闭包类型别名集合。
- 不删除有独立调用或所有权语义的 `Bi*`、`Stateful*`、`*Once` 家族。
- 不把所有回调签名强行统一为一个高阶泛型容器；Rust 缺少适合该目标的稳定高阶类型抽象，这会降低可读性并增加宏复杂度。
- 不隐藏 `Arc<Mutex<FnMut>>` 和 `Rc<RefCell<FnMut>>` 的锁、重入或共享状态语义。
- 不在本仓库内修改 `rs-batch`、`rs-executor`、`rs-cas` 等兄弟仓库；迁移影响通过文档记录，兄弟仓库在各自变更中适配。

## 4. 总体架构

重构后的公共 API 分成三层。

### 4.1 语义 trait 层

保留 `Predicate`、`Consumer`、`Function`、`Transformer`、`Supplier`、`Mutator`、`Runnable`、`Callable`、`Comparator`、`Tester` 及其确有独立调用语义的 Bi、Stateful、Once 变体。

语义 trait 遵循以下约束：

- 只包含核心调用方法；
- 保持 object-safe；
- 不包含组合方法、构造方法、名称方法或所有权转换方法；
- 为匹配的 `Fn`、`FnMut`、`FnOnce` 闭包提供 blanket implementation；
- 允许领域结构体直接实现 trait；
- trait 本身不强制 `Send`、`Sync` 或 `'static`，并发边界由调用方或具体包装器声明。

示例：

```rust
pub trait Runnable<E> {
    fn run(&mut self) -> Result<(), E>;
}

impl<E, F> Runnable<E> for F
where
    F: FnMut() -> Result<(), E>,
{
    fn run(&mut self) -> Result<(), E> {
        self()
    }
}
```

### 4.2 所有权包装层

保留 Box/Rc/Arc 具体包装器：

- Box 表示单一所有权和动态类型擦除；
- Rc 表示单线程共享所有权；
- Arc 表示线程安全共享所有权；
- Stateful Rc 使用 `Rc<RefCell<_>>` 共享可变状态；
- Stateful Arc 使用 `Arc<parking_lot::Mutex<_>>` 串行化可变回调调用；
- Once 家族只保留有实际意义的单一所有权包装。

构造器接受对应的语义 trait，而不是只接受闭包。这使闭包、自定义语义对象和已有包装器都可作为输入。

名称和诊断信息保留，但内部改为共享元数据：

```rust
#[derive(Clone, Default)]
pub(crate) struct CallbackMetadata {
    name: Option<std::sync::Arc<str>>,
}
```

公开名称 API 继续使用 `Option<&str>` 和 `&str`，不把内部存储类型泄漏到公共接口。Arc/Rc 包装器克隆时不再复制名称字符串。

### 4.3 对象组合层

链式组合只定义在具体包装器上：

```rust
let predicate = BoxPredicate::new(|value: &i32| *value > 0)
    .and(|value: &i32| value % 2 == 0)
    .not();
```

删除全部 `Fn*Ops` trait 及其根级重导出。裸闭包需要链式组合时，必须先通过 `Box*::new`、`Rc*::new` 或 `Arc*::new` 显式选择对象模型。

组合规则如下：

- Box 组合方法通常消费 `self`，返回同家族 Box 包装器；
- Rc/Arc 组合方法借用 `&self`，返回同家族 Rc/Arc 包装器；
- Stateful 组合保持 `FnMut` 语义，并明确记录 clone 后共享状态；
- predicate 的 `and`、`or`、`nand`、`nor` 保持短路规则，`xor` 总是计算两侧；
- conditional builder 保留 staged `when(...).or_else(...)` 能力；
- conditional 类型可以使用私有通用内核减少重复，但公开返回类型仍表达具体所有权和调用语义；
- 组合方法需要类型擦除时可以要求 `'static`，文档必须明确该边界；核心 trait 和直接闭包使用不增加该约束。

## 5. Feature 设计

删除 `combinators` feature，组合 API 默认可用。

重构后的 feature：

```toml
[features]
default = []
rc = []
once = []
stateful = ["dep:parking_lot"]
full = ["rc", "once", "stateful"]
```

规则：

- 默认功能包含核心 trait、Box 包装器、无状态 Arc 包装器和全部相应组合方法；
- `rc` 仅控制 Rc 包装器；
- `once` 仅控制 Once trait、包装器和组合方法；
- `stateful` 控制 Stateful trait、包装器及 `parking_lot`；
- CI feature matrix 覆盖 baseline、单 feature、关键组合和 full。

## 6. 模块和导入规则

顶层继续按语义家族组织，避免把所有对象压入少数超大文件：

```text
src/
├── lib.rs
├── metadata.rs
├── comparator/
├── consumers/
├── functions/
├── mutators/
├── predicates/
├── suppliers/
├── tasks/
├── testers/
└── transformers/
```

具体规则：

- `lib.rs` 和聚合模块只负责模块声明、文档和公开重导出；
- 每个具体 Rust 文件直接导入 `std`、`parking_lot` 和 `crate::...` 中使用的名称；
- 禁止父模块集中导入外部类型后由子模块通过 `use super::{Arc, Mutex, ...}` 获取；
- `use super::...` 只用于导入真正由直接父模块定义或重导出的同家族公共类型；优先使用清晰的 `crate::...` 绝对路径；
- 删除仅用于规避显式导入检查的 `qubit-style: allow explicit-imports`；确有例外时逐文件说明原因；
- 删除 `Fn*Ops` 源码文件及对应测试文件；
- 现有宏按职责拆分，宏不承担隐藏 import、feature 空实现和大段行为文档生成。

## 7. 公共导出策略

crate 根继续导出最常用的语义 trait 和具体包装器，支持：

```rust
use qubit_function::{
    ArcConsumer,
    Consumer,
    Runnable,
};
```

根级导出不得包含会为同一闭包提供同名方法的扩展 trait。删除 `Fn*Ops` 后，`use qubit_function::*` 不应产生组合方法歧义。

深层模块路径继续可用，但文档优先展示精确根级导入，不推荐 glob import。

## 8. Stateful 共享语义

Stateful Rc/Arc 包装器保留，因为它们提供可保存、可克隆的共享 `FnMut` 对象。

必须保持并测试以下契约：

- clone 共享同一个可变回调状态；
- Arc 调用期间持有 `parking_lot::Mutex`；
- 同步重入同一 Arc 状态会死锁；
- Rc 调用期间持有可变 `RefCell` borrow；
- 同步重入同一 Rc 状态会触发 borrow panic；
- panic 不回滚回调在 panic 前完成的状态修改；
- trait 调用继续保持 `&mut self`，与 `FnMut` 语义一致；
- 不新增另一套仅为省略 `mut` 的共享调用 trait。

## 9. 文档契约审计

README、中文 README 和全部 rustdoc 按以下清单审计：

1. `Fn` 不保证纯度、确定性、无副作用或可重复性；这些只能描述为语义建议。
2. Box 包装器包含堆分配和动态分派，不描述为“无开销”。
3. 普通 Box wrapper 可重复调用；只有 Once trait 的核心调用消费对象。
4. 组合是否消费 self、是否借用 self、返回何种所有权包装器必须准确描述。
5. 删除所有已不存在的 `Fn*Ops`、`into_fn`、`to_fn`、`into_mut_fn`、`to_mut_fn` 宣称。
6. “类型保持”限定为具体 wrapper 的适用组合方法，不作全局保证。
7. Stateful Arc/Rc 文档明确共享状态、锁/borrow 持有范围、重入结果和 panic 行为。
8. feature 示例使用重构后的 feature 集合，组合示例不再标记 `combinators`。
9. README 中 Box/Rc/Arc 对比不把“所有 Box 方法都消费 self”作为统一规则。
10. 中英文 README 表格、版本和示例保持一致。

## 10. CI 设计

`.github/workflows/ci.yml` 必须：

- 把 `qubit-ltd/rs-ci/.github/workflows/rust-ci.yml@main` 改为不可变 commit SHA；
- 移除无必要的 `secrets: inherit`；
- 将调用方权限缩减为 reusable workflow 实际需要的最小集合；
- 保留 push、pull_request 和 workflow_dispatch 触发；
- 更新 feature matrix，删除所有 `combinators` 组合，保留 baseline、rc、once、stateful、关键组合和 full；
- 在 CI 或 feature contract 测试中验证 `Fn*Ops` 不再从根或深层模块导出。

固定 SHA 必须来自当前 `.rs-ci` 子模块所指向的提交，避免源码检查与 CI 实际执行版本漂移。

## 11. 测试策略

行为变更和重构遵循测试先行。

### 11.1 编译契约测试

- 先添加一个当前会因多个 `when` 候选而失败的 consumer fixture；
- 重构后验证 `use qubit_function::*` 不再产生同名组合方法歧义；
- 验证裸闭包不再拥有 `when`、`and_then` 等 `Fn*Ops` 方法；
- 验证包装后的 Box/Rc/Arc 对象仍提供对应组合方法；
- 验证关闭 rc、once、stateful 时对应类型不可见；
- 验证组合 API 在无 feature 的 baseline 中可用。

### 11.2 行为测试

- 每个语义家族至少覆盖闭包 blanket impl、自定义结构体实现和具体 wrapper；
- Box/Rc/Arc 组合覆盖正常、条件、短路、错误和边界行为；
- Stateful clone 覆盖共享状态和跨线程串行化；
- CallbackMetadata 覆盖 name、set_name、clear_name、clone、Debug 和 Display；
- 删除只测试已删除 `Fn*Ops` 的测试，保留并迁移其有价值的组合行为断言。

### 11.3 验证命令

开发期间按变更范围运行精确测试。最终必须按顺序运行：

```bash
./align-ci.sh
./ci-check.sh
```

`align-ci.sh` 负责格式化和可自动修复的 lint；`ci-check.sh` 必须完整通过，结果作为完成声明的依据。

## 12. 实施顺序

1. 建立能够复现 `Fn*Ops` 歧义和目标新 API 的编译契约测试。
2. 删除 `Fn*Ops` 根级导出、实现和测试，并把有价值示例迁移到具体 wrapper。
3. 删除 `combinators` feature，使组合 API 默认编译。
4. 引入 `CallbackMetadata`，逐家族迁移名称、Clone、Debug/Display。
5. 整理组合宏和 conditional 内核，保持行为测试通过。
6. 逐家族改为直接导入，删除显式导入例外。
7. 完成英文和中文文档契约审计。
8. 固定 CI workflow、收紧权限并更新 feature matrix。
9. 运行全量格式化、lint、测试、doctest、package 和 CI 对齐检查。

## 13. 成功标准

- 不再存在公开 `Fn*Ops` trait 或对应源文件；
- 裸闭包仍可直接满足语义 trait bound；
- 自定义结构体仍可实现语义 trait；
- Box/Rc/Arc wrapper 可以保存、命名、克隆并链式组合回调；
- baseline 默认提供组合 API；
- `use qubit_function::*` 不再触发闭包组合方法歧义；
- 具体源码文件不通过父模块获取外部依赖；
- README、中文 README 和 rustdoc 与真实代码行为一致；
- CI 使用不可变 reusable workflow 引用和最小权限；
- `./align-ci.sh` 成功完成；
- `./ci-check.sh` 全部通过。
