<p align="center">
<a href="./assets/CPC 2.R.svg">
<img src="./assets/CPC 2.R.svg" width="120" height="120" alt="logo">
</a>
<h3 align="center">the CAIE Pseudocode Interpreter 2</h3>
</p>
<p align="center">
<a href="./README_zh.md">简体中文</a> | <a href="./README.md">English</a>
</p>

> The current version is in the early development stage, and there are a lot of missing functions and unknown errors. If you want to use it stably, please move to [CAIE_Code](https://github.com/iewnfod/CAIE_Code).

## Features & Roadmap
- [x] Basic Literal Types `INT`, `REAL`, `STRING`, `BOOLEAN`.
- [x] Array (Multi-Dimension Array is logically supported, but still without index method)
- [x] `IF` Statement.
- [x] `FOR`, `WHILE`, `UNTIL` loop.
- [ ] `FUNCTION` and `PROCEDURE`.
- [ ] `INPUT` and `OUTPUT`.
- [ ] `CALL` and `RETURN`.
- [ ] `MATCH` and `CASE`.
- [ ] `RECORD` and `POINTER`.
- [ ] `CLASS` for OOP.

### Improvements in v2
- A new Scope and Object Unified Model.
- Fully implementation in Rust with a better performance and safer memory.
- A new parser to fix syntax detection problems.
- Better error detection and output.

## Contribution
We welcome contributions from the community! Whether it's fixing bugs, improving documentation, or implementing new features from the CAIE syllabus.

### AI-Generated Code Policy
While we recognize AI as a powerful productivity tool, we prioritize code quality and long-term maintainability:
* Human-in-the-loop is mandatory: We do not accept "lazy" PRs consisting of raw, unverified AI outputs.
* Responsibility: If you use AI to assist your coding, you are 100% responsible for explaining every line of your code during the review process.
* Quality over Quantity: We value deep understanding of interpreter logic over rapid but shallow feature expansion.

### How to Contribute
1. Issue First: For major changes, please open an issue to discuss your design before implementation.
2. Dev Branch: All pull requests should be directed to the dev branch.
3. Review: Expect a rigorous code review. We might ask you to refactor or explain your implementation details to ensure it aligns with our Unified Object Model.
4. Test: Test case about your implementation would be useful to make sure your codes can work properly without breaking existing function. Github Action will automatically run `cargo test` to run all test cases for each commit and PR.

## Technologies Used
* [Rust](https://rust-lang.org/)
* [Colored](https://github.com/colored-rs/colored)

## License
[MPL-2.0](./LICENSE)
