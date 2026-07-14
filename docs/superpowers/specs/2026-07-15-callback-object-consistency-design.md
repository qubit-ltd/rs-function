# rs-function 回调对象一致性设计

## 1. 背景

`rs-function` 已完成语义 trait 与具体 Box、Rc、Arc 包装器的职责拆分，组合方法也已从裸闭包扩展 trait 收敛到具体包装器。当前剩余问题主要是不同回调家族之间的能力和行为仍不完全一致：

- Comparator、Tester、StatefulTester 尚未统一使用 `CallbackMetadata`，缺少一致的命名与诊断能力；
- Comparator 和无状态 Tester 的部分组合方法只接受同一种具体包装器，不能直接接受对应语义 trait 的其他实现；
- 不同组合器对名称的保留或清除缺少统一规则；
- 闭包 blanket impl 的物理位置、共享 `FnMut` 的锁或借用作用域，以及 feature 文档仍有职责不清之处；
- Comparator、Tester 等模块仍有主观宣传或与真实调用语义不一致的文档。

本次不处理公开 API 总量问题，不新增公共命名 trait，也不为 Conditional 中间构建器增加独立元数据。

## 2. 目标

1. 为 Comparator、Tester、StatefulTester 的最终 Box、Rc、Arc 包装器补齐统一元数据、命名、Clone 和诊断契约。
2. 为所有已具备命名能力的最终回调包装器增加 `with_name(self, name: &str) -> Self`。
3. 让 Comparator、Tester、StatefulTester 的 Box、Rc、Arc 组合器接受符合所有权约束的语义 trait 实现。
4. 明确定义并统一实现组合后的名称传播规则。
5. 把闭包 blanket impl 移回对应语义 trait 模块，并显式绑定共享 `FnMut` 调用期间的锁 guard 或 `RefCell` borrow。
6. 保持现有 feature 名称和依赖边界，只修正文档与自动化契约。
7. 清理与实际行为不符、过度宣传或容易误解的文档。

## 3. 非目标

- 不收缩或重组当前公开 API 家族。
- 不新增 `NamedCallback`、`CallbackMetadata` 等公共 trait 或类型。
- 不为 `BoxConditional*`、`RcConditional*`、`ArcConditional*` 增加独立名称字段。
- 不引入通用高阶泛型回调容器。
- 不修改现有 Box、Rc、Arc 的底层所有权和动态分派模型。
- 不重命名 `rc`、`once`、`stateful` 或 `full` feature。

## 4. 元数据和命名 API

Comparator、Tester、StatefulTester 的最终包装器统一增加 crate-private `CallbackMetadata` 字段。适用类型包括：

- `BoxComparator<T>`、`RcComparator<T>`、`ArcComparator<T>`；
- `BoxTester`、`RcTester`、`ArcTester`；
- `BoxStatefulTester`、`RcStatefulTester`、`ArcStatefulTester`。

这些类型与现有可命名包装器保持相同的公开方法：

```rust
pub fn new_with_name<F>(name: &str, source: F) -> Self;
pub fn new_with_optional_name<F>(source: F, name: Option<String>) -> Self;
pub fn name(&self) -> Option<&str>;
pub fn set_name(&mut self, name: &str);
pub fn clear_name(&mut self);
pub fn with_name(mut self, name: &str) -> Self;
```

`with_name` 也增加到所有当前已经提供 `name`、`set_name` 和 `clear_name` 的最终回调包装器。它只设置当前包装器的名称并返回 `self`，用于在组合结束后链式命名。

共享包装器 clone 时继续共享 `Arc<str>` 名称字节；对任一 clone 调用 `set_name`、`clear_name` 或 `with_name` 只替换该 clone 的元数据句柄，不修改其他 clone 的可见名称。

Comparator、Tester 和 StatefulTester 同时补齐与其他家族一致的 `Debug`、`Display`：

- `Debug` 显示具体包装器类型和可选名称，不尝试格式化闭包；
- `Display` 在有名称时显示 `Type(name)`，无名称时显示类型名；
- 现有类型的格式保持不变。

## 5. 组合器参数契约

组合从具体包装器开始，但后续参数接受对应语义 trait 的实现，而不是只接受同一种包装器。

### 5.1 Comparator

```rust
impl<T> BoxComparator<T> {
    pub fn then_comparing<C>(self, other: C) -> Self
    where
        T: 'static,
        C: Comparator<T> + 'static;
}

impl<T> RcComparator<T> {
    pub fn then_comparing<C>(&self, other: C) -> Self
    where
        T: 'static,
        C: Comparator<T> + 'static;
}

impl<T> ArcComparator<T> {
    pub fn then_comparing<C>(&self, other: C) -> Self
    where
        T: 'static,
        C: Comparator<T> + Send + Sync + 'static;
}
```

### 5.2 Tester

`and`、`or`、`nand`、`xor`、`nor` 使用相同所有权规则：

- Box 消费 `self`，按值接受 `Tester + 'static`；
- Rc 借用 `self`，按值接受 `Tester + 'static`；
- Arc 借用 `self`，按值接受 `Tester + Send + Sync + 'static`。

### 5.3 StatefulTester

`and`、`or`、`nand`、`xor`、`nor` 继续接受按值传入的 `StatefulTester` 实现：

- Box：`StatefulTester + 'static`；
- Rc：`StatefulTester + 'static`；
- Arc：`StatefulTester + Send + 'static`。Arc 使用 Mutex 串行执行 `FnMut`，因此不要求回调本身实现 `Sync`。

共享包装器方法借用左侧 `self`，但按值捕获右侧回调。调用方若需要继续使用右侧 Rc 或 Arc 包装器，应显式传入 `right.clone()`。本次允许相应的破坏性签名调整，不新增平行的 `*_with` 兼容方法。

## 6. 名称传播契约

名称表示一个可被日志、诊断和运维识别的最终回调对象。组合器不得以不一致方式隐式继承名称。

### 6.1 保留源名称

以下操作被视为对同一回调身份的单源变换，结果保留左侧源对象名称：

- `map`；
- `map_err`；
- `not`；
- `reversed`。

### 6.2 清除名称

以下操作引入额外行为、分支或独立回调，结果为未命名对象：

- `and_then`；
- `and`、`or`、`nand`、`xor`、`nor`；
- `zip`；
- `filter`；
- `when` 以及后续 `or_else` 生成的最终包装器。

Conditional 中间对象不增加独立元数据。其 `Debug` 和 `Display` 可以继续显示内部组件信息，但最终组合结果必须通过 `with_name` 显式获得新名称：

```rust
let pipeline = BoxCallable::new_with_name("load", load)
    .and_then(validate)
    .with_name("load-and-validate");
```

所有受影响的组合器都增加名称传播契约测试，避免不同 family 或 ownership 产生不同结果。

## 7. Blanket impl 与共享可变调用

闭包到语义 trait 的 blanket impl 必须与 trait 定义位于同一语义模块，不能继续依附于 Arc 包装器文件。feature 关闭某个包装器时，闭包是否实现语义 trait 不得随之变化。

所有 `Arc<Mutex<FnMut>>` 和 `Rc<RefCell<FnMut>>` 调用显式绑定 guard 或 borrow：

```rust
let mut function = self.function.lock();
function(value)
```

```rust
let mut function = self.function.borrow_mut();
function(value)
```

这样不改变锁或借用的生命周期：用户回调执行期间仍全程持有 guard/borrow，现有串行化、重入死锁或 panic、以及 panic 前状态不回滚的契约保持不变。

## 8. Feature 契约

feature 名称和依赖关系保持：

```toml
default = []
rc = []
once = []
stateful = ["dep:parking_lot"]
full = ["rc", "once", "stateful"]
```

文档明确以下边界：

- `stateful` 启用显式 Stateful family，以及依赖 `parking_lot::Mutex` 的 Arc task 包装器；
- `rc` 启用单线程共享包装器，其中 Rc task 使用 `RefCell` 支持其固有的 `FnMut` 调用语义；
- baseline 中的 Box task 保留 `FnMut` 语义，但不引入共享同步依赖；
- feature 不表示 Rust `FnMut` 能力的完整分类，而表示可选 API 和依赖成本边界。

feature contract 测试继续验证 baseline、rc、once、stateful、组合和 full 构建。

## 9. 文档清理

文档只描述类型系统和实现能够保证的行为：

- 删除 “perfect balance”“most flexible and elegant”等主观宣传；
- 不把可重复调用的 Box 包装器描述为 one-time；
- 构造器文档说明其接受语义 trait 实现，闭包只是 blanket impl 提供的一种输入；
- 组合器文档准确描述参数所有权、Send/Sync 要求、名称传播和结果类型；
- Comparator、Tester、StatefulTester 的 README 与 rustdoc 补充命名和诊断能力；
- 中英文 README 保持 feature、示例和行为说明一致。

## 10. 测试策略

实现遵循测试先行，每组行为先观察目标测试因缺少 API 或行为不一致而失败，再写最小实现。

### 10.1 元数据测试

每个新增 family 至少覆盖 Box、Rc、Arc：

- 未命名构造；
- `new_with_name` 和 `new_with_optional_name`；
- `with_name`、`set_name`、`clear_name`；
- clone 保留名称；
- 修改 clone 名称不影响原对象；
- `Debug`、`Display` 的有名和无名格式。

### 10.2 组合器测试

- Comparator 可组合闭包、自定义 Comparator 和相同包装器；
- Tester、StatefulTester 可组合闭包、自定义语义对象和相同包装器；
- Box 消费语义、Rc/Arc 左侧复用语义保持不变；
- Arc 组合参数满足 Send/Sync 或 Send 边界；
- `and`、`or`、`nand`、`nor` 保持短路，`xor` 计算两侧。

### 10.3 名称传播测试

- `map`、`map_err`、`not`、`reversed` 保留名称；
- `and_then`、逻辑组合、`zip`、`filter`、`when/or_else` 清除名称；
- 最终结果可通过 `with_name` 重新命名。

### 10.4 验证

按影响范围运行精确集成测试和 feature contract 测试；最终运行：

```bash
./align-ci.sh
./ci-check.sh
```

完成标准是格式、Clippy、风格、debug/release 构建、全部测试、doctest、文档、feature matrix、package、覆盖率和安全审计全部通过，且工作区只包含本次批准的改动。

## 11. 实施顺序

1. 为新增命名能力、泛型组合器和名称传播契约添加失败测试。
2. 扩展公共命名宏并为现有可命名最终包装器增加 `with_name`。
3. 迁移 Comparator、Tester、StatefulTester 的元数据和诊断实现。
4. 泛化 Comparator、Tester、StatefulTester 组合器参数。
5. 按统一规则修正所有受影响组合器的名称传播。
6. 移动 blanket impl，显式绑定共享可变调用 guard/borrow。
7. 修正 feature 和中英文文档。
8. 运行完整验证并审查 diff。
