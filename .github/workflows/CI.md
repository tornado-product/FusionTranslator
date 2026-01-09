# CI 是什么？

**CI** 是 **Continuous Integration（持续集成）** 的缩写。

## CI 的含义

持续集成是一种开发实践，核心是：

- 频繁将代码合并到主分支
- 每次合并后自动运行测试、构建等检查
- 尽早发现并修复问题

## CI 的作用

1. **自动化测试**：每次推送代码后自动运行测试
2. **代码质量检查**：自动检查代码格式、代码规范（如 Clippy）
3. **多平台验证**：在不同操作系统和 Rust 版本上测试
4. **快速反馈**：快速发现集成问题

## 在项目中

项目中的 `ci.yml` 工作流：

项目的 `ci.yml` 工作流包含以下 CI 步骤：

### 1. 代码格式化检查（第 59-61 行）

```yaml
- name: Check formatting
  if: matrix.rust == 'stable' && matrix.os == 'ubuntu-latest'
  run: cargo fmt --all -- --check
```

- 检查代码格式是否符合规范
- 确保代码风格统一

### 2. 代码质量检查（第 63-65 行）

```yaml
- name: Clippy
  if: matrix.rust == 'stable' && matrix.os == 'ubuntu-latest'
  run: cargo clippy --all-targets --all-features
```

- 使用 Clippy 检查潜在问题
- 提供改进建议

### 3. 构建项目（第 67-71 行）

```yaml
- name: Build
  timeout-minutes: 30
  env:
    CARGO_BUILD_JOBS: 2
  run: cargo build --verbose --all
```

- 编译代码，确保能正常构建

### 4. 运行测试（第 73-75 行）

```yaml
- name: Run tests
  timeout-minutes: 30
  run: cargo test --verbose --all
```

- 运行所有测试用例
- 验证功能是否正常

### 5. 多平台测试（第 21-33 行）

```yaml
matrix:
  os: [ubuntu-latest, windows-latest, macos-latest]
  rust: [stable, 1.83.0]
  include:
    - os: ubuntu-latest
      rust: stable
      target: x86_64-unknown-linux-gnu
    - os: windows-latest
      rust: stable
      target: x86_64-pc-windows-msvc
    - os: macos-latest
      rust: stable
      target: x86_64-apple-darwin
```

- 在 Linux、Windows、macOS 上测试
- 确保跨平台兼容性

## 实际工作流程

当你推送代码时：

```bash
git push origin main
```

CI 会自动：

1. 检查代码格式
2. 运行 Clippy 检查
3. 在多个平台上构建项目
4. 运行所有测试
5. 如果都通过，显示绿色对勾 ✅；如果有问题，显示红色叉号 ❌ 并显示错误信息

## CI vs CD

- **CI（持续集成）**：自动测试和检查代码
- **CD（持续部署/交付）**：自动发布和部署（`release.yml` 和 `publish.yml` 属于 CD）

## 总结

CI 就像自动化的代码审查员，每次提交代码时自动检查：

- 代码能否编译
- 测试是否通过
- 代码质量是否达标
- 是否能在不同平台上运行

这样可以尽早发现问题，避免把有问题的代码合并到主分支。
