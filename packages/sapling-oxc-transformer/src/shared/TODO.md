# Functions to be migrated from utils.js

The following functions from `packages/babel-plugin-jsx-dom-expressions/src/shared/utils.js` need to be migrated to Rust:

1. ⏳ `isDynamic` (89 lines) - Complex function for checking dynamic expressions
2. ✅ `escapeHTML` (44 lines) -> `string_utils.rs` - HTML escaping utility
3. 🔄 `transformCondition` (41 lines) -> `condition.rs` - Complex transformation for conditional expressions (implementation in progress)
4. ⏳ `registerImportMethod` (22 lines) - For handling module imports and maintaining import caches
5. ✅ `wrappedByText` (21 lines) -> `text_wrap.rs` - Checking if an element is wrapped by text nodes
6. ⏳ `convertJSXIdentifier` (15 lines) - Converting JSX identifiers to regular identifiers
7. ✅ `filterChildren` (7 lines) -> `children.rs` - Filtering JSX children nodes
8. ✅ `jsxElementNameToString` (7 lines) -> `tag_name.rs` - Converting JSX element names to strings
9. ⏳ `checkLength` (7 lines) - Checking length of JSX children
10. ✅ `trimWhitespace` (7 lines) -> `string_utils.rs` - Whitespace handling utility
11. ⏳ `tagNameToIdentifier` (6 lines) - Converting tag names to identifiers
12. ⏳ `getStaticExpression` (6 lines) - Evaluating static JSX expressions
13. ✅ `getTagName` (3 lines) -> `tag_name.rs` - Extracting tag name from JSX elements
14. ✅ `isComponent` (3 lines) -> `component.rs` - Checking if a tag name represents a component
15. ✅ `toEventName` (3 lines) -> `string_utils.rs` - Converting event prop names
16. ✅ `toAttributeName` (3 lines) -> `string_utils.rs` - Converting to kebab-case attribute names
17. ✅ `toPropertyName` (3 lines) -> `string_utils.rs` - Converting to camelCase property names
18. ✅ `canNativeSpread` (3 lines) -> `native_spread.rs` - Checking if props can use native spread
19. ✅ `getNumberedId` (3 lines) -> `id_gen.rs` - Generating numbered identifiers
20. ✅ `escapeStringForTemplate` (3 lines) -> `string_utils.rs` - String escaping for templates

Core constants that need to be migrated:
- ✅ `reservedNameSpaces` (9 items) -> `mod.rs` - Reserved namespace constants
- ✅ `nonSpreadNameSpaces` (6 items) -> `native_spread.rs` - Non-spread namespace constants
- ⏳ `templateEscapes` (12 items)

Legend:
✅ Migrated
🔄 In Progress
⏳ To Be Migrated
