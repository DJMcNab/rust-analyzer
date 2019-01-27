use crate::completion::{CompletionItem, Completions, CompletionKind, CompletionContext};

pub(super) fn complete_scope(acc: &mut Completions, ctx: &CompletionContext) {
    if !ctx.is_trivial_path {
        return;
    }
    let names = ctx.resolver.all_names();

    // let module_scope = module.scope(ctx.db);
    names
        .into_iter()
        // FIXME check tests
        // .filter(|(_name, res)| {
        //     // For cases like `use self::foo<|>` don't suggest foo itself.
        //     match res.import {
        //         None => true,
        //         Some(import) => {
        //             let source = module.import_source(ctx.db, import);
        //             !source.syntax().range().is_subrange(&ctx.leaf.range())
        //         }
        //     }
        // })
        .for_each(|(name, res)| {
            CompletionItem::new(
                CompletionKind::Reference,
                ctx.source_range(),
                name.to_string(),
            )
            .from_resolution(ctx, &res)
            .add_to(acc)
        });
}

#[cfg(test)]
mod tests {
    use crate::completion::CompletionKind;
    use crate::completion::completion_item::check_completion;

    fn check_reference_completion(name: &str, code: &str) {
        check_completion(name, code, CompletionKind::Reference);
    }

    #[test]
    fn completes_bindings_from_let() {
        check_reference_completion(
            "bindings_from_let",
            r"
            fn quux(x: i32) {
                let y = 92;
                1 + <|>;
                let z = ();
            }
            ",
        );
    }

    #[test]
    fn completes_bindings_from_if_let() {
        check_reference_completion(
            "bindings_from_if_let",
            r"
            fn quux() {
                if let Some(x) = foo() {
                    let y = 92;
                };
                if let Some(a) = bar() {
                    let b = 62;
                    1 + <|>
                }
            }
            ",
        );
    }

    #[test]
    fn completes_bindings_from_for() {
        check_reference_completion(
            "bindings_from_for",
            r"
            fn quux() {
                for x in &[1, 2, 3] {
                    <|>
                }
            }
            ",
        );
    }

    #[test]
    fn completes_module_items() {
        check_reference_completion(
            "module_items",
            r"
            struct Foo;
            enum Baz {}
            fn quux() {
                <|>
            }
            ",
        );
    }

    #[test]
    fn completes_module_items_in_nested_modules() {
        check_reference_completion(
            "module_items_in_nested_modules",
            r"
            struct Foo;
            mod m {
                struct Bar;
                fn quux() { <|> }
            }
            ",
        );
    }

    #[test]
    fn completes_return_type() {
        check_reference_completion(
            "return_type",
            r"
            struct Foo;
            fn x() -> <|>
            ",
        )
    }

    #[test]
    fn dont_show_both_completions_for_shadowing() {
        check_reference_completion(
            "dont_show_both_completions_for_shadowing",
            r"
            fn foo() -> {
                let bar = 92;
                {
                    let bar = 62;
                    <|>
                }
            }
            ",
        )
    }

    #[test]
    fn completes_self_in_methods() {
        check_reference_completion("self_in_methods", r"impl S { fn foo(&self) { <|> } }")
    }

}
