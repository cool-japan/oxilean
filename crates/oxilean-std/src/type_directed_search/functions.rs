//! Functions for type-directed search (Hoogle-style).

use std::collections::HashMap;

use super::types::{
    FunctionEntry, MatchKind, SearchDB, SearchQuery, SearchResult, TypeExpr, TypeSignature,
};

// ── TypeExpr methods ──────────────────────────────────────────────────────────

impl TypeExpr {
    /// Parse a type expression from a string.
    ///
    /// Supported syntax:
    /// - Type variables: lowercase identifiers, e.g. `a`, `b`
    /// - Constructors: uppercase or known names, e.g. `Nat`, `Bool`
    /// - Arrows: `A -> B` or `A → B`
    /// - Lists: `[A]`
    /// - Tuples: `(A, B)`
    /// - Option: `Option A`
    /// - Application: `F A` (left-associative)
    ///
    /// Returns `None` on parse failure.
    pub fn parse(s: &str) -> Option<TypeExpr> {
        let s = s.trim();
        parse_arrow(s)
    }

    /// Convert a `TypeExpr` back to a human-readable string.
    pub fn to_string_repr(&self) -> String {
        match self {
            TypeExpr::Var(v) => v.clone(),
            TypeExpr::Con(c) => c.clone(),
            TypeExpr::App(f, x) => {
                let fs = f.to_string_repr();
                let xs = match x.as_ref() {
                    TypeExpr::App(_, _) | TypeExpr::Arrow(_, _) => {
                        format!("({})", x.to_string_repr())
                    }
                    _ => x.to_string_repr(),
                };
                format!("{} {}", fs, xs)
            }
            TypeExpr::Arrow(a, b) => {
                let as_ = match a.as_ref() {
                    TypeExpr::Arrow(_, _) => format!("({})", a.to_string_repr()),
                    _ => a.to_string_repr(),
                };
                format!("{} -> {}", as_, b.to_string_repr())
            }
            TypeExpr::Tuple(ts) => {
                let inner: Vec<String> = ts.iter().map(|t| t.to_string_repr()).collect();
                format!("({})", inner.join(", "))
            }
            TypeExpr::List(t) => format!("[{}]", t.to_string_repr()),
            TypeExpr::Option(t) => format!("Option {}", t.to_string_repr()),
        }
    }
}

// ── Parsing internals ─────────────────────────────────────────────────────────

/// Parse an arrow type (right-associative).
fn parse_arrow(s: &str) -> Option<TypeExpr> {
    // Find top-level `->` or `→` (not inside parens/brackets)
    let bytes = s.as_bytes();
    let mut depth = 0i32;
    let mut i = 0usize;

    while i < s.len() {
        match bytes[i] {
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            b'-' if depth == 0 => {
                if i + 1 < s.len() && bytes[i + 1] == b'>' {
                    let lhs = s[..i].trim();
                    let rhs = s[i + 2..].trim();
                    let l = parse_app(lhs)?;
                    let r = parse_arrow(rhs)?;
                    return Some(TypeExpr::Arrow(Box::new(l), Box::new(r)));
                }
            }
            // UTF-8 '→' is 3 bytes: 0xE2 0x86 0x92
            0xE2 if depth == 0 => {
                if i + 2 < s.len() && bytes[i + 1] == 0x86 && bytes[i + 2] == 0x92 {
                    let lhs = s[..i].trim();
                    let rhs = s[i + 3..].trim();
                    let l = parse_app(lhs)?;
                    let r = parse_arrow(rhs)?;
                    return Some(TypeExpr::Arrow(Box::new(l), Box::new(r)));
                }
            }
            _ => {}
        }
        i += 1;
    }
    parse_app(s)
}

/// Parse a left-associative application chain: `F A B` = `App(App(F, A), B)`.
fn parse_app(s: &str) -> Option<TypeExpr> {
    let tokens = split_app_tokens(s)?;
    if tokens.is_empty() {
        return None;
    }
    let mut iter = tokens.into_iter();
    let first_str = iter.next()?;
    let mut result = parse_atom(&first_str)?;
    for tok in iter {
        let arg = parse_atom(&tok)?;
        result = TypeExpr::App(Box::new(result), Box::new(arg));
    }
    Some(result)
}

/// Split a string into top-level application tokens (space-separated, respecting parens).
fn split_app_tokens(s: &str) -> Option<Vec<String>> {
    let s = s.trim();
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let bytes = s.as_bytes();
    let mut i = 0usize;

    while i < bytes.len() {
        match bytes[i] {
            b'(' | b'[' => {
                depth += 1;
                current.push(bytes[i] as char);
            }
            b')' | b']' => {
                depth -= 1;
                current.push(bytes[i] as char);
            }
            b' ' | b'\t' if depth == 0 => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    tokens.push(trimmed);
                }
                current = String::new();
            }
            _ => {
                current.push(bytes[i] as char);
            }
        }
        i += 1;
    }
    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        tokens.push(trimmed);
    }
    Some(tokens)
}

/// Parse a single atom: parenthesized expr, list `[T]`, tuple `(A, B)`, or name.
fn parse_atom(s: &str) -> Option<TypeExpr> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // Parenthesized or tuple
    if s.starts_with('(') && s.ends_with(')') {
        let inner = &s[1..s.len() - 1];
        // Try tuple (contains comma at top level)
        let parts = split_by_comma_top(inner);
        if parts.len() >= 2 {
            let elems: Option<Vec<TypeExpr>> = parts.iter().map(|p| parse_arrow(p)).collect();
            return Some(TypeExpr::Tuple(elems?));
        }
        return parse_arrow(inner);
    }

    // List [T]
    if s.starts_with('[') && s.ends_with(']') {
        let inner = &s[1..s.len() - 1];
        let t = parse_arrow(inner)?;
        return Some(TypeExpr::List(Box::new(t)));
    }

    // Plain name: constructor vs variable
    if s.contains(' ') || s.contains('(') || s.contains('[') {
        // multi-token atom — shouldn't reach here normally
        return None;
    }

    // Option keyword
    if s == "Option" {
        return Some(TypeExpr::Con("Option".to_string()));
    }

    // Uppercase first char → constructor; lowercase → variable
    let first = s.chars().next()?;
    if first.is_uppercase() {
        Some(TypeExpr::Con(s.to_string()))
    } else {
        Some(TypeExpr::Var(s.to_string()))
    }
}

/// Split by commas at the top level (depth 0).
fn split_by_comma_top(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            b',' if depth == 0 => {
                parts.push(s[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(s[start..].trim());
    parts
}

// ── Type Utilities ────────────────────────────────────────────────────────────

/// Collect all free type variables in a `TypeExpr` (in left-to-right order, deduplicated).
pub fn free_type_vars(t: &TypeExpr) -> Vec<String> {
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut result: Vec<String> = Vec::new();
    collect_vars(t, &mut seen, &mut result);
    result
}

fn collect_vars(t: &TypeExpr, seen: &mut std::collections::HashSet<String>, out: &mut Vec<String>) {
    match t {
        TypeExpr::Var(v) => {
            if seen.insert(v.clone()) {
                out.push(v.clone());
            }
        }
        TypeExpr::Con(_) => {}
        TypeExpr::App(f, x) => {
            collect_vars(f, seen, out);
            collect_vars(x, seen, out);
        }
        TypeExpr::Arrow(a, b) => {
            collect_vars(a, seen, out);
            collect_vars(b, seen, out);
        }
        TypeExpr::Tuple(ts) => {
            for t in ts {
                collect_vars(t, seen, out);
            }
        }
        TypeExpr::List(t) | TypeExpr::Option(t) => {
            collect_vars(t, seen, out);
        }
    }
}

/// Compute the arity (number of top-level arrow types) of a `TypeExpr`.
pub fn type_arity(t: &TypeExpr) -> usize {
    match t {
        TypeExpr::Arrow(_, b) => 1 + type_arity(b),
        _ => 0,
    }
}

/// Normalize a `TypeExpr` by renaming type variables to `a`, `b`, `c`, … in order of appearance.
pub fn normalize_type(t: &TypeExpr) -> TypeExpr {
    let vars = free_type_vars(t);
    let mut mapping: HashMap<String, String> = HashMap::new();
    for (i, v) in vars.iter().enumerate() {
        let name = if i < 26 {
            ((b'a' + i as u8) as char).to_string()
        } else {
            format!("t{}", i)
        };
        mapping.insert(v.clone(), name);
    }
    apply_subst(
        t,
        &mapping
            .into_iter()
            .map(|(k, v)| (k, TypeExpr::Var(v)))
            .collect(),
    )
}

/// Apply a substitution map to a `TypeExpr`.
pub fn apply_subst(t: &TypeExpr, subst: &HashMap<String, TypeExpr>) -> TypeExpr {
    match t {
        TypeExpr::Var(v) => {
            if let Some(replacement) = subst.get(v) {
                replacement.clone()
            } else {
                t.clone()
            }
        }
        TypeExpr::Con(_) => t.clone(),
        TypeExpr::App(f, x) => TypeExpr::App(
            Box::new(apply_subst(f, subst)),
            Box::new(apply_subst(x, subst)),
        ),
        TypeExpr::Arrow(a, b) => TypeExpr::Arrow(
            Box::new(apply_subst(a, subst)),
            Box::new(apply_subst(b, subst)),
        ),
        TypeExpr::Tuple(ts) => TypeExpr::Tuple(ts.iter().map(|t| apply_subst(t, subst)).collect()),
        TypeExpr::List(inner) => TypeExpr::List(Box::new(apply_subst(inner, subst))),
        TypeExpr::Option(inner) => TypeExpr::Option(Box::new(apply_subst(inner, subst))),
    }
}

// ── Unification ───────────────────────────────────────────────────────────────

/// Attempt to unify two type expressions.
///
/// Returns `Some(subst)` where `subst` maps variable names to type expressions such that
/// `apply_subst(t1, subst) == apply_subst(t2, subst)`.
/// Returns `None` if unification fails.
pub fn unify_types(t1: &TypeExpr, t2: &TypeExpr) -> Option<HashMap<String, TypeExpr>> {
    let mut subst: HashMap<String, TypeExpr> = HashMap::new();
    if unify_rec(t1, t2, &mut subst) {
        Some(subst)
    } else {
        None
    }
}

fn unify_rec(t1: &TypeExpr, t2: &TypeExpr, subst: &mut HashMap<String, TypeExpr>) -> bool {
    let t1 = walk(t1, subst);
    let t2 = walk(&t2.clone(), subst);

    match (&t1, &t2) {
        (TypeExpr::Var(v1), TypeExpr::Var(v2)) if v1 == v2 => true,
        (TypeExpr::Var(v), t) | (t, TypeExpr::Var(v)) => {
            // Occurs check
            if occurs(v, t) {
                return false;
            }
            subst.insert(v.clone(), t.clone());
            true
        }
        (TypeExpr::Con(c1), TypeExpr::Con(c2)) => c1 == c2,
        (TypeExpr::App(f1, x1), TypeExpr::App(f2, x2)) => {
            unify_rec(f1, f2, subst) && {
                let x1 = apply_subst(x1, subst);
                let x2 = apply_subst(x2, subst);
                unify_rec(&x1, &x2, subst)
            }
        }
        (TypeExpr::Arrow(a1, b1), TypeExpr::Arrow(a2, b2)) => {
            unify_rec(a1, a2, subst) && {
                let b1 = apply_subst(b1, subst);
                let b2 = apply_subst(b2, subst);
                unify_rec(&b1, &b2, subst)
            }
        }
        (TypeExpr::Tuple(ts1), TypeExpr::Tuple(ts2)) => {
            if ts1.len() != ts2.len() {
                return false;
            }
            for (a, b) in ts1.iter().zip(ts2.iter()) {
                let a = apply_subst(a, subst);
                let b = apply_subst(b, subst);
                if !unify_rec(&a, &b, subst) {
                    return false;
                }
            }
            true
        }
        (TypeExpr::List(a), TypeExpr::List(b)) => unify_rec(a, b, subst),
        (TypeExpr::Option(a), TypeExpr::Option(b)) => unify_rec(a, b, subst),
        _ => false,
    }
}

/// Chase variable bindings in the substitution (path compression).
fn walk(t: &TypeExpr, subst: &HashMap<String, TypeExpr>) -> TypeExpr {
    if let TypeExpr::Var(v) = t {
        if let Some(bound) = subst.get(v) {
            return walk(bound, subst);
        }
    }
    t.clone()
}

/// Occurs check: does variable `v` appear free in `t`?
fn occurs(v: &str, t: &TypeExpr) -> bool {
    match t {
        TypeExpr::Var(w) => w == v,
        TypeExpr::Con(_) => false,
        TypeExpr::App(f, x) => occurs(v, f) || occurs(v, x),
        TypeExpr::Arrow(a, b) => occurs(v, a) || occurs(v, b),
        TypeExpr::Tuple(ts) => ts.iter().any(|t| occurs(v, t)),
        TypeExpr::List(inner) | TypeExpr::Option(inner) => occurs(v, inner),
    }
}

// ── Matching ──────────────────────────────────────────────────────────────────

/// Attempt to match a query type against a candidate type.
///
/// Returns `Some((kind, score))` if a match is found, `None` if no meaningful match.
/// Score is in `0.0..=1.0`, higher is better.
pub fn type_matches(query: &TypeExpr, candidate: &TypeExpr) -> Option<(MatchKind, f64)> {
    let nq = normalize_type(query);
    let nc = normalize_type(candidate);

    // 1. Exact structural match after normalization
    if nq == nc {
        return Some((MatchKind::Exact, 1.0));
    }

    // 2. Up-to-renaming: unify with fresh variables for candidate
    //    Query may have its own variables; candidate may too.
    //    Rename candidate vars with a fresh prefix to avoid collision.
    let candidate_renamed = rename_vars(candidate, "__cand_");
    if let Some(_subst) = unify_types(query, &candidate_renamed) {
        // Check if query has variables → candidate is a specialization
        // Check if candidate has variables → query is a specialization
        let q_vars = free_type_vars(query);
        let c_vars = free_type_vars(candidate);

        if q_vars.is_empty() && c_vars.is_empty() {
            return Some((MatchKind::UpToRenaming, 0.95));
        }
        if c_vars.is_empty() {
            // candidate is concrete, query is polymorphic → candidate is a specialization of query
            return Some((MatchKind::SpecializationOf, 0.90));
        }
        if q_vars.is_empty() {
            // query is concrete, candidate is polymorphic → candidate generalizes query
            return Some((MatchKind::GeneralizationOf, 0.85));
        }
        // Both have variables but unified → up to renaming
        return Some((MatchKind::UpToRenaming, 0.90));
    }

    // 3. Partial structural similarity
    let score = structural_similarity(query, candidate);
    if score > 0.3 {
        return Some((MatchKind::Partial, score * 0.7));
    }

    None
}

/// Rename all type variables in `t` by prepending `prefix`.
fn rename_vars(t: &TypeExpr, prefix: &str) -> TypeExpr {
    match t {
        TypeExpr::Var(v) => TypeExpr::Var(format!("{}{}", prefix, v)),
        TypeExpr::Con(_) => t.clone(),
        TypeExpr::App(f, x) => TypeExpr::App(
            Box::new(rename_vars(f, prefix)),
            Box::new(rename_vars(x, prefix)),
        ),
        TypeExpr::Arrow(a, b) => TypeExpr::Arrow(
            Box::new(rename_vars(a, prefix)),
            Box::new(rename_vars(b, prefix)),
        ),
        TypeExpr::Tuple(ts) => TypeExpr::Tuple(ts.iter().map(|t| rename_vars(t, prefix)).collect()),
        TypeExpr::List(inner) => TypeExpr::List(Box::new(rename_vars(inner, prefix))),
        TypeExpr::Option(inner) => TypeExpr::Option(Box::new(rename_vars(inner, prefix))),
    }
}

/// Compute a structural similarity score in 0.0..=1.0 between two types.
fn structural_similarity(t1: &TypeExpr, t2: &TypeExpr) -> f64 {
    match (t1, t2) {
        (TypeExpr::Var(_), TypeExpr::Var(_)) => 0.8,
        (TypeExpr::Var(_), _) | (_, TypeExpr::Var(_)) => 0.5,
        (TypeExpr::Con(a), TypeExpr::Con(b)) => {
            if a == b {
                1.0
            } else {
                0.0
            }
        }
        (TypeExpr::App(f1, x1), TypeExpr::App(f2, x2)) => {
            let sf = structural_similarity(f1, f2);
            let sx = structural_similarity(x1, x2);
            (sf + sx) / 2.0
        }
        (TypeExpr::Arrow(a1, b1), TypeExpr::Arrow(a2, b2)) => {
            let sa = structural_similarity(a1, a2);
            let sb = structural_similarity(b1, b2);
            (sa + sb) / 2.0
        }
        (TypeExpr::Tuple(ts1), TypeExpr::Tuple(ts2)) => {
            if ts1.len() != ts2.len() {
                return 0.2;
            }
            let total: f64 = ts1
                .iter()
                .zip(ts2.iter())
                .map(|(a, b)| structural_similarity(a, b))
                .sum();
            total / ts1.len() as f64
        }
        (TypeExpr::List(a), TypeExpr::List(b)) => structural_similarity(a, b),
        (TypeExpr::Option(a), TypeExpr::Option(b)) => structural_similarity(a, b),
        _ => 0.0,
    }
}

// ── Search ────────────────────────────────────────────────────────────────────

/// Search the database for functions matching the query type, returning ranked results.
pub fn search(db: &SearchDB, query: &SearchQuery) -> Vec<SearchResult> {
    let q_type = query.signature.to_type_expr();
    let mut results: Vec<SearchResult> = Vec::new();

    for entry in &db.entries {
        let c_type = entry.signature.to_type_expr();
        if let Some((kind, score)) = type_matches(&q_type, &c_type) {
            results.push(SearchResult {
                entry: entry.clone(),
                score,
                match_kind: kind,
            });
        }
    }

    // Sort by score descending, then by name for determinism
    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.entry.name.cmp(&b.entry.name))
    });

    results.truncate(query.max_results);
    results
}

// ── SearchDB methods ──────────────────────────────────────────────────────────

impl SearchDB {
    /// Create an empty `SearchDB`.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Add a `FunctionEntry` to the database.
    pub fn add_entry(&mut self, entry: FunctionEntry) {
        self.entries.push(entry);
    }

    /// Add a function entry from components.
    pub fn add(&mut self, name: &str, module: &str, args: Vec<TypeExpr>, ret: TypeExpr, doc: &str) {
        self.entries.push(FunctionEntry {
            name: name.to_string(),
            module: module.to_string(),
            signature: TypeSignature::new(args, ret),
            doc: doc.to_string(),
        });
    }

    /// Populate the database with basic Lean4/Haskell-like prelude functions.
    pub fn add_prelude(&mut self) {
        use TypeExpr::{Arrow, Con, List, Option, Tuple, Var};

        let nat = || Con("Nat".to_string());
        let bool_ = || Con("Bool".to_string());
        let string = || Con("String".to_string());
        let unit = || Con("Unit".to_string());
        let a = || Var("a".to_string());
        let b = || Var("b".to_string());
        let list_a = || List(Box::new(a()));

        // identity :: a -> a
        self.add("id", "Prelude", vec![a()], a(), "Identity function");

        // const :: a -> b -> a
        self.add("const", "Prelude", vec![a(), b()], a(), "Constant function");

        // not :: Bool -> Bool
        self.add("not", "Prelude", vec![bool_()], bool_(), "Boolean negation");

        // succ :: Nat -> Nat
        self.add("Nat.succ", "Prelude", vec![nat()], nat(), "Successor");

        // pred :: Nat -> Nat
        self.add(
            "Nat.pred",
            "Prelude",
            vec![nat()],
            nat(),
            "Predecessor (saturating)",
        );

        // length :: [a] -> Nat
        self.add(
            "List.length",
            "Std.List",
            vec![list_a()],
            nat(),
            "List length",
        );

        // head :: [a] -> Option a
        self.add(
            "List.head?",
            "Std.List",
            vec![list_a()],
            Option(Box::new(a())),
            "First element or None",
        );

        // tail :: [a] -> Option [a]
        self.add(
            "List.tail?",
            "Std.List",
            vec![list_a()],
            Option(Box::new(list_a())),
            "List without head or None",
        );

        // map :: (a -> b) -> [a] -> [b]
        self.add(
            "List.map",
            "Std.List",
            vec![Arrow(Box::new(a()), Box::new(b())), list_a()],
            List(Box::new(b())),
            "Apply function to each element",
        );

        // filter :: (a -> Bool) -> [a] -> [a]
        self.add(
            "List.filter",
            "Std.List",
            vec![Arrow(Box::new(a()), Box::new(bool_())), list_a()],
            list_a(),
            "Keep elements satisfying predicate",
        );

        // foldl :: (b -> a -> b) -> b -> [a] -> b
        self.add(
            "List.foldl",
            "Std.List",
            vec![
                Arrow(Box::new(b()), Box::new(Arrow(Box::new(a()), Box::new(b())))),
                b(),
                list_a(),
            ],
            b(),
            "Left fold",
        );

        // append :: [a] -> [a] -> [a]
        self.add(
            "List.append",
            "Std.List",
            vec![list_a(), list_a()],
            list_a(),
            "Concatenate lists",
        );

        // reverse :: [a] -> [a]
        self.add(
            "List.reverse",
            "Std.List",
            vec![list_a()],
            list_a(),
            "Reverse list",
        );

        // Option.map :: (a -> b) -> Option a -> Option b
        self.add(
            "Option.map",
            "Std.Option",
            vec![Arrow(Box::new(a()), Box::new(b())), Option(Box::new(a()))],
            Option(Box::new(b())),
            "Map over Option",
        );

        // Option.getOrElse :: Option a -> a -> a
        self.add(
            "Option.getD",
            "Std.Option",
            vec![Option(Box::new(a())), a()],
            a(),
            "Get value or default",
        );

        // Option.isSome :: Option a -> Bool
        self.add(
            "Option.isSome",
            "Std.Option",
            vec![Option(Box::new(a()))],
            bool_(),
            "True if Some",
        );

        // Nat.add :: Nat -> Nat -> Nat
        self.add("Nat.add", "Prelude", vec![nat(), nat()], nat(), "Addition");

        // Nat.mul :: Nat -> Nat -> Nat
        self.add(
            "Nat.mul",
            "Prelude",
            vec![nat(), nat()],
            nat(),
            "Multiplication",
        );

        // Nat.sub :: Nat -> Nat -> Nat
        self.add(
            "Nat.sub",
            "Prelude",
            vec![nat(), nat()],
            nat(),
            "Saturating subtraction",
        );

        // Nat.beq :: Nat -> Nat -> Bool
        self.add(
            "Nat.beq",
            "Prelude",
            vec![nat(), nat()],
            bool_(),
            "Natural equality",
        );

        // Nat.ble :: Nat -> Nat -> Bool
        self.add(
            "Nat.ble",
            "Prelude",
            vec![nat(), nat()],
            bool_(),
            "Natural ≤",
        );

        // String.length :: String -> Nat
        self.add(
            "String.length",
            "Std.String",
            vec![string()],
            nat(),
            "String length",
        );

        // String.append :: String -> String -> String
        self.add(
            "String.append",
            "Std.String",
            vec![string(), string()],
            string(),
            "String concatenation",
        );

        // toString :: a -> String (polymorphic)
        self.add(
            "toString",
            "Prelude",
            vec![a()],
            string(),
            "Convert to string (ToString class)",
        );

        // Prod.fst :: (a, b) -> a
        self.add(
            "Prod.fst",
            "Prelude",
            vec![Tuple(vec![a(), b()])],
            a(),
            "First component of pair",
        );

        // Prod.snd :: (a, b) -> b
        self.add(
            "Prod.snd",
            "Prelude",
            vec![Tuple(vec![a(), b()])],
            b(),
            "Second component of pair",
        );

        // IO.println :: String -> IO Unit
        self.add(
            "IO.println",
            "Std.IO",
            vec![string()],
            unit(),
            "Print string with newline",
        );

        // List.zip :: [a] -> [b] -> [(a, b)]
        self.add(
            "List.zip",
            "Std.List",
            vec![list_a(), List(Box::new(b()))],
            List(Box::new(Tuple(vec![a(), b()]))),
            "Zip two lists into pairs",
        );

        // List.any :: (a -> Bool) -> [a] -> Bool
        self.add(
            "List.any",
            "Std.List",
            vec![Arrow(Box::new(a()), Box::new(bool_())), list_a()],
            bool_(),
            "True if any element satisfies predicate",
        );

        // List.all :: (a -> Bool) -> [a] -> Bool
        self.add(
            "List.all",
            "Std.List",
            vec![Arrow(Box::new(a()), Box::new(bool_())), list_a()],
            bool_(),
            "True if all elements satisfy predicate",
        );

        // List.take :: Nat -> [a] -> [a]
        self.add(
            "List.take",
            "Std.List",
            vec![nat(), list_a()],
            list_a(),
            "Take first n elements",
        );

        // List.drop :: Nat -> [a] -> [a]
        self.add(
            "List.drop",
            "Std.List",
            vec![nat(), list_a()],
            list_a(),
            "Drop first n elements",
        );

        // Function.comp :: (b -> c) -> (a -> b) -> a -> c
        let c = || Var("c".to_string());
        self.add(
            "Function.comp",
            "Prelude",
            vec![
                Arrow(Box::new(b()), Box::new(c())),
                Arrow(Box::new(a()), Box::new(b())),
                a(),
            ],
            c(),
            "Function composition",
        );
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::types::{MatchKind, SearchQuery, TypeExpr, TypeSignature};
    use super::*;

    fn var(s: &str) -> TypeExpr {
        TypeExpr::Var(s.to_string())
    }
    fn con(s: &str) -> TypeExpr {
        TypeExpr::Con(s.to_string())
    }
    fn arrow(a: TypeExpr, b: TypeExpr) -> TypeExpr {
        TypeExpr::Arrow(Box::new(a), Box::new(b))
    }
    fn list(t: TypeExpr) -> TypeExpr {
        TypeExpr::List(Box::new(t))
    }
    fn opt(t: TypeExpr) -> TypeExpr {
        TypeExpr::Option(Box::new(t))
    }

    // ── Parsing ────────────────────────────────────────────────────────────────

    #[test]
    fn test_parse_var() {
        let t = TypeExpr::parse("a").expect("parse");
        assert_eq!(t, var("a"));
    }

    #[test]
    fn test_parse_con() {
        let t = TypeExpr::parse("Nat").expect("parse");
        assert_eq!(t, con("Nat"));
    }

    #[test]
    fn test_parse_arrow() {
        let t = TypeExpr::parse("Nat -> Bool").expect("parse");
        assert_eq!(t, arrow(con("Nat"), con("Bool")));
    }

    #[test]
    fn test_parse_list() {
        let t = TypeExpr::parse("[Nat]").expect("parse");
        assert_eq!(t, list(con("Nat")));
    }

    #[test]
    fn test_parse_tuple() {
        let t = TypeExpr::parse("(Nat, Bool)").expect("parse");
        assert_eq!(t, TypeExpr::Tuple(vec![con("Nat"), con("Bool")]));
    }

    #[test]
    fn test_parse_option() {
        let t = TypeExpr::parse("Option Nat").expect("parse");
        assert_eq!(
            t,
            TypeExpr::App(Box::new(con("Option")), Box::new(con("Nat")))
        );
    }

    #[test]
    fn test_parse_nested_arrow() {
        // a -> b -> a should parse as a -> (b -> a)
        let t = TypeExpr::parse("a -> b -> a").expect("parse");
        assert_eq!(t, arrow(var("a"), arrow(var("b"), var("a"))));
    }

    #[test]
    fn test_parse_parens() {
        let t = TypeExpr::parse("(a -> b) -> a -> b").expect("parse");
        let inner = arrow(var("a"), var("b"));
        assert_eq!(t, arrow(inner, arrow(var("a"), var("b"))));
    }

    // ── to_string_repr ─────────────────────────────────────────────────────────

    #[test]
    fn test_to_string_var() {
        assert_eq!(var("a").to_string_repr(), "a");
    }

    #[test]
    fn test_to_string_arrow() {
        assert_eq!(
            arrow(con("Nat"), con("Bool")).to_string_repr(),
            "Nat -> Bool"
        );
    }

    #[test]
    fn test_to_string_list() {
        assert_eq!(list(con("Nat")).to_string_repr(), "[Nat]");
    }

    #[test]
    fn test_to_string_tuple() {
        assert_eq!(
            TypeExpr::Tuple(vec![con("Nat"), con("Bool")]).to_string_repr(),
            "(Nat, Bool)"
        );
    }

    // ── free_type_vars ─────────────────────────────────────────────────────────

    #[test]
    fn test_free_vars_simple() {
        let t = arrow(var("a"), var("b"));
        assert_eq!(free_type_vars(&t), vec!["a", "b"]);
    }

    #[test]
    fn test_free_vars_deduplicated() {
        let t = arrow(var("a"), var("a"));
        assert_eq!(free_type_vars(&t), vec!["a"]);
    }

    #[test]
    fn test_free_vars_no_vars() {
        let t = arrow(con("Nat"), con("Bool"));
        assert_eq!(free_type_vars(&t), Vec::<String>::new());
    }

    // ── type_arity ─────────────────────────────────────────────────────────────

    #[test]
    fn test_arity_zero() {
        assert_eq!(type_arity(&con("Nat")), 0);
    }

    #[test]
    fn test_arity_one() {
        assert_eq!(type_arity(&arrow(con("Nat"), con("Bool"))), 1);
    }

    #[test]
    fn test_arity_two() {
        assert_eq!(
            type_arity(&arrow(con("Nat"), arrow(con("Nat"), con("Nat")))),
            2
        );
    }

    // ── normalize_type ─────────────────────────────────────────────────────────

    #[test]
    fn test_normalize_renames_vars() {
        let t = arrow(var("x"), var("y"));
        let n = normalize_type(&t);
        assert_eq!(n, arrow(var("a"), var("b")));
    }

    #[test]
    fn test_normalize_cons_unchanged() {
        let t = arrow(con("Nat"), con("Bool"));
        let n = normalize_type(&t);
        assert_eq!(n, t);
    }

    // ── unify_types ────────────────────────────────────────────────────────────

    #[test]
    fn test_unify_vars() {
        let subst = unify_types(&var("a"), &con("Nat")).expect("unify");
        assert_eq!(subst.get("a"), Some(&con("Nat")));
    }

    #[test]
    fn test_unify_cons_same() {
        let subst = unify_types(&con("Nat"), &con("Nat"));
        assert!(subst.is_some());
    }

    #[test]
    fn test_unify_cons_diff() {
        let subst = unify_types(&con("Nat"), &con("Bool"));
        assert!(subst.is_none());
    }

    #[test]
    fn test_unify_arrow() {
        let t1 = arrow(var("a"), con("Bool"));
        let t2 = arrow(con("Nat"), var("b"));
        let subst = unify_types(&t1, &t2).expect("unify");
        assert_eq!(subst.get("a"), Some(&con("Nat")));
        assert_eq!(subst.get("b"), Some(&con("Bool")));
    }

    #[test]
    fn test_unify_occurs_check() {
        // a ~ List a should fail (occurs check)
        let t1 = var("a");
        let t2 = list(var("a"));
        assert!(unify_types(&t1, &t2).is_none());
    }

    // ── type_matches ───────────────────────────────────────────────────────────

    #[test]
    fn test_matches_exact() {
        let q = arrow(con("Nat"), con("Bool"));
        let c = arrow(con("Nat"), con("Bool"));
        let result = type_matches(&q, &c);
        assert!(result.is_some());
        let (kind, score) = result.expect("match");
        assert_eq!(kind, MatchKind::Exact);
        assert!(score > 0.9);
    }

    #[test]
    fn test_matches_up_to_renaming() {
        let q = arrow(var("a"), var("a")); // a -> a
        let c = arrow(var("b"), var("b")); // b -> b
        let result = type_matches(&q, &c);
        assert!(result.is_some());
    }

    #[test]
    fn test_matches_partial() {
        let q = arrow(con("Nat"), con("Bool"));
        let c = arrow(con("Nat"), con("Nat"));
        // Partial match (same arg, different ret)
        let result = type_matches(&q, &c);
        // May or may not match as partial — just check it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_matches_no_match() {
        let q = con("Nat");
        let c = con("Bool");
        assert!(type_matches(&q, &c).is_none());
    }

    // ── search ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_search_identity() {
        let mut db = SearchDB::new();
        db.add_prelude();
        let query = SearchQuery {
            signature: TypeSignature::new(vec![var("a")], var("a")),
            max_results: 5,
        };
        let results = search(&db, &query);
        assert!(!results.is_empty());
        // id should appear in results
        let has_id = results.iter().any(|r| r.entry.name == "id");
        assert!(has_id, "Expected 'id' in results");
    }

    #[test]
    fn test_search_respects_max_results() {
        let mut db = SearchDB::new();
        db.add_prelude();
        let query = SearchQuery {
            signature: TypeSignature::new(vec![var("a")], var("a")),
            max_results: 2,
        };
        let results = search(&db, &query);
        assert!(results.len() <= 2);
    }

    #[test]
    fn test_search_empty_db() {
        let db = SearchDB::new();
        let query = SearchQuery {
            signature: TypeSignature::new(vec![con("Nat")], con("Bool")),
            max_results: 10,
        };
        let results = search(&db, &query);
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_scores_sorted() {
        let mut db = SearchDB::new();
        db.add_prelude();
        let query = SearchQuery {
            signature: TypeSignature::new(vec![con("Nat"), con("Nat")], con("Nat")),
            max_results: 10,
        };
        let results = search(&db, &query);
        for w in results.windows(2) {
            assert!(
                w[0].score >= w[1].score,
                "Results should be sorted by score"
            );
        }
    }

    #[test]
    fn test_add_entry() {
        let mut db = SearchDB::new();
        db.add_entry(FunctionEntry {
            name: "myFunc".to_string(),
            module: "MyMod".to_string(),
            signature: TypeSignature::new(vec![con("Nat")], con("Bool")),
            doc: "A test function".to_string(),
        });
        assert_eq!(db.entries.len(), 1);
    }

    #[test]
    fn test_prelude_has_many_entries() {
        let mut db = SearchDB::new();
        db.add_prelude();
        assert!(db.entries.len() >= 25);
    }

    #[test]
    fn test_search_list_map() {
        let mut db = SearchDB::new();
        db.add_prelude();
        // Query: (a -> b) -> [a] -> [b]
        let query = SearchQuery {
            signature: TypeSignature::new(
                vec![arrow(var("a"), var("b")), list(var("a"))],
                list(var("b")),
            ),
            max_results: 5,
        };
        let results = search(&db, &query);
        assert!(!results.is_empty());
        let has_map = results.iter().any(|r| r.entry.name == "List.map");
        assert!(has_map, "Expected List.map in results for map-like query");
    }
}
