# jsx_template 目录函数调用关系问题与修复建议

## 1. 循环依赖问题
- **问题**：`transform_expression_with_tracker` 和 `transform_arrow_function` 互相导入，形成循环依赖，可能导致编译问题。
- **解决方式**：将 `transform_arrow_function` 移动到 `transform_expression.rs` 内部，或创建一个共同的上级模块协调两者。

---

## 2. 导入路径不一致
- **问题**：部分文件使用 `crate::transformations::jsx_template::xxx`，部分使用 `crate::jsx_template::xxx`，路径风格不统一。
- **解决方式**：统一所有导入为 `crate::jsx_template::xxx` 格式，避免混用。

---

## 3. Helper函数依赖混乱
- **问题**：`transform_statement.rs` 直接依赖 `crate::helpers::jsx_template::*`，但实际应避免依赖 helper 层。
- **解决方式**：将必要 helper 逻辑内聚到主转换流程，或通过接口隔离，减少直接依赖。

---

## 4. 缺少错误处理逻辑
- **问题**：大部分函数返回 `Option<T>`，但调用链缺少统一的错误处理，`None` 情况未做兜底。
- **解决方式**：在主流程增加错误兜底和日志，保证转换失败时有明确提示。

---

## 5. Fragment处理不完整
- **问题**：`transform_expression_with_tracker` 处理 Fragment 时未覆盖 `JsxText` 节点。
- **解决方式**：在 Fragment 处理分支补充 `AnyJsxChild::JsxText` 的处理逻辑。

---

## 6. 自定义组件递归处理有缺陷
- **问题**：自定义组件递归后直接返回 JSX，未真正转换为 SolidJS 调用，且属性未处理。
- **解决方式**：自定义组件应走完整转换流程，并处理属性，生成组件调用表达式。

---

## 7. HelperUsageTracker 状态不完整
- **问题**：`transform_arrow_function` 内新建 tracker，未合并回主 tracker，导致 helper 使用统计不准确。
- **解决方式**：实现 tracker 的 merge 方法，递归转换时合并状态。

---

## 8. JSX收集链自递归缺少深度限制
- **问题**：`collect_jsx_from_expression` 存在自递归，理论上可能导致栈溢出。
- **解决方式**：增加递归深度限制或尾递归优化，防止极端情况下栈溢出。
