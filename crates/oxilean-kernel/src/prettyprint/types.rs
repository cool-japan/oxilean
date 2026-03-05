//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::Write as FmtWrite;

use super::functions::*;
use crate::{BinderInfo, Expr, Level, Name};

/// Pretty printer for kernel expressions.
pub struct ExprPrinter {
    /// Output buffer
    buffer: String,
    /// Print configuration
    pub(super) config: PrintConfig,
}
impl ExprPrinter {
    /// Create a new expression printer with default config.
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            config: PrintConfig::default(),
        }
    }
    /// Create a printer with a specific configuration.
    pub fn with_config(config: PrintConfig) -> Self {
        Self {
            buffer: String::new(),
            config,
        }
    }
    /// Set whether to use unicode symbols.
    pub fn with_unicode(mut self, unicode: bool) -> Self {
        self.config.unicode = unicode;
        self
    }
    /// Get the output.
    pub fn output(self) -> String {
        self.buffer
    }
    /// Print an expression.
    pub fn print(&mut self, expr: &Expr) -> fmt::Result {
        self.print_expr(expr, 0)
    }
    fn print_expr(&mut self, expr: &Expr, prec: u32) -> fmt::Result {
        match expr {
            Expr::Sort(level) => self.print_sort(level),
            Expr::BVar(idx) => {
                if self.config.show_indices {
                    write!(self.buffer, "#{}", idx)
                } else {
                    write!(self.buffer, "_")
                }
            }
            Expr::FVar(fvar) => write!(self.buffer, "@{}", fvar.0),
            Expr::Const(name, levels) => {
                self.print_name(name)?;
                if self.config.show_universes && !levels.is_empty() {
                    write!(self.buffer, ".{{")?;
                    for (i, level) in levels.iter().enumerate() {
                        if i > 0 {
                            write!(self.buffer, ", ")?;
                        }
                        self.print_level(level)?;
                    }
                    write!(self.buffer, "}}")?;
                }
                Ok(())
            }
            Expr::App(fun, arg) => {
                let (head, args) = collect_app_args(expr);
                if args.len() > 1 {
                    if prec > 10 {
                        write!(self.buffer, "(")?;
                    }
                    self.print_expr(head, 10)?;
                    for a in &args {
                        write!(self.buffer, " ")?;
                        self.print_expr(a, 11)?;
                    }
                    if prec > 10 {
                        write!(self.buffer, ")")?;
                    }
                    Ok(())
                } else {
                    if prec > 10 {
                        write!(self.buffer, "(")?;
                    }
                    self.print_expr(fun, 10)?;
                    write!(self.buffer, " ")?;
                    self.print_expr(arg, 11)?;
                    if prec > 10 {
                        write!(self.buffer, ")")?;
                    }
                    Ok(())
                }
            }
            Expr::Lam(bi, name, ty, body) => {
                if prec > 0 {
                    write!(self.buffer, "(")?;
                }
                if self.config.unicode {
                    write!(self.buffer, "λ ")?;
                } else {
                    write!(self.buffer, "fun ")?;
                }
                self.print_binder(*bi, name, ty)?;
                let mut current_body = body.as_ref();
                while let Expr::Lam(bi2, name2, ty2, body2) = current_body {
                    write!(self.buffer, " ")?;
                    self.print_binder(*bi2, name2, ty2)?;
                    current_body = body2;
                }
                if self.config.unicode {
                    write!(self.buffer, ", ")?;
                } else {
                    write!(self.buffer, " => ")?;
                }
                self.print_expr(current_body, 0)?;
                if prec > 0 {
                    write!(self.buffer, ")")?;
                }
                Ok(())
            }
            Expr::Pi(bi, name, ty, body) => {
                if prec > 0 {
                    write!(self.buffer, "(")?;
                }
                if name.is_anonymous() || *name == Name::str("_") {
                    self.print_expr(ty, 25)?;
                    if self.config.unicode {
                        write!(self.buffer, " → ")?;
                    } else {
                        write!(self.buffer, " -> ")?;
                    }
                    self.print_expr(body, 24)?;
                } else {
                    if self.config.unicode {
                        write!(self.buffer, "∀ ")?;
                    } else {
                        write!(self.buffer, "forall ")?;
                    }
                    self.print_binder(*bi, name, ty)?;
                    let mut current_body = body.as_ref();
                    while let Expr::Pi(bi2, name2, ty2, body2) = current_body {
                        if !name2.is_anonymous() && *name2 != Name::str("_") {
                            write!(self.buffer, " ")?;
                            self.print_binder(*bi2, name2, ty2)?;
                            current_body = body2;
                        } else {
                            break;
                        }
                    }
                    write!(self.buffer, ", ")?;
                    self.print_expr(current_body, 0)?;
                }
                if prec > 0 {
                    write!(self.buffer, ")")?;
                }
                Ok(())
            }
            Expr::Let(name, ty, val, body) => {
                write!(self.buffer, "let ")?;
                self.print_name(name)?;
                write!(self.buffer, " : ")?;
                self.print_expr(ty, 0)?;
                write!(self.buffer, " := ")?;
                self.print_expr(val, 0)?;
                write!(self.buffer, " in ")?;
                self.print_expr(body, 0)
            }
            Expr::Lit(lit) => write!(self.buffer, "{}", lit),
            Expr::Proj(name, idx, expr) => {
                self.print_expr(expr, 11)?;
                write!(self.buffer, ".{}.{}", name, idx)
            }
        }
    }
    /// Print a binder with its info.
    fn print_binder(&mut self, bi: BinderInfo, name: &Name, ty: &Expr) -> fmt::Result {
        if self.config.show_binder_info {
            match bi {
                BinderInfo::Default => {
                    write!(self.buffer, "(")?;
                    self.print_name(name)?;
                    write!(self.buffer, " : ")?;
                    self.print_expr(ty, 0)?;
                    write!(self.buffer, ")")
                }
                BinderInfo::Implicit => {
                    write!(self.buffer, "{{")?;
                    self.print_name(name)?;
                    write!(self.buffer, " : ")?;
                    self.print_expr(ty, 0)?;
                    write!(self.buffer, "}}")
                }
                BinderInfo::StrictImplicit => {
                    if self.config.unicode {
                        write!(self.buffer, "\u{2983}")?;
                    } else {
                        write!(self.buffer, "{{{{")?;
                    }
                    self.print_name(name)?;
                    write!(self.buffer, " : ")?;
                    self.print_expr(ty, 0)?;
                    if self.config.unicode {
                        write!(self.buffer, "\u{2984}")
                    } else {
                        write!(self.buffer, "}}}}")
                    }
                }
                BinderInfo::InstImplicit => {
                    write!(self.buffer, "[")?;
                    self.print_name(name)?;
                    write!(self.buffer, " : ")?;
                    self.print_expr(ty, 0)?;
                    write!(self.buffer, "]")
                }
            }
        } else {
            self.print_name(name)?;
            write!(self.buffer, " : ")?;
            self.print_expr(ty, 0)
        }
    }
    /// Print a sort expression with smart level display.
    fn print_sort(&mut self, level: &Level) -> fmt::Result {
        match level {
            Level::Zero => write!(self.buffer, "Prop"),
            _ => {
                if let Some(n) = level_to_nat(level) {
                    if n == 1 {
                        write!(self.buffer, "Type")
                    } else {
                        write!(self.buffer, "Type {}", n - 1)
                    }
                } else {
                    write!(self.buffer, "Sort ")?;
                    self.print_level(level)
                }
            }
        }
    }
    pub(super) fn print_level(&mut self, level: &Level) -> fmt::Result {
        match level {
            Level::Zero => write!(self.buffer, "0"),
            Level::Succ(_) => {
                if let Some(n) = level_to_nat(level) {
                    write!(self.buffer, "{}", n)
                } else {
                    let (base, offset) = level_to_offset(level);
                    if offset > 0 {
                        self.print_level(base)?;
                        write!(self.buffer, "+{}", offset)
                    } else {
                        write!(self.buffer, "(")?;
                        self.print_level(level)?;
                        write!(self.buffer, ")")
                    }
                }
            }
            Level::Max(l1, l2) => {
                write!(self.buffer, "max(")?;
                self.print_level(l1)?;
                write!(self.buffer, ", ")?;
                self.print_level(l2)?;
                write!(self.buffer, ")")
            }
            Level::IMax(l1, l2) => {
                write!(self.buffer, "imax(")?;
                self.print_level(l1)?;
                write!(self.buffer, ", ")?;
                self.print_level(l2)?;
                write!(self.buffer, ")")
            }
            Level::Param(name) => self.print_name(name),
            Level::MVar(id) => write!(self.buffer, "?u_{}", id.0),
        }
    }
    fn print_name(&mut self, name: &Name) -> fmt::Result {
        write!(self.buffer, "{}", name)
    }
}
/// Prints an expression as an S-expression.
#[allow(dead_code)]
pub struct SExprPrinter {
    indent: usize,
}
#[allow(dead_code)]
impl SExprPrinter {
    /// Creates a new S-expression printer.
    pub fn new() -> Self {
        Self { indent: 0 }
    }
    /// Renders a name applied to arguments as `(name arg1 arg2 ...)`.
    pub fn app(&self, name: &str, args: &[&str]) -> String {
        if args.is_empty() {
            return name.to_string();
        }
        format!("({} {})", name, args.join(" "))
    }
    /// Renders a nested S-expression with indentation.
    pub fn nested_app(&self, name: &str, args: &[String]) -> String {
        if args.is_empty() {
            return name.to_string();
        }
        let pad = " ".repeat(self.indent + 2);
        let inner = args
            .iter()
            .map(|a| format!("{}{}", pad, a))
            .collect::<Vec<_>>()
            .join("\n");
        format!("({}\n{})", name, inner)
    }
    /// Renders a list.
    pub fn list(&self, items: &[String]) -> String {
        format!("[{}]", items.join(", "))
    }
}
/// A simple LRU cache backed by a linked list + hash map.
#[allow(dead_code)]
pub struct SimpleLruCache<K: std::hash::Hash + Eq + Clone, V: Clone> {
    capacity: usize,
    map: std::collections::HashMap<K, usize>,
    keys: Vec<K>,
    vals: Vec<V>,
    order: Vec<usize>,
}
#[allow(dead_code)]
impl<K: std::hash::Hash + Eq + Clone, V: Clone> SimpleLruCache<K, V> {
    /// Creates a new LRU cache with the given capacity.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            map: std::collections::HashMap::new(),
            keys: Vec::new(),
            vals: Vec::new(),
            order: Vec::new(),
        }
    }
    /// Inserts or updates a key-value pair.
    pub fn put(&mut self, key: K, val: V) {
        if let Some(&idx) = self.map.get(&key) {
            self.vals[idx] = val;
            self.order.retain(|&x| x != idx);
            self.order.insert(0, idx);
            return;
        }
        if self.keys.len() >= self.capacity {
            let evict_idx = *self
                .order
                .last()
                .expect("order list must be non-empty before eviction");
            self.map.remove(&self.keys[evict_idx]);
            self.order.pop();
            self.keys[evict_idx] = key.clone();
            self.vals[evict_idx] = val;
            self.map.insert(key, evict_idx);
            self.order.insert(0, evict_idx);
        } else {
            let idx = self.keys.len();
            self.keys.push(key.clone());
            self.vals.push(val);
            self.map.insert(key, idx);
            self.order.insert(0, idx);
        }
    }
    /// Returns a reference to the value for `key`, promoting it.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let idx = *self.map.get(key)?;
        self.order.retain(|&x| x != idx);
        self.order.insert(0, idx);
        Some(&self.vals[idx])
    }
    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.keys.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}
/// An abstract pretty-print document.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum PrettyDoc {
    /// A literal text string.
    Text(String),
    /// A newline followed by indentation.
    Newline,
    /// Increase indentation for nested content.
    Indent(Box<PrettyDoc>),
    /// Concatenation of two documents.
    Concat(Box<PrettyDoc>, Box<PrettyDoc>),
    /// A group: try to fit on one line, break if too long.
    Group(Box<PrettyDoc>),
    /// An empty document.
    Empty,
}
#[allow(dead_code)]
impl PrettyDoc {
    /// Creates a `Text` document.
    pub fn text(s: impl Into<String>) -> Self {
        PrettyDoc::Text(s.into())
    }
    /// Creates a `Concat` document.
    pub fn concat(a: PrettyDoc, b: PrettyDoc) -> Self {
        PrettyDoc::Concat(Box::new(a), Box::new(b))
    }
    /// Creates an `Indent` document.
    pub fn indent(inner: PrettyDoc) -> Self {
        PrettyDoc::Indent(Box::new(inner))
    }
    /// Creates a `Group` document.
    pub fn group(inner: PrettyDoc) -> Self {
        PrettyDoc::Group(Box::new(inner))
    }
    /// Renders the document to a `String` using the given line width and indent style.
    pub fn render(&self, _width: usize, style: IndentStyle) -> String {
        let mut out = String::new();
        let mut depth = 0usize;
        self.render_inner(&mut out, &mut depth, &style);
        out
    }
    fn render_inner(&self, out: &mut String, depth: &mut usize, style: &IndentStyle) {
        match self {
            PrettyDoc::Text(s) => out.push_str(s),
            PrettyDoc::Newline => {
                out.push('\n');
                out.push_str(&style.for_depth(*depth));
            }
            PrettyDoc::Indent(inner) => {
                *depth += 1;
                inner.render_inner(out, depth, style);
                *depth -= 1;
            }
            PrettyDoc::Concat(a, b) => {
                a.render_inner(out, depth, style);
                b.render_inner(out, depth, style);
            }
            PrettyDoc::Group(inner) => inner.render_inner(out, depth, style),
            PrettyDoc::Empty => {}
        }
    }
}
/// A pretty printer that works with `PrettyToken` streams.
#[allow(dead_code)]
pub struct TokenPrinter {
    config: PrettyConfig,
    scheme: ColorScheme,
}
#[allow(dead_code)]
impl TokenPrinter {
    /// Creates a new token printer.
    pub fn new(config: PrettyConfig) -> Self {
        Self {
            config,
            scheme: ColorScheme::MONO,
        }
    }
    /// Sets the color scheme.
    pub fn with_colors(mut self, scheme: ColorScheme) -> Self {
        self.scheme = scheme;
        self
    }
    /// Renders a slice of tokens to a `String`.
    pub fn render(&self, tokens: &[PrettyToken]) -> String {
        let mut out = String::new();
        for tok in tokens {
            if self.config.colored {
                out.push_str(&tok.colored_text(&self.scheme));
            } else {
                out.push_str(tok.raw_text());
            }
        }
        out
    }
    /// Renders with line wrapping at `config.width`.
    pub fn render_wrapped(&self, tokens: &[PrettyToken]) -> String {
        let mut out = String::new();
        let mut col = 0usize;
        for tok in tokens {
            let text = tok.raw_text();
            if matches!(tok, PrettyToken::LineBreak) {
                out.push('\n');
                col = 0;
                continue;
            }
            if col + text.len() > self.config.width && col > 0 {
                out.push('\n');
                col = 0;
            }
            out.push_str(text);
            col += text.len();
        }
        out
    }
}
/// Utilities for computing display widths.
#[allow(dead_code)]
pub struct FmtWidth;
#[allow(dead_code)]
impl FmtWidth {
    /// Returns the display width of a `usize` in decimal.
    pub fn decimal_width(n: usize) -> usize {
        if n == 0 {
            return 1;
        }
        let mut w = 0;
        let mut v = n;
        while v > 0 {
            v /= 10;
            w += 1;
        }
        w
    }
    /// Pads `s` on the right with spaces to `width` total chars.
    pub fn pad_right(s: &str, width: usize) -> String {
        if s.len() >= width {
            return s.to_string();
        }
        format!("{}{}", s, " ".repeat(width - s.len()))
    }
    /// Pads `s` on the left with spaces to `width` total chars.
    pub fn pad_left(s: &str, width: usize) -> String {
        if s.len() >= width {
            return s.to_string();
        }
        format!("{}{}", " ".repeat(width - s.len()), s)
    }
    /// Centers `s` within `width` chars.
    pub fn center(s: &str, width: usize) -> String {
        if s.len() >= width {
            return s.to_string();
        }
        let pad = width - s.len();
        let left = pad / 2;
        let right = pad - left;
        format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
    }
}
/// An indented document for pretty-printing with line wrapping.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Doc {
    /// Empty document.
    Empty,
    /// A text fragment.
    Text(String),
    /// Append two documents.
    Concat(Box<Doc>, Box<Doc>),
    /// A line break; collapses to a space when flattened.
    Line,
    /// Indented document.
    Nest(usize, Box<Doc>),
    /// Choose between two layouts.
    Union(Box<Doc>, Box<Doc>),
}
#[allow(dead_code)]
impl Doc {
    /// Create a text document.
    pub fn text(s: impl Into<String>) -> Self {
        Doc::Text(s.into())
    }
    /// Append two documents.
    pub fn concat(self, other: Doc) -> Self {
        Doc::Concat(Box::new(self), Box::new(other))
    }
    /// Add a line break.
    pub fn line() -> Self {
        Doc::Line
    }
    /// Indent a document by `n` spaces.
    pub fn nest(n: usize, doc: Doc) -> Self {
        Doc::Nest(n, Box::new(doc))
    }
    /// Render to a string with the given page width.
    pub fn render(&self, width: usize) -> String {
        let mut out = String::new();
        self.render_impl(0, &mut out, width);
        out
    }
    fn render_impl(&self, indent: usize, out: &mut String, _width: usize) {
        match self {
            Doc::Empty => {}
            Doc::Text(s) => out.push_str(s),
            Doc::Concat(a, b) => {
                a.render_impl(indent, out, _width);
                b.render_impl(indent, out, _width);
            }
            Doc::Line => {
                out.push('\n');
                for _ in 0..indent {
                    out.push(' ');
                }
            }
            Doc::Nest(n, doc) => {
                doc.render_impl(indent + n, out, _width);
            }
            Doc::Union(a, _b) => {
                a.render_impl(indent, out, _width);
            }
        }
    }
}
/// A simple ASCII table formatter.
#[allow(dead_code)]
pub struct PrettyTable {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}
#[allow(dead_code)]
impl PrettyTable {
    /// Creates a new table with the given headers.
    pub fn new(headers: Vec<String>) -> Self {
        Self {
            headers,
            rows: Vec::new(),
        }
    }
    /// Adds a row.  Pads or truncates to match the number of columns.
    pub fn add_row(&mut self, row: Vec<String>) {
        let n = self.headers.len();
        let mut r = row;
        r.resize(n, String::new());
        self.rows.push(r);
    }
    /// Renders the table as an ASCII grid.
    pub fn render(&self) -> String {
        let n = self.headers.len();
        let mut widths: Vec<usize> = self.headers.iter().map(|h| h.len()).collect();
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < n {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }
        let sep: String = widths
            .iter()
            .map(|&w| "-".repeat(w + 2))
            .collect::<Vec<_>>()
            .join("+");
        let sep = format!("+{}+", sep);
        let mut out = String::new();
        out.push_str(&sep);
        out.push('\n');
        let header_line: String = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, h)| FmtWidth::pad_right(h, widths[i]))
            .collect::<Vec<_>>()
            .join(" | ");
        out.push_str(&format!("| {} |", header_line));
        out.push('\n');
        out.push_str(&sep);
        out.push('\n');
        for row in &self.rows {
            let line: String = (0..n)
                .map(|i| {
                    FmtWidth::pad_right(row.get(i).map(|s| s.as_str()).unwrap_or(""), widths[i])
                })
                .collect::<Vec<_>>()
                .join(" | ");
            out.push_str(&format!("| {} |", line));
            out.push('\n');
        }
        out.push_str(&sep);
        out
    }
}
/// A FIFO work queue.
#[allow(dead_code)]
pub struct WorkQueue<T> {
    items: std::collections::VecDeque<T>,
}
#[allow(dead_code)]
impl<T> WorkQueue<T> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self {
            items: std::collections::VecDeque::new(),
        }
    }
    /// Enqueues a work item.
    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }
    /// Dequeues the next work item.
    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Returns the number of pending items.
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
/// A key-value store for diagnostic metadata.
#[allow(dead_code)]
pub struct DiagMeta {
    pub(super) entries: Vec<(String, String)>,
}
#[allow(dead_code)]
impl DiagMeta {
    /// Creates an empty metadata store.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Adds a key-value pair.
    pub fn add(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.entries.push((key.into(), val.into()));
    }
    /// Returns the value for `key`, or `None`.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A type-safe wrapper around a `u32` identifier.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedId<T> {
    pub(super) id: u32,
    _phantom: std::marker::PhantomData<T>,
}
#[allow(dead_code)]
impl<T> TypedId<T> {
    /// Creates a new typed ID.
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            _phantom: std::marker::PhantomData,
        }
    }
    /// Returns the raw `u32` ID.
    pub fn raw(&self) -> u32 {
        self.id
    }
}
/// Mutable state threaded through a recursive pretty printer.
#[allow(dead_code)]
pub struct PrettyPrinterState {
    output: String,
    indent_depth: usize,
    indent_str: String,
    pub(crate) col: usize,
    max_width: usize,
}
#[allow(dead_code)]
impl PrettyPrinterState {
    /// Creates a new printer state.
    pub fn new(max_width: usize, indent: IndentStyle) -> Self {
        Self {
            output: String::new(),
            indent_depth: 0,
            indent_str: indent.one_level(),
            col: 0,
            max_width,
        }
    }
    /// Writes a string token.
    pub fn write(&mut self, s: &str) {
        self.output.push_str(s);
        self.col += s.len();
    }
    /// Writes a newline and current indentation.
    pub fn newline(&mut self) {
        self.output.push('\n');
        let indent = self.indent_str.repeat(self.indent_depth);
        self.output.push_str(&indent);
        self.col = indent.len();
    }
    /// Increases indentation depth.
    pub fn push_indent(&mut self) {
        self.indent_depth += 1;
    }
    /// Decreases indentation depth.
    pub fn pop_indent(&mut self) {
        if self.indent_depth > 0 {
            self.indent_depth -= 1;
        }
    }
    /// Returns `true` if the current line is past `max_width`.
    pub fn over_width(&self) -> bool {
        self.col > self.max_width
    }
    /// Returns the accumulated output.
    pub fn finish(self) -> String {
        self.output
    }
}
/// A set of non-overlapping integer intervals.
#[allow(dead_code)]
pub struct IntervalSet {
    intervals: Vec<(i64, i64)>,
}
#[allow(dead_code)]
impl IntervalSet {
    /// Creates an empty interval set.
    pub fn new() -> Self {
        Self {
            intervals: Vec::new(),
        }
    }
    /// Adds the interval `[lo, hi]` to the set.
    pub fn add(&mut self, lo: i64, hi: i64) {
        if lo > hi {
            return;
        }
        let mut new_lo = lo;
        let mut new_hi = hi;
        let mut i = 0;
        while i < self.intervals.len() {
            let (il, ih) = self.intervals[i];
            if ih < new_lo - 1 {
                i += 1;
                continue;
            }
            if il > new_hi + 1 {
                break;
            }
            new_lo = new_lo.min(il);
            new_hi = new_hi.max(ih);
            self.intervals.remove(i);
        }
        self.intervals.insert(i, (new_lo, new_hi));
    }
    /// Returns `true` if `x` is in the set.
    pub fn contains(&self, x: i64) -> bool {
        self.intervals.iter().any(|&(lo, hi)| x >= lo && x <= hi)
    }
    /// Returns the number of intervals.
    pub fn num_intervals(&self) -> usize {
        self.intervals.len()
    }
    /// Returns the total count of integers covered.
    pub fn cardinality(&self) -> i64 {
        self.intervals.iter().map(|&(lo, hi)| hi - lo + 1).sum()
    }
}
/// A counter that dispenses monotonically increasing `TypedId` values.
#[allow(dead_code)]
pub struct IdDispenser<T> {
    next: u32,
    _phantom: std::marker::PhantomData<T>,
}
#[allow(dead_code)]
impl<T> IdDispenser<T> {
    /// Creates a new dispenser starting from zero.
    pub fn new() -> Self {
        Self {
            next: 0,
            _phantom: std::marker::PhantomData,
        }
    }
    /// Dispenses the next ID.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> TypedId<T> {
        let id = TypedId::new(self.next);
        self.next += 1;
        id
    }
    /// Returns the number of IDs dispensed.
    pub fn count(&self) -> u32 {
        self.next
    }
}
/// A key-value annotation table for arbitrary metadata.
#[allow(dead_code)]
pub struct AnnotationTable {
    map: std::collections::HashMap<String, Vec<String>>,
}
#[allow(dead_code)]
impl AnnotationTable {
    /// Creates an empty annotation table.
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }
    /// Adds an annotation value for the given key.
    pub fn annotate(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.map.entry(key.into()).or_default().push(val.into());
    }
    /// Returns all annotations for `key`.
    pub fn get_all(&self, key: &str) -> &[String] {
        self.map.get(key).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Returns the number of distinct annotation keys.
    pub fn num_keys(&self) -> usize {
        self.map.len()
    }
    /// Returns `true` if the table has any annotations for `key`.
    pub fn has(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }
}
/// Tracks the frequency of items.
#[allow(dead_code)]
pub struct FrequencyTable<T: std::hash::Hash + Eq + Clone> {
    counts: std::collections::HashMap<T, u64>,
}
#[allow(dead_code)]
impl<T: std::hash::Hash + Eq + Clone> FrequencyTable<T> {
    /// Creates a new empty frequency table.
    pub fn new() -> Self {
        Self {
            counts: std::collections::HashMap::new(),
        }
    }
    /// Records one occurrence of `item`.
    pub fn record(&mut self, item: T) {
        *self.counts.entry(item).or_insert(0) += 1;
    }
    /// Returns the frequency of `item`.
    pub fn freq(&self, item: &T) -> u64 {
        self.counts.get(item).copied().unwrap_or(0)
    }
    /// Returns the item with the highest frequency.
    pub fn most_frequent(&self) -> Option<(&T, u64)> {
        self.counts
            .iter()
            .max_by_key(|(_, &v)| v)
            .map(|(k, &v)| (k, v))
    }
    /// Returns the total number of recordings.
    pub fn total(&self) -> u64 {
        self.counts.values().sum()
    }
    /// Returns the number of distinct items.
    pub fn distinct(&self) -> usize {
        self.counts.len()
    }
}
/// A builder for `PrettyDoc` values with a fluent API.
#[allow(dead_code)]
pub struct DocBuilder {
    doc: PrettyDoc,
}
#[allow(dead_code)]
impl DocBuilder {
    /// Creates a builder from an existing doc.
    pub fn from(doc: PrettyDoc) -> Self {
        Self { doc }
    }
    /// Creates a text builder.
    pub fn text(s: impl Into<String>) -> Self {
        Self {
            doc: PrettyDoc::text(s),
        }
    }
    /// Creates an empty builder.
    pub fn empty() -> Self {
        Self {
            doc: PrettyDoc::Empty,
        }
    }
    /// Appends another document.
    pub fn then(self, other: PrettyDoc) -> Self {
        Self {
            doc: PrettyDoc::concat(self.doc, other),
        }
    }
    /// Appends a text string.
    pub fn then_text(self, s: impl Into<String>) -> Self {
        self.then(PrettyDoc::text(s))
    }
    /// Appends a newline.
    pub fn then_newline(self) -> Self {
        self.then(PrettyDoc::Newline)
    }
    /// Wraps in an indent.
    pub fn indented(self) -> Self {
        Self {
            doc: PrettyDoc::indent(self.doc),
        }
    }
    /// Returns the built document.
    pub fn build(self) -> PrettyDoc {
        self.doc
    }
}
/// A simple LIFO work queue.
#[allow(dead_code)]
pub struct WorkStack<T> {
    items: Vec<T>,
}
#[allow(dead_code)]
impl<T> WorkStack<T> {
    /// Creates a new empty stack.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// Pushes a work item.
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    /// Pops the next work item.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Returns the number of pending work items.
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
/// A memoized computation slot that stores a cached value.
#[allow(dead_code)]
pub struct MemoSlot<T: Clone> {
    cached: Option<T>,
}
#[allow(dead_code)]
impl<T: Clone> MemoSlot<T> {
    /// Creates an uncomputed memo slot.
    pub fn new() -> Self {
        Self { cached: None }
    }
    /// Returns the cached value, computing it with `f` if absent.
    pub fn get_or_compute(&mut self, f: impl FnOnce() -> T) -> &T {
        if self.cached.is_none() {
            self.cached = Some(f());
        }
        self.cached
            .as_ref()
            .expect("cached value must be initialized before access")
    }
    /// Invalidates the cached value.
    pub fn invalidate(&mut self) {
        self.cached = None;
    }
    /// Returns `true` if the value has been computed.
    pub fn is_cached(&self) -> bool {
        self.cached.is_some()
    }
}
/// A counted-access cache that tracks hit and miss statistics.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct StatCache<K: std::hash::Hash + Eq + Clone, V: Clone> {
    /// The inner LRU cache.
    pub inner: SimpleLruCache<K, V>,
    /// Number of cache hits.
    pub hits: u64,
    /// Number of cache misses.
    pub misses: u64,
}
#[allow(dead_code)]
impl<K: std::hash::Hash + Eq + Clone, V: Clone> StatCache<K, V> {
    /// Creates a new stat cache with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: SimpleLruCache::new(capacity),
            hits: 0,
            misses: 0,
        }
    }
    /// Performs a lookup, tracking hit/miss.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let result = self.inner.get(key);
        if result.is_some() {
            self.hits += 1;
        } else {
            self.misses += 1;
        }
        None
    }
    /// Inserts a key-value pair.
    pub fn put(&mut self, key: K, val: V) {
        self.inner.put(key, val);
    }
    /// Returns the hit rate.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
}
/// Helpers for string escaping in pretty output.
#[allow(dead_code)]
pub struct EscapeHelper;
#[allow(dead_code)]
impl EscapeHelper {
    /// Escapes `\n`, `\t`, `\\`, and `"` in `s`.
    pub fn escape_str(s: &str) -> String {
        let mut out = String::with_capacity(s.len() + 2);
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\t' => out.push_str("\\t"),
                _ => out.push(c),
            }
        }
        out.push('"');
        out
    }
    /// Unescapes a string that may contain `\\n`, `\\t`, `\\\\`, `\\"`.
    pub fn unescape_str(s: &str) -> String {
        let s = if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
            &s[1..s.len() - 1]
        } else {
            s
        };
        let mut out = String::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => out.push('\n'),
                    Some('t') => out.push('\t'),
                    Some('\\') => out.push('\\'),
                    Some('"') => out.push('"'),
                    Some(x) => {
                        out.push('\\');
                        out.push(x);
                    }
                    None => out.push('\\'),
                }
            } else {
                out.push(c);
            }
        }
        out
    }
}
/// A simple event counter with named events.
#[allow(dead_code)]
pub struct EventCounter {
    counts: std::collections::HashMap<String, u64>,
}
#[allow(dead_code)]
impl EventCounter {
    /// Creates a new empty counter.
    pub fn new() -> Self {
        Self {
            counts: std::collections::HashMap::new(),
        }
    }
    /// Increments the counter for `event`.
    pub fn inc(&mut self, event: &str) {
        *self.counts.entry(event.to_string()).or_insert(0) += 1;
    }
    /// Adds `n` to the counter for `event`.
    pub fn add(&mut self, event: &str, n: u64) {
        *self.counts.entry(event.to_string()).or_insert(0) += n;
    }
    /// Returns the count for `event`.
    pub fn get(&self, event: &str) -> u64 {
        self.counts.get(event).copied().unwrap_or(0)
    }
    /// Returns the total count across all events.
    pub fn total(&self) -> u64 {
        self.counts.values().sum()
    }
    /// Resets all counters.
    pub fn reset(&mut self) {
        self.counts.clear();
    }
    /// Returns all event names.
    pub fn event_names(&self) -> Vec<&str> {
        self.counts.keys().map(|s| s.as_str()).collect()
    }
}
/// A bidirectional map between two types.
#[allow(dead_code)]
pub struct BiMap<A: std::hash::Hash + Eq + Clone, B: std::hash::Hash + Eq + Clone> {
    forward: std::collections::HashMap<A, B>,
    backward: std::collections::HashMap<B, A>,
}
#[allow(dead_code)]
impl<A: std::hash::Hash + Eq + Clone, B: std::hash::Hash + Eq + Clone> BiMap<A, B> {
    /// Creates an empty bidirectional map.
    pub fn new() -> Self {
        Self {
            forward: std::collections::HashMap::new(),
            backward: std::collections::HashMap::new(),
        }
    }
    /// Inserts a pair `(a, b)`.
    pub fn insert(&mut self, a: A, b: B) {
        self.forward.insert(a.clone(), b.clone());
        self.backward.insert(b, a);
    }
    /// Looks up `b` for a given `a`.
    pub fn get_b(&self, a: &A) -> Option<&B> {
        self.forward.get(a)
    }
    /// Looks up `a` for a given `b`.
    pub fn get_a(&self, b: &B) -> Option<&A> {
        self.backward.get(b)
    }
    /// Returns the number of pairs.
    pub fn len(&self) -> usize {
        self.forward.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.forward.is_empty()
    }
}
/// A slot that can hold a value, with lazy initialization.
#[allow(dead_code)]
pub struct Slot<T> {
    inner: Option<T>,
}
#[allow(dead_code)]
impl<T> Slot<T> {
    /// Creates an empty slot.
    pub fn empty() -> Self {
        Self { inner: None }
    }
    /// Fills the slot with `val`.  Panics if already filled.
    pub fn fill(&mut self, val: T) {
        assert!(self.inner.is_none(), "Slot: already filled");
        self.inner = Some(val);
    }
    /// Returns the slot's value, or `None`.
    pub fn get(&self) -> Option<&T> {
        self.inner.as_ref()
    }
    /// Returns `true` if the slot is filled.
    pub fn is_filled(&self) -> bool {
        self.inner.is_some()
    }
    /// Takes the value out of the slot.
    pub fn take(&mut self) -> Option<T> {
        self.inner.take()
    }
    /// Fills the slot if empty, returning a reference to the value.
    pub fn get_or_fill_with(&mut self, f: impl FnOnce() -> T) -> &T {
        if self.inner.is_none() {
            self.inner = Some(f());
        }
        self.inner
            .as_ref()
            .expect("inner value must be initialized before access")
    }
}
/// Interns strings to save memory (each unique string stored once).
#[allow(dead_code)]
pub struct StringInterner {
    strings: Vec<String>,
    map: std::collections::HashMap<String, u32>,
}
#[allow(dead_code)]
impl StringInterner {
    /// Creates a new string interner.
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            map: std::collections::HashMap::new(),
        }
    }
    /// Interns `s` and returns its ID.
    pub fn intern(&mut self, s: &str) -> u32 {
        if let Some(&id) = self.map.get(s) {
            return id;
        }
        let id = self.strings.len() as u32;
        self.strings.push(s.to_string());
        self.map.insert(s.to_string(), id);
        id
    }
    /// Returns the string for `id`.
    pub fn get(&self, id: u32) -> Option<&str> {
        self.strings.get(id as usize).map(|s| s.as_str())
    }
    /// Returns the total number of interned strings.
    pub fn len(&self) -> usize {
        self.strings.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
}
/// Implements Wadler/Lindig-style pretty printing using a box model.
#[allow(dead_code)]
pub struct BoxedPrinter {
    width: usize,
}
#[allow(dead_code)]
impl BoxedPrinter {
    /// Creates a printer with the given line width.
    pub fn new(width: usize) -> Self {
        Self { width }
    }
    /// Renders a `PrettyDoc` to a `String`.
    pub fn render(&self, doc: &PrettyDoc) -> String {
        doc.render(self.width, IndentStyle::Spaces(2))
    }
}
/// Configuration for pretty printing.
#[derive(Debug, Clone)]
pub struct PrintConfig {
    /// Use unicode symbols (λ, ∀, →)
    pub unicode: bool,
    /// Show implicit arguments
    pub show_implicit: bool,
    /// Show universe levels
    pub show_universes: bool,
    /// Maximum line width for wrapping
    pub max_width: usize,
    /// Show binder info (default, implicit, etc.)
    pub show_binder_info: bool,
    /// Show de Bruijn indices
    pub show_indices: bool,
}
impl PrintConfig {
    /// ASCII-only configuration.
    pub fn ascii() -> Self {
        Self {
            unicode: false,
            ..Default::default()
        }
    }
    /// Verbose configuration showing everything.
    pub fn verbose() -> Self {
        Self {
            show_implicit: true,
            show_universes: true,
            show_binder_info: true,
            show_indices: true,
            ..Default::default()
        }
    }
}
/// Configuration for the pretty printer.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct PrettyConfig {
    /// Maximum line width.
    pub width: usize,
    /// Indentation style.
    pub indent: IndentStyle,
    /// Whether to use color.
    pub colored: bool,
}
#[allow(dead_code)]
impl PrettyConfig {
    /// Default config: 80 columns, 2-space indent, no color.
    pub fn default_config() -> Self {
        Self {
            width: 80,
            indent: IndentStyle::Spaces(2),
            colored: false,
        }
    }
    /// Creates a compact single-line config (no newlines honoured).
    pub fn compact() -> Self {
        Self {
            width: usize::MAX,
            indent: IndentStyle::Spaces(0),
            colored: false,
        }
    }
}
/// A simple sparse bit set.
#[allow(dead_code)]
pub struct SparseBitSet {
    words: Vec<u64>,
}
#[allow(dead_code)]
impl SparseBitSet {
    /// Creates a new bit set that can hold at least `capacity` bits.
    pub fn new(capacity: usize) -> Self {
        let words = (capacity + 63) / 64;
        Self {
            words: vec![0u64; words],
        }
    }
    /// Sets bit `i`.
    pub fn set(&mut self, i: usize) {
        let word = i / 64;
        let bit = i % 64;
        if word < self.words.len() {
            self.words[word] |= 1u64 << bit;
        }
    }
    /// Clears bit `i`.
    pub fn clear(&mut self, i: usize) {
        let word = i / 64;
        let bit = i % 64;
        if word < self.words.len() {
            self.words[word] &= !(1u64 << bit);
        }
    }
    /// Returns `true` if bit `i` is set.
    pub fn get(&self, i: usize) -> bool {
        let word = i / 64;
        let bit = i % 64;
        self.words.get(word).is_some_and(|w| w & (1u64 << bit) != 0)
    }
    /// Returns the number of set bits.
    pub fn count_ones(&self) -> u32 {
        self.words.iter().map(|w| w.count_ones()).sum()
    }
    /// Returns the union with another bit set.
    pub fn union(&self, other: &SparseBitSet) -> SparseBitSet {
        let len = self.words.len().max(other.words.len());
        let mut result = SparseBitSet {
            words: vec![0u64; len],
        };
        for i in 0..self.words.len() {
            result.words[i] |= self.words[i];
        }
        for i in 0..other.words.len() {
            result.words[i] |= other.words[i];
        }
        result
    }
}
/// A simple stack-based scope tracker.
#[allow(dead_code)]
pub struct ScopeStack {
    names: Vec<String>,
}
#[allow(dead_code)]
impl ScopeStack {
    /// Creates a new empty scope stack.
    pub fn new() -> Self {
        Self { names: Vec::new() }
    }
    /// Pushes a scope name.
    pub fn push(&mut self, name: impl Into<String>) {
        self.names.push(name.into());
    }
    /// Pops the current scope.
    pub fn pop(&mut self) -> Option<String> {
        self.names.pop()
    }
    /// Returns the current (innermost) scope name, or `None`.
    pub fn current(&self) -> Option<&str> {
        self.names.last().map(|s| s.as_str())
    }
    /// Returns the depth of the scope stack.
    pub fn depth(&self) -> usize {
        self.names.len()
    }
    /// Returns `true` if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
    /// Returns the full path as a dot-separated string.
    pub fn path(&self) -> String {
        self.names.join(".")
    }
}
/// A monotonic timestamp in microseconds.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(u64);
#[allow(dead_code)]
impl Timestamp {
    /// Creates a timestamp from microseconds.
    pub const fn from_us(us: u64) -> Self {
        Self(us)
    }
    /// Returns the timestamp in microseconds.
    pub fn as_us(self) -> u64 {
        self.0
    }
    /// Returns the duration between two timestamps.
    pub fn elapsed_since(self, earlier: Timestamp) -> u64 {
        self.0.saturating_sub(earlier.0)
    }
}
/// A clock that measures elapsed time in a loop.
#[allow(dead_code)]
pub struct LoopClock {
    start: std::time::Instant,
    iters: u64,
}
#[allow(dead_code)]
impl LoopClock {
    /// Starts the clock.
    pub fn start() -> Self {
        Self {
            start: std::time::Instant::now(),
            iters: 0,
        }
    }
    /// Records one iteration.
    pub fn tick(&mut self) {
        self.iters += 1;
    }
    /// Returns the elapsed time in microseconds.
    pub fn elapsed_us(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1e6
    }
    /// Returns the average microseconds per iteration.
    pub fn avg_us_per_iter(&self) -> f64 {
        if self.iters == 0 {
            return 0.0;
        }
        self.elapsed_us() / self.iters as f64
    }
    /// Returns the number of iterations.
    pub fn iters(&self) -> u64 {
        self.iters
    }
}
/// A stream of pretty-printable tokens.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum PrettyToken {
    /// A keyword.
    Keyword(String),
    /// An identifier.
    Ident(String),
    /// A literal (number, string, etc.).
    Literal(String),
    /// An operator or punctuation.
    Operator(String),
    /// Whitespace.
    Space,
    /// A line break.
    LineBreak,
}
#[allow(dead_code)]
impl PrettyToken {
    /// Returns the raw text of the token (no color).
    pub fn raw_text(&self) -> &str {
        match self {
            PrettyToken::Keyword(s) => s.as_str(),
            PrettyToken::Ident(s) => s.as_str(),
            PrettyToken::Literal(s) => s.as_str(),
            PrettyToken::Operator(s) => s.as_str(),
            PrettyToken::Space => " ",
            PrettyToken::LineBreak => "\n",
        }
    }
    /// Returns colored text using the given color scheme.
    pub fn colored_text(&self, cs: &ColorScheme) -> String {
        match self {
            PrettyToken::Keyword(s) => format!("{}{}{}", cs.keyword, s, cs.reset),
            PrettyToken::Ident(s) => format!("{}{}{}", cs.ident, s, cs.reset),
            PrettyToken::Literal(s) => format!("{}{}{}", cs.literal, s, cs.reset),
            PrettyToken::Operator(s) => format!("{}{}{}", cs.operator, s, cs.reset),
            PrettyToken::Space => " ".to_string(),
            PrettyToken::LineBreak => "\n".to_string(),
        }
    }
}
/// Describes the indentation style used by the pretty printer.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndentStyle {
    /// Indent using `n` space characters.
    Spaces(usize),
    /// Indent using a single tab character.
    Tabs,
}
#[allow(dead_code)]
impl IndentStyle {
    /// Returns the indentation string for one level.
    pub fn one_level(self) -> String {
        match self {
            IndentStyle::Spaces(n) => " ".repeat(n),
            IndentStyle::Tabs => "\t".to_string(),
        }
    }
    /// Returns the indentation string for `depth` levels.
    pub fn for_depth(self, depth: usize) -> String {
        self.one_level().repeat(depth)
    }
}
/// A set of ANSI color codes for syntax-highlighted pretty printing.
#[allow(dead_code)]
pub struct ColorScheme {
    /// Color for keywords.
    pub keyword: &'static str,
    /// Color for identifiers.
    pub ident: &'static str,
    /// Color for literals.
    pub literal: &'static str,
    /// Color for operators.
    pub operator: &'static str,
    /// Reset color.
    pub reset: &'static str,
}
#[allow(dead_code)]
impl ColorScheme {
    /// The default ANSI color scheme.
    pub const DEFAULT: ColorScheme = ColorScheme {
        keyword: "\x1b[34m",
        ident: "\x1b[0m",
        literal: "\x1b[32m",
        operator: "\x1b[33m",
        reset: "\x1b[0m",
    };
    /// A monochrome (no color) scheme.
    pub const MONO: ColorScheme = ColorScheme {
        keyword: "",
        ident: "",
        literal: "",
        operator: "",
        reset: "",
    };
    /// Returns `true` if this is a colored scheme.
    pub fn is_colored(&self) -> bool {
        !self.keyword.is_empty()
    }
}
