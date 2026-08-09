//! Cross-resource reference expressions.
//!
//! References are not a field type. A field declared `#[ref(tag)]` is a plain
//! `i32` (or `Vec<i32>`); the descriptor records its target type in
//! [`crate::FieldDescriptor::reference`]. At config time the value is a
//! `${ref.<type>.<key>}` template living in the JSON Value tree; the resolver
//! scans for those, builds the dependency graph from the [`RefExpr`]s, and
//! substitutes resolved ids in topological order before the resource is decoded
//! into its typed struct.

/// One entry in a resource's dependency list — a `(type, key)` pair parsed from
/// a `${ref.<type>.<key>}` expression in the desired-state Value tree.
///
/// Used to build the apply-order dep graph: an edge from `B` to `A` exists when
/// `A` references `B`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefExpr {
    pub target_type: String,
    pub key: String,
}
