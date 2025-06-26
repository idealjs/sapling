# Babel Plugin JSX DOM Expressions Migration Tasks

## 状态说明

- TODO: 待实现
- IN_PROGRESS: 开发中
- DONE: 已完成
- PARTIAL: 部分实现

## dom/

### constants.js

| 函数/常量名称  | 行数 | 描述                                       | 迁移目标          | 目标函数        | 状态 |
| -------------- | ---- | ------------------------------------------ | ----------------- | --------------- | ---- |
| InlineElements | 54   | 内联元素列表常量，定义了HTML中所有内联元素 | html_constants.rs | INLINE_ELEMENTS | DONE |
| BlockElements  | 35   | 块级元素列表常量，定义了HTML中所有块级元素 | html_constants.rs | BLOCK_ELEMENTS  | DONE |

### element.js

| 函数/常量名称                | 行数 | 描述                                       | 迁移目标               | 目标函数                      | 状态 |
| ---------------------------- | ---- | ------------------------------------------ | ---------------------- | ----------------------------- | ---- |
| transformAttributes          | 450  | 处理和转换JSX元素的属性                    | element_attributes.rs  | transform_attributes          | TODO |
| setAttr                      | 185  | 设置元素属性，处理不同类型的属性和命名空间 | element_attributes.rs  | set_attr                      | TODO |
| transformChildren            | 125  | 转换和处理JSX子元素                        | element_children.rs    | transform_children            | TODO |
| processSpreads               | 120  | 处理JSX的展开属性                          | spread_attributes.rs   | process_spreads               | TODO |
| transformElement             | 98   | 转换JSX元素为DOM元素                       | dom_element.rs         | transform_element             | TODO |
| detectExpressions            | 58   | 检测JSX中的动态表达式                      | expression_detector.rs | detect_expressions            | TODO |
| createPlaceholder            | 35   | 创建DOM占位符元素                          | placeholder.rs         | create_placeholder            | TODO |
| findLastElement              | 18   | 在子元素列表中查找最后一个元素             | element_utils.rs       | find_last_element             | TODO |
| detectResolvableEventHandler | 15   | 检测事件处理器是否可解析                   | event_handler.rs       | detect_resolvable_handler     | TODO |
| contextToCustomElement       | 15   | 为自定义元素添加上下文支持                 | custom_element.rs      | add_context_to_custom_element | TODO |
| nextChild                    | 3    | 获取下一个子元素节点                       | element_utils.rs       | next_child                    | TODO |
| alwaysClose                  | 21   | 总是需要闭合的HTML元素列表                 | html_constants.rs      | ALWAYS_CLOSE                  | TODO |

### template.js

| 函数/常量名称    | 行数 | 描述                                       | 迁移目标             | 目标函数          | 状态 |
| ---------------- | ---- | ------------------------------------------ | -------------------- | ----------------- | ---- |
| wrapDynamics     | 95   | 封装动态属性的更新逻辑，处理动态样式和类名 | dynamic_wrapper.rs   | wrap_dynamics     | TODO |
| appendTemplates  | 45   | 添加模板到AST，处理SVG和MathML特殊情况     | template_append.rs   | append_templates  | TODO |
| registerTemplate | 42   | 注册模板并处理模板的hydration逻辑          | template_registry.rs | register_template | TODO |
| createTemplate   | 35   | 创建模板，处理模板的声明和动态表达式       | template_creator.rs  | create_template   | TODO |

## shared/

### component.js

| 函数/常量名称              | 行数 | 描述                             | 迁移目标               | 目标函数                     | 状态 |
| -------------------------- | ---- | -------------------------------- | ---------------------- | ---------------------------- | ---- |
| transformComponent         | 220  | 转换JSX组件，处理props和children | component_transform.rs | transform_component          | TODO |
| transformComponentChildren | 85   | 转换组件子元素，处理动态内容     | component_children.rs  | transform_component_children | TODO |
| convertComponentIdentifier | 15   | 转换JSX组件标识符为AST表达式     | component_utils.rs     | convert_component_identifier | TODO |

### fragment.js

| 函数/常量名称             | 行数 | 描述                                         | 迁移目标              | 目标函数                    | 状态 |
| ------------------------- | ---- | -------------------------------------------- | --------------------- | --------------------------- | ---- |
| transformFragmentChildren | 16   | 转换JSX Fragment的子元素，处理文本和节点转换 | fragment_transform.rs | transform_fragment_children | TODO |

### postprocess.js

| 函数/常量名称  | 行数 | 描述                                      | 迁移目标          | 目标函数    | 状态 |
| -------------- | ---- | ----------------------------------------- | ----------------- | ----------- | ---- |
| default export | 45   | 后处理转换后的AST，添加事件代理和模板验证 | post_processor.rs | process_ast | TODO |

### preprocess.js

| 函数/常量名称           | 行数 | 描述                            | 迁移目标         | 目标函数             | 状态 |
| ----------------------- | ---- | ------------------------------- | ---------------- | -------------------- | ---- |
| default export          | 25   | 预处理AST，处理导入源和验证选项 | preprocessor.rs  | preprocess_ast       | TODO |
| JSXValidator.JSXElement | 20   | 验证JSX嵌套的合法性             | jsx_validator.rs | validate_jsx_element | TODO |

### transform.js

| 函数/常量名称           | 行数 | 描述                                          | 迁移目标                            | 目标函数                   | 状态    |
| ----------------------- | ---- | --------------------------------------------- | ----------------------------------- | -------------------------- | ------- |
| transformNode           | 125  | 转换各种类型的JSX节点（元素、文本、表达式等） | node_transform.rs                   | transform_node             | TODO    |
| transformThis           | 65   | 处理this表达式和JSX中的this引用               | this_transform.rs                   | transform_this             | TODO    |
| transformJSX            | 20   | 转换JSX代码，处理顶层节点和模板创建           | oxc_transformer/src/jsx/jsx_impl.rs | transform_jsx              | PARTIAL |
| getCreateTemplate       | 15   | 获取对应的模板创建函数                        | template_factory.rs                 | get_template_creator       | TODO    |
| transformElement        | 15   | 根据不同渲染器转换JSX元素                     | element_transform.rs                | transform_element          | TODO    |
| getTargetFunctionParent | 10   | 获取目标函数的父作用域                        | scope_utils.rs                      | get_target_function_parent | TODO    |

### utils.js

| 函数/常量名称           | 行数 | 描述                      | 迁移目标               | 目标函数               | 状态 |
| ----------------------- | ---- | ------------------------- | ---------------------- | ---------------------- | ---- |
| isDynamic               | 89   | 检查表达式是否为动态的    | expression_utils.rs    | is_dynamic             | TODO |
| escapeHTML              | 44   | HTML转义                  | string_utils.rs        | escape_html            | DONE |
| transformCondition      | 41   | 转换条件表达式            | condition_transform.rs | transform_condition    | TODO |
| wrappedByText           | 21   | 检查是否被文本节点包围    | text_wrap.rs           | is_wrapped_by_text     | DONE |
| convertJSXIdentifier    | 15   | 转换JSX标识符             | jsx_utils.rs           | convert_jsx_identifier | TODO |
| templateEscapes         | 12   | 模板转义字符映射          | escape_utils.rs        | TEMPLATE_ESCAPES       | TODO |
| reservedNameSpaces      | 10   | 保留的命名空间集合        | namespace_constants.rs | RESERVED_NAMESPACES    | TODO |
| jsxElementNameToString  | 7    | 将JSX元素名称转换为字符串 | tag_name.rs            | element_name_to_string | DONE |
| filterChildren          | 7    | 过滤JSX子节点             | children.rs            | filter_children        | DONE |
| checkLength             | 7    | 检查子节点长度            | length_checker.rs      | check_length           | DONE |
| trimWhitespace          | 7    | 处理空白字符              | string_utils.rs        | trim_whitespace        | DONE |
| tagNameToIdentifier     | 6    | 将标签名转换为标识符      | jsx_utils.rs           | tag_name_to_identifier | TODO |
| getStaticExpression     | 6    | 获取静态表达式值          | dynamic.rs             | get_static_expression  | DONE |
| canNativeSpread         | 6    | 检查是否可以使用原生展开  | native_spread.rs       | can_native_spread      | DONE |
| getNumberedId           | 3    | 生成数字ID                | id_gen.rs              | get_numbered_id        | DONE |
| escapeStringForTemplate | 3    | 转义模板字符串            | string_utils.rs        | escape_template_string | DONE |
| getConfig               | 3    | 获取配置信息              | config_utils.rs        | get_config             | TODO |
| getRendererConfig       | 3    | 获取渲染器配置            | config_utils.rs        | get_renderer_config    | TODO |
| getTagName              | 3    | 获取JSX标签名             | tag_name.rs            | get_tag_name           | TODO |
| isComponent             | 3    | 检查是否为组件            | component.rs           | is_component           | DONE |
| toEventName             | 3    | 转换事件名称              | event_utils.rs         | to_event_name          | TODO |
| toAttributeName         | 3    | 转换属性名称              | attribute_utils.rs     | to_attribute_name      | TODO |
| toPropertyName          | 3    | 转换属性名到驼峰式        | property_utils.rs      | to_property_name       | TODO |

### validate.js

| 函数/常量名称   | 行数 | 描述                                     | 迁移目标          | 目标函数          | 状态 |
| --------------- | ---- | ---------------------------------------- | ----------------- | ----------------- | ---- |
| isInvalidMarkup | 85   | 验证HTML标记的有效性，处理特殊情况和转义 | validate.rs       | is_invalid_markup | DONE |
| bodyElement     | 7    | 用作innerHTML上下文的body元素常量        | html_constants.rs | BODY_ELEMENT      | DONE |
| innerHTML       | 5    | 解析HTML片段并返回序列化结果             | validate.rs       | inner_html        | DONE |

## ssr/

### element.js

| 函数/常量名称            | 行数 | 描述                   | 迁移目标                | 目标函数             | 状态 |
| ------------------------ | ---- | ---------------------- | ----------------------- | -------------------- | ---- |
| transformAttributes      | 250  | 转换JSX属性为SSR格式   | ssr_attributes.rs       | transform_attributes | TODO |
| createElement            | 125  | 创建SSR元素            | ssr_element.rs          | create_element       | TODO |
| normalizeAttributes      | 75   | 规范化JSX属性          | ssr_attributes.rs       | normalize_attributes | TODO |
| escapeExpression         | 65   | 转义表达式为安全的HTML | ssr_escape.rs           | escape_expression    | TODO |
| transformChildren        | 65   | 转换子元素为SSR格式    | ssr_children.rs         | transform_children   | TODO |
| transformElement         | 45   | 转换JSX元素为SSR模板   | ssr_element.rs          | transform_element    | TODO |
| transformClasslistObject | 45   | 转换classList对象      | ssr_classlist.rs        | transform_classlist  | TODO |
| transformToObject        | 35   | 将属性转换为对象结构   | ssr_object_transform.rs | transform_to_object  | TODO |
| setAttr                  | 20   | 设置SSR属性            | ssr_attributes.rs       | set_attr             | TODO |
| appendToTemplate         | 8    | 向模板添加值和数组     | ssr_template.rs         | append_to_template   | TODO |
| toAttribute              | 5    | 转换属性名为SSR格式    | ssr_attributes.rs       | to_attribute         | TODO |

### template.js

| 函数/常量名称   | 行数 | 描述                              | 迁移目标        | 目标函数         | 状态 |
| --------------- | ---- | --------------------------------- | --------------- | ---------------- | ---- |
| createTemplate  | 65   | 创建SSR模板，处理模板字符串和缓存 | ssr_template.rs | create_template  | TODO |
| appendTemplates | 5    | 添加模板声明到程序体中            | ssr_template.rs | append_templates | TODO |

## universal/

### element.js

| 函数/常量名称       | 行数 | 描述                        | 迁移目标                | 目标函数             | 状态 |
| ------------------- | ---- | --------------------------- | ----------------------- | -------------------- | ---- |
| transformAttributes | 150  | 转换属性为通用格式          | universal_attributes.rs | transform_attributes | TODO |
| processSpreads      | 120  | 处理展开属性                | universal_spreads.rs    | process_spreads      | TODO |
| transformChildren   | 85   | 转换子元素为通用格式        | universal_children.rs   | transform_children   | TODO |
| transformElement    | 45   | 转换JSX元素为通用渲染器格式 | universal_element.rs    | transform_element    | TODO |
| setAttr             | 10   | 设置通用属性                | universal_attributes.rs | set_attr             | TODO |
| nextChild           | 3    | 获取下一个子元素节点        | universal_utils.rs      | next_child           | TODO |

### template.js

| 函数/常量名称  | 行数 | 描述                                   | 迁移目标              | 目标函数        | 状态 |
| -------------- | ---- | -------------------------------------- | --------------------- | --------------- | ---- |
| wrapDynamics   | 65   | 封装动态属性更新逻辑                   | universal_dynamics.rs | wrap_dynamics   | TODO |
| createTemplate | 35   | 创建通用模板，处理组件声明和动态表达式 | universal_template.rs | create_template | TODO |

### volidElements.ts

| 函数/常量名称  | 行数 | 描述                       | 迁移目标          | 目标函数      | 状态 |
| -------------- | ---- | -------------------------- | ----------------- | ------------- | ---- |
| export default | 21   | 总是需要闭合的HTML元素列表 | html_constants.rs | VOID_ELEMENTS | DONE |
