# Technical Context

## OXC Parser Architecture

OXC implements a JavaScript/TypeScript parser in Rust with the following key components:

### Core Modules

1. **oxc_parser**
- Main parser implementation that handles lexing and parsing
- Supports ECMAScript syntax, TypeScript, JSX/TSX 
- Uses a recursive descent parsing approach
- Handles file sizes up to 4GB with u32 span offsets

2. **oxc_ast** 
- Defines the Abstract Syntax Tree (AST) nodes
- Comprehensive node types for all JavaScript/TypeScript constructs
- Uses Rust enums for variant types (e.g. Expression, Statement)
- Maintains span information for source mapping

3. **oxc_allocator**
- Custom memory allocation for AST nodes
- Uses bump allocation for fast allocation and deallocation
- Provides Arena-based memory management

### Key Design Patterns

1. **Memory Management**
- AST nodes allocated in an arena for efficient memory use
- Uses `'a` lifetime parameters to tie allocations to the parser
- Avoids expensive heap allocations where possible

2. **Error Recovery**
- Parser can recover from some syntax errors and continue parsing
- Maintains a list of errors while still producing a valid AST
- Defers complex syntax error checking to semantic analysis

3. **Visitor Pattern**
- AST nodes support visitor traversal 
- Enables static analysis and transformations
- Scoped traversal with contextual information

### API Design

1. **Parser API**
```rust
pub struct Parser<'a> {
    allocator: &'a Allocator,
    source_text: &'a str,
    source_type: SourceType,
    options: ParseOptions,
}
```

2. **AST Structure**
- Node types use Rust enums for variants
- Each node maintains source span information
- TypeScript nodes extend base JavaScript nodes
- Comprehensive coverage of ECMAScript features

3. **Performance Optimizations**
- Uses u32 for span offsets instead of usize
- Delegates complex syntax checks to semantic analysis
- Fast AST node allocation and dropping

### Language Support 

1. **JavaScript Features**
- Full ECMAScript syntax support
- Stage 3 decorators
- Module syntax (import/export)
- Class fields and private methods

2. **TypeScript Features**
- Type annotations and interfaces  
- Generics and type parameters
- Declaration files (.d.ts)
- Namespaces and ambient declarations

3. **JSX/TSX Support**
- Element and fragment parsing
- JSX expressions and spread attributes
- TypeScript JSX specifics

## Semantic Analysis

1. **Core Capabilities**
- Symbol resolution and binding analysis
- Scope tracking and management 
- Variable reference tracking (read/write analysis)
- Control flow graph generation
- JSDoc parsing and analysis
- Label and jump tracking

2. **Design Architecture**
```rust
pub struct Semantic<'a> {
    source_text: &'a str,
    source_type: SourceType,
    nodes: AstNodes<'a>,
    scoping: Scoping,
    classes: ClassTable<'a>,
    comments: &'a oxc_allocator::Vec<'a, Comment>,
    jsdoc: JSDocFinder<'a>,
    cfg: Option<ControlFlowGraph>,
}
```

3. **Key Features**
- Tracks symbol declarations and references
- Handles lexical scoping rules
- Supports TypeScript type system analysis
- Provides control flow analysis
- Maintains class hierarchy information
- Processes JSDoc comments for documentation

4. **Scoping System**
- Maintains scope hierarchy
- Tracks symbol declarations
- Handles variable shadowing
- Supports block scoping
- Manages function and class scopes

## Code Transformation

1. **Transformer Architecture**
- Plugin-based transformation system
- AST visitor pattern implementation
- Maintains transformation context
- Handles source maps

2. **Core Components**
- Transform context tracking
- State management during transforms
- Compiler assumptions handling
- Source map generation

3. **Capabilities**
- AST manipulation and modification
- Code generation from AST
- Source map maintenance
- Plugin system for custom transforms

## Transformation System Details

1. **Transform Pipeline Architecture**
   - Multi-stage transformation process
   - Version-specific transforms (ES2015-ES2022)
   - TypeScript and JSX handling
   - Plugin-based architecture

2. **Core Transformers**
   ```rust
   struct TransformerImpl<'a, 'ctx> {
       x0_typescript: Option<TypeScript<'a, 'ctx>>,
       x1_jsx: Jsx<'a, 'ctx>,
       x2_es2022: ES2022<'a, 'ctx>,
       x2_es2021: ES2021<'a, 'ctx>,
       // ... more stages
   }
   ```

3. **Transformation Stages**
   - TypeScript removal
   - JSX transformation
   - ECMAScript version downleveling
   - Decorator processing
   - Feature polyfilling

4. **Visitor Pattern Implementation**
   - AST traversal and modification
   - State management during transforms 
   - Context preservation
   - Scope tracking

## Babel to Rust Conversion Patterns

1. **AST Transformation**
   - Node type conversion
   - Scope management
   - Symbol resolution
   - Source mapping

2. **Code Generation**
   - TypeScript to JavaScript
   - JSX to plain JavaScript
   - ES features downleveling
   - Source map generation

3. **Plugin Architecture**
   - Transform registration
   - Plugin ordering
   - State management
   - Error handling

4. **Helper Management**
   - Runtime function injection
   - Polyfill handling
   - Module imports
   - Utility functions

## Implementation Notes

1. **Memory Management**
   - Arena-based allocation for AST nodes
   - Custom allocator for performance
   - Reference counting for shared data
   - Memory pooling strategies

2. **Error Handling**
   - Diagnostic collection
   - Source location tracking
   - Error recovery mechanisms
   - Validation checks

3. **Performance Optimization**
   - Fast path optimizations
   - Lazy transformations
   - Caching strategies
   - Minimal allocations

4. **Code Quality**
   - Strong type safety
   - Extensive testing
   - Error handling patterns
   - Documentation coverage

This technical foundation enables oxc to achieve its goals of:
1. Fast, accurate JavaScript/TypeScript parsing
2. Robust semantic analysis
3. Efficient code transformation
4. Memory efficient operation
5. Error recovery capabilities
6. Reliable Babel-to-Rust conversion
