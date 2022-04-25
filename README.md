# 简单编译器

## 计划

- [x] 词法分析:`lexical_analysis()`
- [ ] 语法分析:`parsing()`
- [ ] 汇编生成:`asm_generate()`

## 运行测试

```bash
$ cargo test
```

## 项目结构

```bash
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── lexical_analysis
│   │   └── ...
│   ├── lexical_analysis.rs #词法分析
│   └── lib.rs
├── static
│   └── keep_str.jsonc #保留字存放位置
└── target
    └── ...
```

