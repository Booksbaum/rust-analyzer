use ra_syntax::{
    algo::{
        visit::{visitor_ctx, VisitorCtx}
    },
    ast,
    AstNode,
};
use rustc_hash::{FxHashMap};

use crate::{
    completion::{CompletionContext, Completions, CompletionKind, CompletionItem},
};

/// Complete repeated parametes, both name and type. For example, if all
/// functions in a file have a `spam: &mut Spam` parameter, a completion with
/// `spam: &mut Spam` insert text/label and `spam` lookup string will be
/// suggested.
pub(super) fn complete_fn_param(acc: &mut Completions, ctx: &CompletionContext) {
    if !ctx.is_param {
        return;
    }

    let mut params = FxHashMap::default();
    for node in ctx.leaf.ancestors() {
        let _ = visitor_ctx(&mut params)
            .visit::<ast::SourceFile, _>(process)
            .visit::<ast::ItemList, _>(process)
            .accept(node);
    }
    params
        .into_iter()
        .filter_map(|(label, (count, param))| {
            let lookup = param.pat()?.syntax().text().to_string();
            if count < 2 {
                None
            } else {
                Some((label, lookup))
            }
        })
        .for_each(|(label, lookup)| {
            CompletionItem::new(label)
                .lookup_by(lookup)
                .kind(CompletionKind::Magic)
                .add_to(acc)
        });

    fn process<'a, N: ast::FnDefOwner<'a>>(
        node: N,
        params: &mut FxHashMap<String, (u32, ast::Param<'a>)>,
    ) {
        node.functions()
            .filter_map(|it| it.param_list())
            .flat_map(|it| it.params())
            .for_each(|param| {
                let text = param.syntax().text().to_string();
                params.entry(text).or_insert((0, param)).0 += 1;
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::completion::*;

    fn check_magic_completion(code: &str, expected_completions: &str) {
        check_completion(code, expected_completions, CompletionKind::Magic);
    }

    #[test]
    fn test_param_completion_last_param() {
        check_magic_completion(
            r"
            fn foo(file_id: FileId) {}
            fn bar(file_id: FileId) {}
            fn baz(file<|>) {}
            ",
            r#"file_id "file_id: FileId""#,
        );
    }

    #[test]
    fn test_param_completion_nth_param() {
        check_magic_completion(
            r"
            fn foo(file_id: FileId) {}
            fn bar(file_id: FileId) {}
            fn baz(file<|>, x: i32) {}
            ",
            r#"file_id "file_id: FileId""#,
        );
    }

    #[test]
    fn test_param_completion_trait_param() {
        check_magic_completion(
            r"
            pub(crate) trait SourceRoot {
                pub fn contains(&self, file_id: FileId) -> bool;
                pub fn module_map(&self) -> &ModuleMap;
                pub fn lines(&self, file_id: FileId) -> &LineIndex;
                pub fn syntax(&self, file<|>)
            }
            ",
            r#"file_id "file_id: FileId""#,
        );
    }
}
