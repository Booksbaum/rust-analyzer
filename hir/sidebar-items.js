initSidebarItems({"enum":[["Access",""],["Adt","A Data Type"],["AssocItem","Invariant: `inner.as_assoc_item(db).is_some()` We do not actively enforce this invariant."],["AssocItemContainer",""],["CallableKind",""],["DefWithBody","The defs which have a body."],["FieldSource",""],["GenericDef",""],["GenericParam",""],["ItemInNs",""],["MacroCallId","`MacroCallId` identifies a particular macro invocation, like `println!(\"Hello, {}\", world)`."],["MacroKind",""],["ModuleDef","The defs which can be visible in the module."],["ModuleSource",""],["Mutability",""],["Namespace",""],["Origin",""],["PathKind",""],["PathResolution",""],["PrefixKind",""],["ScopeDef","For IDE only"],["StructKind",""],["TypeRef","Compare ty::Ty"],["VariantDef",""],["Visibility","Visibility of an item, with the path resolved."]],"mod":[["db","FIXME: write short doc here"],["diagnostics","FIXME: write short doc here"],["import_map","A map of all publicly exported items in a crate."],["known",""]],"struct":[["Attr",""],["Attrs",""],["AttrsWithOwner",""],["BuiltinType",""],["Callable",""],["Const",""],["ConstParam",""],["Crate","hir::Crate describes a single crate. It’s the main interface with which a crate’s dependencies interact. Mostly, it should be just a proxy for the root module."],["CrateDependency",""],["Documentation","Holds documentation"],["Enum",""],["ExpandResult",""],["ExprScopes",""],["Field",""],["Function",""],["HirFileId","Input to the analyzer is a set of files, where each file is identified by `FileId` and contains source code. However, another source of source code in Rust are macros: each macro can be thought of as producing a “temporary file”. To assign an id to such a file, we use the id of the macro call that produced the file. So, a `HirFileId` is either a `FileId` (source code written by user), or a `MacroCallId` (source code produced by macro)."],["Impl",""],["InFile","`InFile<T>` stores a value of `T` inside a particular file/syntax tree."],["Label",""],["LifetimeParam",""],["Local",""],["MacroCallLoc",""],["MacroDef",""],["MacroDefId",""],["MacroFile",""],["ModPath",""],["Module",""],["Name","`Name` is a wrapper around string, which is used in hir for both references and declarations. In theory, names should also carry hygiene info, but we are not there yet!"],["Param",""],["SelfParam",""],["Semantics","Primary API to get semantic information, like types, from syntax trees."],["SemanticsScope","`SemanticScope` encapsulates the notion of a scope (the set of visible names) at a particular program point."],["Static",""],["Struct",""],["Trait",""],["Type",""],["TypeAlias",""],["TypeParam",""],["Union",""],["Variant",""]],"trait":[["AsAssocItem",""],["HasAttrs",""],["HasSource",""],["HasVisibility",""],["HirDisplay",""]]});