use swc_core::common::{SyntaxContext, DUMMY_SP};
use swc_core::ecma::visit::VisitMutWith;
use swc_core::ecma::{
    ast::*,
    transforms::testing::test_inline,
    visit::{visit_mut_pass, VisitMut},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

// https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
impl VisitMut for TransformVisitor {
    // todo: arrow function
    fn visit_mut_function(&mut self, f: &mut Function) {
        let Some(body) = &mut f.body else {
            // 呼び出し元の関数bodyがない場合は終了
            return;
        };

        // 関数ボディの先頭で "use debug"; を探す
        if let Some(Stmt::Expr(ExprStmt { expr, .. })) = body.stmts.get(0) {
            if let Expr::Lit(Lit::Str(Str { value, .. })) = &**expr {
                // todo: 以下を参考にDirectiveを取得するようリファクタ
                // https://github.com/vercel/next.js/blob/564794df56e421d6d4c2575b466a8be3a96dd39a/crates/next-custom-transforms/src/transforms/server_actions.rs#L339
                if value == "use debug" {
                    // console.log を挿入
                    // `process.env.NODE_ENV`などで分岐したいが、サンプルなので省略
                    let log_stmt = Stmt::Expr(ExprStmt {
                        span: DUMMY_SP,
                        expr: Box::new(Expr::Call(CallExpr {
                            span: DUMMY_SP,
                            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                                span: DUMMY_SP,
                                obj: Box::new(Expr::Ident(Ident::new(
                                    "console".into(),
                                    DUMMY_SP,
                                    SyntaxContext::empty(),
                                ))),
                                prop: MemberProp::Ident(IdentName::new("log".into(), DUMMY_SP)),
                            }))),
                            args: vec![
                                ExprOrSpread {
                                    spread: None,
                                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                                        span: DUMMY_SP,
                                        value: "use debug".into(),
                                        raw: None,
                                    }))),
                                },
                                ExprOrSpread {
                                    spread: None,
                                    expr: Box::new(Expr::Ident(Ident::new(
                                        "arguments".into(),
                                        DUMMY_SP,
                                        SyntaxContext::empty(),
                                    ))),
                                },
                            ],
                            type_args: None,
                            ..Default::default()
                        })),
                    });

                    // "use debug"; を削除して console.log を挿入
                    body.stmts.remove(0);
                    body.stmts.insert(0, log_stmt);
                }
            }
        }

        // 子ノードの変換を続行
        body.visit_mut_with(self);
    }
}

#[plugin_transform]
pub fn process_transform(
    mut program: Program,
    _metadata: TransformPluginProgramMetadata,
) -> Program {
    program.visit_mut_with(&mut visit_mut_pass(TransformVisitor));
    program
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    boo,
    // Input codes
    r#"
function boo() {
  "use debug";
}"#,
    // Output codes after transformed with plugin
    r#"
function boo() {
  console.log("use debug", arguments);
}"#
);
