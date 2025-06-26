use std::fmt;
use indextree::Arena;
use oxc_ast::AstKind;

// 用于格式化Node的辅助结构体
struct NodeFormatter<'a, T> {
    inner: &'a T,
    indent: usize,
}

impl<'a, T> NodeFormatter<'a, T> {
    fn new(inner: &'a T) -> Self {
        Self { inner, indent: 0 }
    }

    fn indent(mut self, level: usize) -> Self {
        self.indent = level;
        self
    }

    fn write_indent(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "    ".repeat(self.indent))
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for NodeFormatter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let debug_str = format!("{:#?}", self.inner);
        let lines: Vec<_> = debug_str.lines().collect();
        
        if lines.len() <= 1 {
            write!(f, "{}", debug_str)
        } else {
            writeln!(f)?;
            for line in lines {
                self.write_indent(f)?;
                writeln!(f, "{}", line)?;
            }
            Ok(())
        }
    }
}

// 包装类型，用于实现Debug
pub struct DebugArena<'a> {
    arena: Arena<AstKind<'a>>,
}

impl<'a> DebugArena<'a> {
    pub fn new(arena: Arena<AstKind<'a>>) -> Self {
        Self { arena }
    }
}

impl<'a> fmt::Debug for DebugArena<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "AstArena {{")?;
        writeln!(f, "    nodes: [")?;
        let nodes: Vec<_> = self.arena.iter().collect();
        for (i, node) in nodes.iter().enumerate() {
            NodeFormatter::new(node).indent(2).fmt(f)?;
            if i < nodes.len() - 1 {
                writeln!(f, ",")?;
            } else {
                writeln!(f)?;
            }
        }
        writeln!(f, "    ]")?;
        write!(f, "}}") // 结尾不需要缩进，保持和开始的大括号对齐
    }
}
