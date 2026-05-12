<p align="center">
<a href="./assets/CPC 2.R.svg">
<img src="./assets/CPC 2.R.svg" width="120" height="120" alt="logo">
</a>
<h3 align="center">CAIE 伪代码解释器 2</h3>
</p>
<p align="center">
<a href="./README_zh.md">中文</a> | <a href="./README.md">English</a>
</p>

> 当前版本正在初期开发阶段，存在大量功能缺失以及未知错误，若想要稳定使用，请移步 [CAIE_Code](https://github.com/iewnfod/CAIE_Code)

## 功能 & 规划
- [x] 基础类型 `INT`, `REAL`, `STRING`, `BOOLEAN`
- [x] 数组（多维数组理论上支持，但是目前还没有索引的方式）
- [x] `IF` 表达式
- [ ] `FOR`, `WHILE`, `UNTIL` 循环
- [ ] `FUNCTION` 与 `PROCEDURE`
- [ ] `INPUT` 与 `OUTPUT`
- [ ] `CALL` 与 `RETURN`
- [ ] `MATCH` 与 `CASE`
- [ ] `RECORD` 与 `POINTER`
- [ ] `CLASS` OOP

### Improvements in v2
- 一个全新的作用域与对象统一架构
- 完全使用 Rust 实现，更好的性能，更安全的内存
- 一个新的语法解析器来修复之前的各种不兼容的问题
- 更加合理且完善的错误检查与输出

## 贡献
我们欢迎并期待来自社区的贡献！无论它是关于修复问题、修改文档、或是实现新功能。

### AI 生成代码规范
我们注意到 AI 如今已经是一个强大的生产力工具，但我们依旧更加注重代码质量以及长期的可维护性：
* 有人参与的生产：我们不接受任何包含未经人工审查的原始的 AI 生成的拉取请求
* 责任：如果你使用了 AI 来辅助你的代码书写，你有责任解释你的所有代码
* 质量大于数量：相比于快速实现更多新的功能，我们更重视健壮逻辑与兼容，尽可能贯彻高内聚低耦合的思想

### How to Contribute
1. 提出问题：对于主要修改，请在实现前先开启一个 Issue 来与我们讨论你的设计
2. 开发分支：所有的拉取请求应该合并到 dev 分支，再逐渐发布到其他分支
3. 审查：我们期待严格的代码审查，期间可能会查看你的实现来保证你的逻辑与对象统一的模型匹配
4. 测试：我们期待你提供测试用例，来保证你的代码是正确的，并且不会破坏现有的功能。Github Action 会对每个提交和拉取请求自动运行 `cargo test` 命令来测试所有样例

## 使用的技术
* [Rust](https://rust-lang.org/)

## 开源协议
[MPL-2.0](./LICENSE)
