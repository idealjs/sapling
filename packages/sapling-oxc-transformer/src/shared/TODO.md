# Functions to be migrated from utils.js

The following functions from `packages/babel-plugin-jsx-dom-expressions/src/shared/utils.js` need to be migrated to Rust:

1. ⏳ `registerImportMethod` - For handling module imports and maintaining import caches
2. ⏳ `jsxElementNameToString` - Converting JSX element names to strings
3. ⏳ `tagNameToIdentifier` - Converting tag names to identifiers
4. ⏳ `getTagName` - Extracting tag name from JSX elements
5. ⏳ `isComponent` - Checking if a tag name represents a component
6. ⏳ `isDynamic` - Complex function for checking dynamic expressions
7. ⏳ `getStaticExpression` - Evaluating static JSX expressions
8. ✅ `filterChildren` -> `children.rs` - Filtering JSX children nodes
9. ⏳ `checkLength` - Checking length of JSX children
10. ⏳ `trimWhitespace` - Whitespace handling utility
11. ⏳ `toEventName` - Converting event prop names
12. ⏳ `toAttributeName` - Converting to kebab-case attribute names
13. ⏳ `toPropertyName` - Converting to camelCase property names
14. ✅ `wrappedByText` -> `text_wrap.rs` - Checking if an element is wrapped by text nodes
15. 🔄 `transformCondition` -> `condition.rs` - Complex transformation for conditional expressions (implementation in progress)
16. ⏳ `escapeHTML` - HTML escaping utility
17. ⏳ `convertJSXIdentifier` - Converting JSX identifiers to regular identifiers
18. ⏳ `canNativeSpread` - Checking if props can use native spread
19. ✅ `getNumberedId` -> `id_gen.rs` - Generating numbered identifiers
20. ⏳ `escapeStringForTemplate` - String escaping for templates

Core constants that need to be migrated:
- ⏳ `reservedNameSpaces`
- ⏳ `nonSpreadNameSpaces`
- ⏳ `templateEscapes`

Legend:
✅ Migrated
🔄 In Progress
⏳ To Be Migrated
