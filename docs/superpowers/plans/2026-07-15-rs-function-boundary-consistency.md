# rs-function Boundary Consistency Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use executing-plans to implement this plan task-by-task. Use subagents only when the user explicitly requests delegation.

**Goal:** 放宽 Stateful Arc 回调的真实并发边界，统一任务名称传播，简化 StatefulSupplier memoize，并让宏文档与实现保持一致。

**Architecture:** 保持现有公开 callback 类型和 feature 不变。共享组合宏分别表达 predicate 与 chained callback 的 bounds；Stateful Arc 的用户回调由 Mutex 串行化，因此只要求 Send。名称和 memoize 修改沿用现有 CallbackMetadata 与 wrapper-first 模型。

**Tech Stack:** Rust 1.94、edition 2024、macro_rules!、parking_lot::Mutex、集成测试、rustdoc、Clippy、项目固定 nightly rustfmt。

## Global Constraints

- 不修改 default、rc、once、stateful、full feature。
- 不新增依赖、公开 callback 类型或公开方法。
- 所有行为修改先观察测试以预期原因失败，再写最小实现。
- 测试只放在 tests/，不在源码内新增 cfg(test) 模块。
- 只格式化本次涉及文件，不顺带清理无关代码。
- 未经用户明确授权，不执行 git add、git commit 或 git push。
- 每个任务结束时运行 git diff --check 和精确测试。

---

### Task 1: 放宽 Stateful Arc 基础组合器的 chained callback bounds

**Files:**

- Modify: tests/relaxed_bounds_tests.rs
- Modify: src/consumers/macros/shared_consumer_methods.rs
- Modify: src/functions/macros/shared_function_methods.rs
- Modify: src/mutators/macros/shared_mutator_methods.rs
- Modify: src/transformers/macros/shared_transformer_methods.rs
- Modify: Appendix A.1 中的全部调用文件

**Interfaces:**

- Consumes: 现有 when、and_then 和 Box/Rc/Arc wrapper constructors。
- Produces: 公开 API 不变；Stateful Arc predicate 保持 Send + Sync，chained callback 只需 Send。

- [ ] **Step 1: 添加 Send + !Sync 的基础组合失败测试**

在 tests/relaxed_bounds_tests.rs 中增加 Cell 捕获测试，覆盖：

- ArcStatefulConsumer 和 ArcStatefulBiConsumer 的 and_then；
- ArcStatefulFunction 和 ArcStatefulMutatingFunction 的 and_then；
- ArcStatefulMutator 的 and_then；
- ArcStatefulTransformer 和 ArcStatefulBiTransformer 的 and_then。

测试采用真实组合并断言结果。例如：

~~~rust
let state = std::cell::Cell::new(0);
let mut function = ArcStatefulFunction::new(|value: &i32| value + 1)
    .and_then(move |value: &i32| {
        state.set(*value);
        state.get() * 2
    });
assert_eq!(function.apply(&2), 6);

let state = std::cell::Cell::new(0);
let mut transformer = ArcStatefulTransformer::new(|value: i32| value + 1)
    .and_then(move |value: i32| {
        state.set(value);
        state.get() * 2
    });
assert_eq!(transformer.apply(2), 6);
~~~

Consumer 测试额外捕获 Arc<AtomicI32> 以断言后续 callback 确实执行；BiConsumer、Mutator、MutatingFunction 和 BiTransformer 使用相同的 Cell 模式及各自语义 trait。

- [ ] **Step 2: 运行测试并确认 RED**

Run:

~~~bash
cargo test --all-features --test relaxed_bounds_tests
~~~

Expected: E0277，错误明确指出 Cell<i32> 不实现 Sync，且来源是四个 shared-method 宏中的 and_then bounds。

- [ ] **Step 3: 在四个共享宏中拆分 bounds**

宏调用统一采用：

~~~rust
predicate_bounds = ($($predicate_bounds:tt)+),
chained_bounds = ($($chained_bounds:tt)+)
~~~

生成约束分别使用 predicate_bounds 和 chained_bounds。调用策略：

- Arc stateless：两者均为 Send + Sync + 'static；
- Arc stateful：predicate 为 Send + Sync + 'static，chained 为 Send + 'static；
- Rc stateless/stateful：两者均为 'static。

更新 Appendix A.1 的所有调用点，不能把 chained bounds 再用于 predicate。

- [ ] **Step 4: 运行精确测试并确认 GREEN**

Run:

~~~bash
cargo test --all-features --test relaxed_bounds_tests
cargo test --all-features --test relaxed_macro_bounds_tests
~~~

Expected: 两个测试目标通过，新增组合均实际执行。

- [ ] **Step 5: 审查 Task 1 diff**

Run:

~~~bash
git --no-pager diff --check
git --no-pager diff -- tests/relaxed_bounds_tests.rs src/consumers src/functions src/mutators src/transformers
~~~

Expected: 只有 bounds 参数拆分、调用点适配和测试；无公开方法改名。

---

### Task 2: 放宽 Conditional、Supplier 和 constant 的 Stateful Arc bounds

**Files:**

- Modify: tests/relaxed_bounds_tests.rs
- Modify: src/consumers/macros/shared_conditional_consumer.rs
- Modify: src/functions/macros/shared_conditional_function.rs
- Modify: src/mutators/macros/shared_conditional_mutator.rs
- Modify: src/transformers/macros/shared_conditional_transformer.rs
- Modify: Appendix A.2 中的全部调用文件
- Modify: src/suppliers/macros/shared_supplier_methods.rs
- Modify: src/suppliers/supplier/arc_supplier.rs
- Modify: src/suppliers/stateful_supplier/arc_stateful_supplier.rs
- Modify: src/functions/stateful_function/arc_stateful_function.rs

**Interfaces:**

- Consumes: Task 1 的独立 bounds 模型。
- Produces: Stateful Arc Conditional、Supplier 和 constant 接受 Send + !Sync 回调或值。

- [ ] **Step 1: 添加 Conditional、Supplier 和 constant 的失败测试**

在 tests/relaxed_bounds_tests.rs 增加：

~~~rust
let state = std::cell::Cell::new(0);
let mut conditional = ArcStatefulFunction::new(|value: &i32| *value)
    .when(|value: &i32| *value > 0)
    .or_else(move |value: &i32| {
        state.set(*value);
        state.get()
    });
assert_eq!(conditional.apply(&-2), -2);

let map_state = std::cell::Cell::new(0);
let mut mapped = ArcStatefulSupplier::new(|| 2).map(move |value| {
    map_state.set(value);
    map_state.get() * 2
});
assert_eq!(mapped.get(), 4);

let filter_state = std::cell::Cell::new(0);
let mut filtered = ArcStatefulSupplier::new(|| 2).filter(move |value: &i32| {
    filter_state.set(*value);
    filter_state.get() % 2 == 0
});
assert_eq!(filtered.get(), Some(2));

let zip_state = std::cell::Cell::new(0);
let mut zipped = ArcStatefulSupplier::new(|| 2).zip(move || {
    zip_state.set(zip_state.get() + 1);
    zip_state.get()
});
assert_eq!(zipped.get(), (2, 1));
~~~

另定义 Clone 的 SendNonSyncValue(Cell<i32>)，验证 ArcStatefulFunction::constant；同时覆盖 Stateful Arc Consumer/BiConsumer、Mutator、Transformer/BiTransformer Conditional 的 or_else，以编译两个泛型宏分支。

- [ ] **Step 2: 运行测试并确认 RED**

Run:

~~~bash
cargo test --all-features --test relaxed_bounds_tests
~~~

Expected: E0277，分别来自 Conditional callback、Supplier callback/other supplier 或 constant value 缺少 Sync。

- [ ] **Step 3: 修改 Conditional 和 Supplier 宏能力参数**

Conditional 宏使用：

~~~rust
callback_bounds = ($($callback_bounds:tt)+)
~~~

该 bounds 只约束 and_then/or_else 参数。删除未使用的 predicate_conversion、into_arc 和 into_rc 宏参数；三泛型 Function arm改为显式 callback_bounds 的通用 arm。

调用规则：

- Arc stateless：Send + Sync + 'static；
- Arc stateful：Send + 'static；
- Rc：'static。

Supplier 的 Arc 分支同样接收 callback_bounds：

~~~rust
impl_shared_supplier_methods!(
    ArcSupplier<T>,
    Supplier,
    callback_bounds = (Send + Sync + 'static)
);

impl_shared_supplier_methods!(
    ArcStatefulSupplier<T>,
    StatefulSupplier,
    callback_bounds = (Send + 'static)
);
~~~

map、filter、zip 分别使用该 bounds。Rc 分支保持 'static。ArcStatefulFunction::constant 的返回值约束改为 Clone + Send + 'static。

- [ ] **Step 4: 运行精确测试并确认 GREEN**

Run:

~~~bash
cargo test --all-features --test relaxed_bounds_tests
cargo test --all-features --test relaxed_macro_bounds_tests
~~~

Expected: 全部通过，无状态 Arc 仍保持 Sync 要求。

- [ ] **Step 5: 审查 Task 2 diff**

Run:

~~~bash
git --no-pager diff --check
git --no-pager diff -- tests/relaxed_bounds_tests.rs src/consumers src/functions src/mutators src/transformers src/suppliers
~~~

Expected: Sync 只从 Mutex 保护的 Stateful Arc callback 边界移除。

---

### Task 3: 统一 Runnable 到 Callable 的名称传播

**Files:**

- Modify: tests/callback_wrapper_contract_tests.rs
- Modify: tests/tasks/runnable_tests.rs
- Modify: tests/tasks/runnable_once_tests.rs
- Modify: tests/tasks/runnable_once/local_box_runnable_once_tests.rs
- Modify: tests/tasks/runnable_with_tests.rs
- Modify: src/tasks/runnable/box_runnable.rs
- Modify: src/tasks/runnable_once/box_runnable_once.rs
- Modify: src/tasks/runnable_once/local_box_runnable_once.rs
- Modify: src/tasks/runnable_with/box_runnable_with.rs

**Interfaces:**

- Consumes: “单源变换保留、多源组合清除”名称契约。
- Produces: 四个跨类型 sequencing 方法返回未命名 callable；执行和错误语义不变。

- [ ] **Step 1: 写目标名称契约测试**

把三处现有 Some("prepare") 断言改为 None；为 LocalBoxRunnableOnce 增加：

~~~rust
#[test]
fn test_local_box_runnable_once_then_callable_clears_name() {
    let task = LocalBoxRunnableOnce::new_with_name(
        "prepare",
        || Ok::<(), io::Error>(()),
    );
    let chained = task.then_callable(|| Ok::<i32, io::Error>(42));

    assert_eq!(chained.name(), None);
    assert_eq!(chained.call().expect("callable should succeed"), 42);
}
~~~

在 test_task_name_propagation_contract 中增加 BoxRunnable、BoxRunnableOnce、LocalBoxRunnableOnce 和 BoxRunnableWith 四个统一断言。

- [ ] **Step 2: 运行测试并确认 RED**

Run:

~~~bash
cargo test --all-features --test callback_wrapper_contract_tests test_task_name_propagation_contract
cargo test --all-features --test mod then_callable
~~~

Expected: 名称断言得到 Some("prepare") 而非 None；执行结果仍成功。

- [ ] **Step 3: 清除四个 sequencing 结果的 metadata**

四个方法删除 metadata 提取和 new_with_metadata，改用各结果类型的普通 new。闭包主体保持：

~~~rust
BoxCallable::new(move || {
    function()?;
    callable.call()
})
~~~

Once、Local Once、With 使用对应 new 和原调用 trait。rustdoc 增加“组合两个独立 callback，结果保持未命名”。

- [ ] **Step 4: 运行精确测试并确认 GREEN**

Run:

~~~bash
cargo test --all-features --test callback_wrapper_contract_tests
cargo test --all-features --test mod then_callable
~~~

Expected: 名称、成功顺序和错误短路测试全部通过。

- [ ] **Step 5: 审查 Task 3 diff**

Run:

~~~bash
git --no-pager diff --check
git --no-pager diff -- src/tasks tests/tasks tests/callback_wrapper_contract_tests.rs
~~~

Expected: 公开签名、执行顺序和错误类型未变化。

---

### Task 4: 简化 StatefulSupplier memoize 并保留 metadata

**Files:**

- Modify: tests/callback_wrapper_contract_tests.rs
- Modify: tests/suppliers/stateful_supplier_tests.rs
- Modify: src/suppliers/stateful_supplier/box_stateful_supplier.rs
- Modify: src/suppliers/stateful_supplier/rc_stateful_supplier.rs
- Modify: src/suppliers/stateful_supplier/arc_stateful_supplier.rs

**Interfaces:**

- Consumes: crate-private new_with_metadata 和现有 Clone 缓存契约。
- Produces: 单层 Option<T> 缓存、相同 once-only 行为、保留源 metadata。

- [ ] **Step 1: 添加名称和调用次数失败测试**

在 supplier 名称契约测试中覆盖三种 owner：

~~~rust
let mut boxed =
    BoxStatefulSupplier::new_with_name("source", || 1).memoize();
assert_eq!(boxed.name(), Some("source"));
assert_eq!(boxed.get(), 1);

let rc_source = RcStatefulSupplier::new_with_name("source", || 1);
let mut rc_memoized = rc_source.memoize();
assert_eq!(rc_memoized.name(), Some("source"));

let arc_source = ArcStatefulSupplier::new_with_name("source", || 1);
let mut arc_memoized = arc_source.memoize();
assert_eq!(arc_memoized.name(), Some("source"));
~~~

Box memoize 的计数器改为外部可观察的 Rc<Cell<usize>>，断言三次 get 后计数为 1。Rc/Arc 现有测试保留计数断言，并增加源 wrapper 在 memoize 后仍可调用的断言。

- [ ] **Step 2: 运行测试并确认 RED**

Run:

~~~bash
cargo test --all-features --test callback_wrapper_contract_tests test_supplier_name_propagation_contract
cargo test --all-features --test mod test_memoize
~~~

Expected: metadata 断言失败；现有缓存次数测试通过。

- [ ] **Step 3: 直接捕获缓存并保留 metadata**

Box 移动 metadata 和 function：

~~~rust
let metadata = self.metadata;
let mut function = self.function;
let mut cache: Option<T> = None;
BoxStatefulSupplier::new_with_metadata(
    move || {
        if let Some(ref cached) = cache {
            cached.clone()
        } else {
            let value = function();
            cache = Some(value.clone());
            value
        }
    },
    metadata,
)
~~~

Rc/Arc clone源 function 和 metadata，直接捕获 Option<T>。Arc 首次计算调用 self_fn.lock()()，Rc 使用 self_fn.borrow_mut()()。删除内部 cache 专用的 Arc<Mutex<Option<T>>> 和 Rc<RefCell<Option<T>>>。更新 memoize rustdoc，说明缓存位于返回 closure 的外层锁或 borrow 内。

- [ ] **Step 4: 运行精确测试并确认 GREEN**

Run:

~~~bash
cargo test --all-features --test callback_wrapper_contract_tests
cargo test --all-features --test mod test_memoize
~~~

Expected: 三种 owner 均保留名称、只计算一次，Rc/Arc 源 wrapper 仍可用。

- [ ] **Step 5: 审查 Task 4 diff**

Run:

~~~bash
git --no-pager diff --check
git --no-pager diff -- src/suppliers tests/suppliers tests/callback_wrapper_contract_tests.rs
~~~

Expected: 没有新增同步原语、依赖或公开 API。

---

### Task 5: 收敛宏文档和 blanket impl 位置

**Files:**

- Modify: src/consumers/macros/shared_consumer_methods.rs
- Modify: src/consumers/macros/shared_conditional_consumer.rs
- Modify: src/functions/macros/shared_function_methods.rs
- Modify: src/functions/macros/shared_conditional_function.rs
- Modify: src/mutators/macros/shared_mutator_methods.rs
- Modify: src/mutators/macros/shared_conditional_mutator.rs
- Modify: src/transformers/macros/shared_transformer_methods.rs
- Modify: src/transformers/macros/shared_conditional_transformer.rs
- Modify: src/suppliers/macros/shared_supplier_methods.rs
- Modify: src/transformers/stateful_bi_transformer/arc_stateful_bi_transformer.rs
- Modify: src/predicates/stateful_predicate.rs
- Modify: src/predicates/stateful_predicate/arc_stateful_predicate.rs

**Interfaces:**

- Consumes: Tasks 1–4 的绿色行为测试。
- Produces: 与真实宏参数一致的能力表；blanket impl 与 trait 同模块；运行时行为不变。

- [ ] **Step 1: 记录重构前 GREEN 基线**

Run:

~~~bash
cargo test --all-features --test relaxed_bounds_tests
cargo test --all-features --test callback_wrapper_contract_tests
~~~

Expected: 全部通过。

- [ ] **Step 2: 更新宏文档和锁模型**

基础 shared-method 宏列出 predicate_bounds 与 chained_bounds；Conditional 宏列出 callback_bounds；Supplier 文档列出 Rc、Arc stateless、Arc stateful 三种能力。删除已移除参数示例：predicate_conversion、into_arc、into_rc。

把 arc_stateful_bi_transformer.rs 的内部标题改为：

~~~rust
// ArcStatefulBiTransformer - Arc<Mutex<dyn FnMut(T, U) -> R + Send>>
~~~

保留“wrapper 实现 Send + Sync”的正确说明，同时明确 callback 自身只需 Send。

- [ ] **Step 3: 移动 StatefulPredicate blanket impl**

从 arc_stateful_predicate.rs 删除闭包 blanket impl，将完全相同的实现和文档放到 stateful_predicate.rs 的 trait 定义之后。不得修改泛型 bounds 或增加 feature gate。

- [ ] **Step 4: 验证重构保持 GREEN**

Run:

~~~bash
rg -n '\$predicate_conversion|into_arc|into_rc' src/consumers/macros/shared_consumer_methods.rs src/consumers/macros/shared_conditional_consumer.rs src/functions/macros/shared_function_methods.rs src/functions/macros/shared_conditional_function.rs src/mutators/macros/shared_mutator_methods.rs src/mutators/macros/shared_conditional_mutator.rs src/transformers/macros/shared_transformer_methods.rs src/transformers/macros/shared_conditional_transformer.rs
cargo test --all-features --test relaxed_bounds_tests
cargo test --all-features --test callback_wrapper_contract_tests
RUSTDOCFLAGS='-D warnings' cargo doc --all-features --no-deps
~~~

Expected: rg 无匹配；测试和 rustdoc 通过。

- [ ] **Step 5: 审查 Task 5 diff**

Run:

~~~bash
git --no-pager diff --check
git --no-pager diff -- src/predicates src/consumers/macros src/functions/macros src/mutators/macros src/transformers/macros src/suppliers/macros
~~~

Expected: blanket impl 只移动一次，文档与真实 bounds 一致。

---

### Task 6: 完整验证与交付审查

**Files:**

- Verify: 全部修改文件
- Verify: docs/superpowers/specs/2026-07-15-rs-function-boundary-consistency-design.md
- Verify: docs/superpowers/plans/2026-07-15-rs-function-boundary-consistency.md

**Interfaces:**

- Consumes: Tasks 1–5 的全部修改。
- Produces: 格式、lint、测试、文档和 feature matrix 的新鲜证据。

- [ ] **Step 1: 应用项目固定格式并检查 diff**

Run:

~~~bash
cargo +nightly-2026-06-05 fmt --all -- --config-path .rs-ci/rustfmt.toml
cargo +nightly-2026-06-05 fmt --all -- --check --config-path .rs-ci/rustfmt.toml
git --no-pager diff --check
~~~

Expected: 全部通过；确认没有无关文件被格式化。

- [ ] **Step 2: 运行 lint、文档和完整测试**

Run:

~~~bash
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --all-features --no-deps
cargo test --all-features
~~~

Expected: 全部以 0 退出，无 warning 或测试失败。

- [ ] **Step 3: 运行 feature matrix**

Run:

~~~bash
cargo check --all-targets --no-default-features
cargo check --all-targets --no-default-features --features rc
cargo check --all-targets --no-default-features --features once
cargo check --all-targets --no-default-features --features stateful
cargo check --all-targets --no-default-features --features rc,once
cargo check --all-targets --no-default-features --features rc,stateful
cargo check --all-targets --no-default-features --features once,stateful
cargo check --all-targets --no-default-features --features full
~~~

Expected: 八种配置全部通过。

- [ ] **Step 4: 运行项目 CI 并审查最终工作区**

Run:

~~~bash
./ci-check.sh
git status --short
git --no-pager diff --stat
git --no-pager diff
~~~

Expected: CI 通过；工作区只包含本计划列出的源码、测试、spec 和 plan。不要执行 add、commit 或 push。

## Appendix A: Macro Invocation Files

### A.1 Base shared-method invocations

Consumer:

- src/consumers/consumer/arc_consumer.rs
- src/consumers/consumer/rc_consumer.rs
- src/consumers/bi_consumer/arc_bi_consumer.rs
- src/consumers/bi_consumer/rc_bi_consumer.rs
- src/consumers/stateful_consumer/arc_stateful_consumer.rs
- src/consumers/stateful_consumer/rc_stateful_consumer.rs
- src/consumers/stateful_bi_consumer/arc_stateful_bi_consumer.rs
- src/consumers/stateful_bi_consumer/rc_stateful_bi_consumer.rs

Function:

- src/functions/function/arc_function.rs
- src/functions/function/rc_function.rs
- src/functions/bi_function/arc_bi_function.rs
- src/functions/bi_function/rc_bi_function.rs
- src/functions/mutating_function/arc_mutating_function.rs
- src/functions/mutating_function/rc_mutating_function.rs
- src/functions/bi_mutating_function/arc_bi_mutating_function.rs
- src/functions/bi_mutating_function/rc_bi_mutating_function.rs
- src/functions/stateful_function/arc_stateful_function.rs
- src/functions/stateful_function/rc_stateful_function.rs
- src/functions/stateful_mutating_function/arc_stateful_mutating_function.rs
- src/functions/stateful_mutating_function/rc_stateful_mutating_function.rs

Mutator:

- src/mutators/mutator/arc_mutator.rs
- src/mutators/mutator/rc_mutator.rs
- src/mutators/stateful_mutator/arc_stateful_mutator.rs
- src/mutators/stateful_mutator/rc_stateful_mutator.rs

Transformer:

- src/transformers/transformer/arc_transformer.rs
- src/transformers/transformer/rc_transformer.rs
- src/transformers/bi_transformer/arc_bi_transformer.rs
- src/transformers/bi_transformer/rc_bi_transformer.rs
- src/transformers/stateful_transformer/arc_stateful_transformer.rs
- src/transformers/stateful_transformer/rc_stateful_transformer.rs
- src/transformers/stateful_bi_transformer/arc_stateful_bi_transformer.rs
- src/transformers/stateful_bi_transformer/rc_stateful_bi_transformer.rs

### A.2 Conditional shared-method invocations

Consumer:

- src/consumers/consumer/arc_conditional_consumer.rs
- src/consumers/consumer/rc_conditional_consumer.rs
- src/consumers/bi_consumer/arc_conditional_bi_consumer.rs
- src/consumers/bi_consumer/rc_conditional_bi_consumer.rs
- src/consumers/stateful_consumer/arc_conditional_stateful_consumer.rs
- src/consumers/stateful_consumer/rc_conditional_stateful_consumer.rs
- src/consumers/stateful_bi_consumer/arc_conditional_stateful_bi_consumer.rs
- src/consumers/stateful_bi_consumer/rc_conditional_stateful_bi_consumer.rs

Function:

- src/functions/function/arc_conditional_function.rs
- src/functions/function/rc_conditional_function.rs
- src/functions/bi_function/arc_conditional_bi_function.rs
- src/functions/bi_function/rc_conditional_bi_function.rs
- src/functions/mutating_function/arc_conditional_mutating_function.rs
- src/functions/mutating_function/rc_conditional_mutating_function.rs
- src/functions/bi_mutating_function/arc_conditional_bi_mutating_function.rs
- src/functions/bi_mutating_function/rc_conditional_bi_mutating_function.rs
- src/functions/stateful_function/arc_conditional_stateful_function.rs
- src/functions/stateful_function/rc_conditional_stateful_function.rs
- src/functions/stateful_mutating_function/arc_conditional_stateful_mutating_function.rs
- src/functions/stateful_mutating_function/rc_conditional_stateful_mutating_function.rs

Mutator:

- src/mutators/mutator/arc_conditional_mutator.rs
- src/mutators/mutator/rc_conditional_mutator.rs
- src/mutators/stateful_mutator/arc_conditional_stateful_mutator.rs
- src/mutators/stateful_mutator/rc_conditional_stateful_mutator.rs

Transformer:

- src/transformers/transformer/arc_conditional_transformer.rs
- src/transformers/transformer/rc_conditional_transformer.rs
- src/transformers/bi_transformer/arc_conditional_bi_transformer.rs
- src/transformers/bi_transformer/rc_conditional_bi_transformer.rs
- src/transformers/stateful_transformer/arc_conditional_stateful_transformer.rs
- src/transformers/stateful_transformer/rc_conditional_stateful_transformer.rs
- src/transformers/stateful_bi_transformer/arc_conditional_stateful_bi_transformer.rs
- src/transformers/stateful_bi_transformer/rc_conditional_stateful_bi_transformer.rs

