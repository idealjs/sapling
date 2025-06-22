# Active Context

## Current Development Focus

### Babel-to-Rust Transformation

1. **Transform Pipeline Development**
   - Understanding Babel's transform pipeline
   - Implementing equivalent transforms in Rust
   - Maintaining transform order compatibility
   - Handling edge cases

2. **Current Implementation Points**
   - ECMAScript feature downleveling
   - TypeScript transforms
   - JSX/TSX handling 
   - Source map generation

3. **Key Patterns**
   ```rust
   // Visitor pattern for AST traversal
   impl<'a> Traverse<'a, TransformState<'a>> for TransformerImpl<'a, '_> {
       fn enter_program(&mut self, program: &mut Program<'a>, ctx: &mut TraverseCtx<'a>) {
           // Transform implementations
       }
       // Other visitor methods
   }
   ```

## Active Learnings

1. **AST Manipulation**
   - Node transformation patterns
   - Scope and context management
   - Symbol resolution strategies
   - State management during transforms

2. **Code Generation**
   - Converting TypeScript features
   - Handling JSX syntax
   - Managing polyfills
   - Source mapping

3. **Error Handling**
   - Recovery strategies
   - Diagnostic collection
   - Source location tracking
   - Validation approaches

## Important Patterns

1. **Transform Organization**
   - Sequential transform stages
   - Plugin architecture
   - State management
   - Context preservation

2. **Type Safety**
   - Strong type checking
   - AST node validation
   - Reference tracking
   - Memory safety

3. **Performance**
   - Minimal allocations
   - Efficient traversal
   - State reuse
   - Memory pooling

## Next Steps

1. **Immediate Tasks**
   - Study each ES transform implementation
   - Understand TypeScript elimination patterns
   - Analyze JSX conversion strategies
   - Review helper injection methods

2. **Implementation Focus**
   - Port complex Babel transforms
   - Maintain compatibility
   - Ensure performance
   - Add test coverage

3. **Documentation Needs**
   - Transform patterns
   - AST manipulation guides
   - Migration strategies
   - Best practices

## Working Notes

1. **Key Concepts**
   - Transform pipeline structure
   - Visitor pattern implementation
   - State management approaches
   - Error handling strategies

2. **Implementation Tips**
   - Use arena allocation for AST nodes
   - Maintain source locations
   - Handle edge cases
   - Write comprehensive tests

3. **Common Patterns**
   - Node type conversion
   - Scope management
   - Symbol resolution
   - Context preservation

## Reference Information

1. **Important Files**
   ```
   oxc/crates/
   ├── oxc_transformer/     # Core transformation
   ├── oxc_ast/            # AST definitions
   ├── oxc_semantic/       # Semantic analysis
   └── oxc_allocator/      # Memory management
   ```

2. **Key Transforms**
   - ECMAScript version transforms
   - TypeScript elimination
   - JSX conversion
   - Decorator handling

3. **Helper Functions**
   - Runtime function injection
   - Polyfill management
   - Type conversions
   - Source mapping

## Development Guidelines

1. **Code Standards**
   - Follow Rust idioms
   - Use strong typing
   - Handle all error cases
   - Document complex logic

2. **Testing Requirements**
   - Unit test transforms
   - Integration test flows
   - Edge case coverage
   - Performance benchmarks

3. **Review Checklist**
   - Type safety
   - Memory management
   - Error handling
   - Documentation
   - Test coverage

This active context provides guidance for implementing Babel-to-Rust transformations while maintaining compatibility and performance.
