#![expect(clippy::print_stdout)]
use napi_derive::napi;

use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_span::SourceType;
use std::path::Path;

