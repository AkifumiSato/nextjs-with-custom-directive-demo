# Next.js with custom directive demo

TBW

## SWC Plugin実装時のメモ

### 雛形がbuildできない

雛形はそのままだとbuildできないので、以下の修正を行う必要がある

```diff
 #[plugin_transform]
 pub fn process_transform(mut program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
-    program.fold_with(&mut visit_mut_pass(TransformVisitor))
+    program.visit_mut_with(&mut visit_mut_pass(TransformVisitor));
+    program
 }
```

### 破壊的変更リリースがいつだったかわからない

- リリースドキュメントがない
- SWC Pluginのページに破壊的変更についてのメモはあるが、大まかなので↑の修正には役立たなかった

### Next.jsのSWC Pluginのページも情報が少ない

experimentalなのでしょうがない気もするが、長いこと実装されてる気はする
