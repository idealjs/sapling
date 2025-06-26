# Babel Plugin JSX DOM Expressions Migration Tasks

## dom/

### constants.js

| 函数/常量名称  | 行数 | 描述                                       | 迁移目标          |
| -------------- | ---- | ------------------------------------------ | ----------------- |
| InlineElements | 54   | 内联元素列表常量，定义了HTML中所有内联元素 | html_constants.rs |
| BlockElements  | 35   | 块级元素列表常量，定义了HTML中所有块级元素 | html_constants.rs |
| alwaysClose    | 21   | 总是需要闭合的HTML元素列表                 | html_constants.rs |

### element.js

| 函数/常量名称                | 行数 | 描述                                       | 迁移目标               |
| ---------------------------- | ---- | ------------------------------------------ | ---------------------- |
| transformAttributes          | 450  | 处理和转换JSX元素的属性                    | element_attributes.rs  |
| setAttr                      | 185  | 设置元素属性，处理不同类型的属性和命名空间 | element_attributes.rs  |
| transformChildren            | 125  | 转换和处理JSX子元素                        | element_children.rs    |
| processSpreads               | 120  | 处理JSX的展开属性                          | spread_attributes.rs   |
| transformElement             | 98   | 转换JSX元素为DOM元素                       | dom_element.rs         |
| detectExpressions            | 58   | 检测JSX中的动态表达式                      | expression_detector.rs |
| createPlaceholder            | 35   | 创建DOM占位符元素                          | placeholder.rs         |
| findLastElement              | 18   | 在子元素列表中查找最后一个元素             | element_utils.rs       |
| detectResolvableEventHandler | 15   | 检测事件处理器是否可解析                   | event_handler.rs       |
| contextToCustomElement       | 15   | 为自定义元素添加上下文支持                 | custom_element.rs      |
| nextChild                    | 3    | 获取下一个子元素节点                       | element_utils.rs       |

### template.js

| 函数/常量名称    | 行数 | 描述                                       | 迁移目标             |
| ---------------- | ---- | ------------------------------------------ | -------------------- |
| wrapDynamics     | 95   | 封装动态属性的更新逻辑，处理动态样式和类名 | dynamic_wrapper.rs   |
| appendTemplates  | 45   | 添加模板到AST，处理SVG和MathML特殊情况     | template_append.rs   |
| registerTemplate | 42   | 注册模板并处理模板的hydration逻辑          | template_registry.rs |
| createTemplate   | 35   | 创建模板，处理模板的声明和动态表达式       | template_creator.rs  |

## shared/

### component.js

| 函数/常量名称              | 行数 | 描述                             | 迁移目标               |
| -------------------------- | ---- | -------------------------------- | ---------------------- |
| transformComponent         | 220  | 转换JSX组件，处理props和children | component_transform.rs |
| transformComponentChildren | 85   | 转换组件子元素，处理动态内容     | component_children.rs  |
| convertComponentIdentifier | 15   | 转换JSX组件标识符为AST表达式     | component_utils.rs     |

### fragment.js

| 函数/常量名称             | 行数 | 描述                                         | 迁移目标              |
| ------------------------- | ---- | -------------------------------------------- | --------------------- |
| transformFragmentChildren | 16   | 转换JSX Fragment的子元素，处理文本和节点转换 | fragment_transform.rs |

### postprocess.js

| 函数/常量名称  | 行数 | 描述                                      | 迁移目标          |
| -------------- | ---- | ----------------------------------------- | ----------------- |
| default export | 45   | 后处理转换后的AST，添加事件代理和模板验证 | post_processor.rs |

### preprocess.js

| 函数/常量名称           | 行数 | 描述                            | 迁移目标         |
| ----------------------- | ---- | ------------------------------- | ---------------- |
| default export          | 25   | 预处理AST，处理导入源和验证选项 | preprocessor.rs  |
| JSXValidator.JSXElement | 20   | 验证JSX嵌套的合法性             | jsx_validator.rs |

### transform.js

| 函数/常量名称           | 行数 | 描述                                          | 迁移目标             |
| ----------------------- | ---- | --------------------------------------------- | -------------------- |
| transformNode           | 125  | 转换各种类型的JSX节点（元素、文本、表达式等） | node_transform.rs    |
| transformThis           | 65   | 处理this表达式和JSX中的this引用               | this_transform.rs    |
| transformJSX            | 20   | 转换JSX代码，处理顶层节点和模板创建           | jsx_transform.rs     |
| getCreateTemplate       | 15   | 获取对应的模板创建函数                        | template_factory.rs  |
| transformElement        | 15   | 根据不同渲染器转换JSX元素                     | element_transform.rs |
| getTargetFunctionParent | 10   | 获取目标函数的父作用域                        | scope_utils.rs       |

### utils.js

| 函数/常量名称           | 行数 | 描述                      | 迁移目标               |
| ----------------------- | ---- | ------------------------- | ---------------------- |
| isDynamic               | 89   | 检查表达式是否为动态的    | expression_utils.rs    |
| escapeHTML              | 44   | HTML转义                  | html_escape.rs         |
| transformCondition      | 41   | 转换条件表达式            | condition_transform.rs |
| wrappedByText           | 21   | 检查是否被文本节点包围    | text_utils.rs          |
| convertJSXIdentifier    | 15   | 转换JSX标识符             | jsx_utils.rs           |
| templateEscapes         | 12   | 模板转义字符映射          | escape_utils.rs        |
| reservedNameSpaces      | 10   | 保留的命名空间集合        | namespace_constants.rs |
| jsxElementNameToString  | 7    | 将JSX元素名称转换为字符串 | jsx_utils.rs           |
| filterChildren          | 7    | 过滤JSX子节点             | children_utils.rs      |
| checkLength             | 7    | 检查子节点长度            | children_utils.rs      |
| trimWhitespace          | 7    | 处理空白字符              | string_utils.rs        |
| tagNameToIdentifier     | 6    | 将标签名转换为标识符      | jsx_utils.rs           |
| getStaticExpression     | 6    | 获取静态表达式值          | expression_utils.rs    |
| canNativeSpread         | 6    | 检查是否可以使用原生展开  | spread_utils.rs        |
| getNumberedId           | 3    | 生成数字ID                | id_utils.rs            |
| escapeStringForTemplate | 3    | 转义模板字符串            | escape_utils.rs        |
| getConfig               | 3    | 获取配置信息              | config_utils.rs        |
| getRendererConfig       | 3    | 获取渲染器配置            | config_utils.rs        |
| getTagName              | 3    | 获取JSX标签名             | jsx_utils.rs           |
| isComponent             | 3    | 检查是否为组件            | component_utils.rs     |
| toEventName             | 3    | 转换事件名称              | event_utils.rs         |
| toAttributeName         | 3    | 转换属性名称              | attribute_utils.rs     |
| toPropertyName          | 3    | 转换属性名到驼峰式        | property_utils.rs      |

### validate.js

| 函数/常量名称   | 行数 | 描述                                     | 迁移目标            |
| --------------- | ---- | ---------------------------------------- | ------------------- |
| isInvalidMarkup | 85   | 验证HTML标记的有效性，处理特殊情况和转义 | markup_validator.rs |
| bodyElement     | 7    | 用作innerHTML上下文的body元素常量        | html_constants.rs   |
| innerHTML       | 5    | 解析HTML片段并返回序列化结果             | html_parser.rs      |

## ssr/

### element.js

| 函数/常量名称            | 行数 | 描述                   | 迁移目标                |
| ------------------------ | ---- | ---------------------- | ----------------------- |
| transformAttributes      | 250  | 转换JSX属性为SSR格式   | ssr_attributes.rs       |
| createElement            | 125  | 创建SSR元素            | ssr_element.rs          |
| normalizeAttributes      | 75   | 规范化JSX属性          | ssr_attributes.rs       |
| escapeExpression         | 65   | 转义表达式为安全的HTML | ssr_escape.rs           |
| transformChildren        | 65   | 转换子元素为SSR格式    | ssr_children.rs         |
| transformElement         | 45   | 转换JSX元素为SSR模板   | ssr_element.rs          |
| transformClasslistObject | 45   | 转换classList对象      | ssr_classlist.rs        |
| transformToObject        | 35   | 将属性转换为对象结构   | ssr_object_transform.rs |
| setAttr                  | 20   | 设置SSR属性            | ssr_attributes.rs       |
| appendToTemplate         | 8    | 向模板添加值和数组     | ssr_template.rs         |
| toAttribute              | 5    | 转换属性名为SSR格式    | ssr_attributes.rs       |

### template.js

| 函数/常量名称   | 行数 | 描述                              | 迁移目标        |
| --------------- | ---- | --------------------------------- | --------------- |
| createTemplate  | 65   | 创建SSR模板，处理模板字符串和缓存 | ssr_template.rs |
| appendTemplates | 5    | 添加模板声明到程序体中            | ssr_template.rs |

## universal/

### element.js

| 函数/常量名称       | 行数 | 描述                        | 迁移目标                |
| ------------------- | ---- | --------------------------- | ----------------------- |
| transformAttributes | 150  | 转换属性为通用格式          | universal_attributes.rs |
| processSpreads      | 120  | 处理展开属性                | universal_spreads.rs    |
| transformChildren   | 85   | 转换子元素为通用格式        | universal_children.rs   |
| transformElement    | 45   | 转换JSX元素为通用渲染器格式 | universal_element.rs    |
| setAttr             | 10   | 设置通用属性                | universal_attributes.rs |
| nextChild           | 3    | 获取下一个子元素节点        | universal_utils.rs      |

### template.js

| 函数/常量名称  | 行数 | 描述                                   | 迁移目标              |
| -------------- | ---- | -------------------------------------- | --------------------- |
| wrapDynamics   | 65   | 封装动态属性更新逻辑                   | universal_dynamics.rs |
| createTemplate | 35   | 创建通用模板，处理组件声明和动态表达式 | universal_template.rs |
