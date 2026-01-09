# GitHub Actions Workflows

## 📖 什么是 GitHub Actions？

GitHub Actions 是 GitHub 提供的自动化 CI/CD（持续集成/持续部署）平台，可以自动执行代码测试、构建、发布等任务。

### 基本概念

- **Workflow（工作流）**: 一个自动化流程，由多个步骤组成
- **Job（任务）**: 工作流中的一个执行单元，可以包含多个步骤
- **Step（步骤）**: 具体的操作，如安装依赖、运行测试等
- **Action（动作）**: 可重用的代码单元，如 `actions/checkout@v4`

### 工作流文件位置

工作流配置文件位于 `.github/workflows/` 目录下，使用 YAML 格式（`.yml` 或 `.yaml`）。

本项目的工作流文件：
- `ci.yml` - 持续集成（测试、构建）
- `release.yml` - 自动发布
- `publish.yml` - 手动发布
- `docs.yml` - 文档构建和部署

## 🎯 如何使用 GitHub Actions

### 1. 在哪里查看工作流？

#### 方法一：GitHub 网页界面

1. **进入仓库页面**
   - 打开你的 GitHub 仓库（如：`https://github.com/tornado-product/FusionTranslator`）

2. **点击 "Actions" 标签页**
   - 在仓库顶部导航栏中，点击 **"Actions"** 标签
   - 这是查看所有工作流运行记录的地方

3. **查看工作流列表**
   - 左侧显示所有可用的工作流（CI、Release、Publish、Docs）
   - 右侧显示每次运行的记录（成功 ✅、失败 ❌、进行中 🟡）

4. **查看详细日志**
   - 点击任意一次运行记录
   - 可以看到每个步骤的执行情况
   - 点击具体的 Job 或 Step 可以查看详细日志

#### 方法二：仓库首页状态徽章

- 在仓库首页（`README.md` 或代码页面）可以看到 CI 状态徽章
- 绿色 ✅ 表示通过，红色 ❌ 表示失败

### 2. 如何触发工作流？

工作流有几种触发方式：

#### 自动触发（最常见）

**推送到分支时自动触发：**
```bash
# 当你推送代码到 main 分支时，CI 工作流会自动运行
git add .
git commit -m "更新代码"
git push origin main
```

**创建 Pull Request 时自动触发：**
- 当你创建 PR 时，CI 会自动运行测试
- 确保代码质量后再合并

**推送 Tag 时自动触发：**
```bash
# 推送版本标签时，Release 工作流会自动运行
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

#### 手动触发

1. **进入 Actions 页面**
   - 在仓库页面点击 **"Actions"** 标签

2. **选择工作流**
   - 在左侧列表中选择要运行的工作流（如 "Publish to crates.io"）

3. **点击 "Run workflow"**
   - 在右侧点击 **"Run workflow"** 按钮
   - 选择分支（通常是 `main`）
   - 填写必要的参数（如果有）
   - 点击绿色的 **"Run workflow"** 按钮

4. **查看运行状态**
   - 工作流会立即开始运行
   - 可以在列表中看到运行状态

### 3. 工作流运行状态说明

- **🟡 黄色圆点**: 正在运行中
- **✅ 绿色对勾**: 成功完成
- **❌ 红色叉号**: 执行失败
- **⏸️ 暂停图标**: 等待中或被暂停

### 4. 查看运行日志

1. 点击任意一次运行记录
2. 展开左侧的 Job（如 "Test"、"Build Release"）
3. 展开具体的 Step（如 "Install Rust"、"Run tests"）
4. 查看右侧的日志输出

**日志用途：**
- 查看错误信息（如果失败）
- 了解执行过程
- 调试问题

### 5. 常见操作

#### 重新运行失败的工作流

1. 进入失败的运行记录页面
2. 点击右上角的 **"Re-run jobs"** 按钮
3. 选择要重新运行的 Job

#### 取消正在运行的工作流

1. 进入正在运行的记录页面
2. 点击右上角的 **"Cancel workflow"** 按钮

#### 查看工作流配置

- 工作流配置文件在 `.github/workflows/` 目录
- 可以直接在 GitHub 上查看或编辑
- 修改后推送到仓库即可生效

## 📋 本项目包含的自动化工作流

### 1. CI (`ci.yml`)

**触发条件**: 
- 推送到 `main`/`master`/`develop` 分支
- 创建 Pull Request

**功能**:
- ✅ 多平台测试 (Linux, Windows, macOS)
- ✅ 多 Rust 版本测试
- ✅ 代码格式化检查 (`cargo fmt`)
- ✅ 代码质量检查 (`cargo clippy`)
- ✅ 构建所有包
- ✅ 运行所有测试
- ✅ 构建 Release 二进制文件（仅 main/master 分支）

### 2. Release (`release.yml`)

**触发条件**: 
- 推送以 `v` 开头的 tag（如 `v1.0.2`）

**功能**:
- ✅ 自动创建 GitHub Release
- ✅ 从 CHANGELOG.md 提取发布说明
- ✅ 发布 `fusion-translator` 包到 crates.io

### 3. Publish (`publish.yml`)

**触发条件**: 
- 手动触发（workflow_dispatch）

**功能**:
- ✅ 手动发布包到 crates.io
- ✅ 可指定版本号

**使用场景**: 
- 修复发布问题
- 测试发布流程

### 4. Docs (`docs.yml`)

**触发条件**: 
- 推送到 `main`/`master` 分支
- 手动触发

**功能**:
- ✅ 构建所有包的文档
- ✅ 自动部署到 GitHub Pages
- ✅ 文档地址: `https://tornado-product.github.io/FusionTranslator/`

## 🔧 配置要求

### 必需的 Secrets

在 GitHub 仓库设置中添加以下 Secret：

1. **CRATES_IO_TOKEN** (必需)
   - 获取方式: [crates.io](https://crates.io) → Account Settings → API Tokens
   - 用于发布包到 crates.io
   - **重要**: 发布前必须验证邮箱地址
     - 访问 https://crates.io/settings/profile
     - 设置并验证你的邮箱地址
     - 未验证邮箱会导致发布失败
   - 点击仓库顶部的 Settings（设置）标签
   - 在左侧边栏找到 Secrets and variables → 点击 Actions
   - 在 Repository secrets 部分，点击 New repository secret 按钮
   - 填写信息：
      Name: CRATES_IO_TOKEN（必须与 workflow 中的名称完全一致）
      Secret: 粘贴你的 crates.io API token
      点击 Add secret

2. **GITHUB_TOKEN** (自动)
   - GitHub 自动提供，无需手动配置
   - 用于创建 Release 和部署文档

### 配置步骤

1. 进入仓库 Settings → Secrets and variables → Actions
2. 点击 "New repository secret"
3. 添加 `CRATES_IO_TOKEN`，值为你的 crates.io API token

## 🚀 使用流程

### 自动发布（推荐）

1. 更新版本号和 CHANGELOG
2. 提交更改
3. 创建并推送 tag:
   ```bash
   git tag -a v1.0.2 -m "Release v1.0.2"
   git push origin v1.0.2
   ```
4. GitHub Actions 自动完成发布

### 手动发布

1. 进入 GitHub Actions 页面
2. 选择 "Publish to crates.io" workflow
3. 点击 "Run workflow"
4. 填写版本号
5. 点击 "Run workflow"

## 📊 查看状态

### 在仓库首页查看

- **CI 状态徽章**: 在 `README.md` 或代码页面顶部可以看到 CI 状态
  - 绿色 ✅ = 所有测试通过
  - 红色 ❌ = 有测试失败
  - 黄色 🟡 = 正在运行

### 在 Actions 页面查看

1. 点击仓库顶部的 **"Actions"** 标签
2. 查看工作流运行历史：
   - 左侧：工作流列表
   - 右侧：运行记录列表
   - 每条记录显示：提交信息、触发者、运行时间、状态

### 查看详细状态

- **CI 进度**: Actions 标签页 → 选择 "CI" 工作流
- **发布进度**: Actions 标签页 → 选择 "Release" 工作流
- **Release 记录**: 点击仓库顶部的 **"Releases"** 标签
- **文档地址**: `https://tornado-product.github.io/FusionTranslator/`

## 🚀 快速入门示例

### 示例 1: 第一次推送代码

```bash
# 1. 在本地修改代码
echo "// 新功能" >> src/main.rs

# 2. 提交并推送
git add .
git commit -m "添加新功能"
git push origin main

# 3. 自动触发 CI
# - 打开 GitHub 仓库页面
# - 点击 "Actions" 标签
# - 你会看到 CI 工作流正在运行
# - 等待几分钟，查看测试结果
```

### 示例 2: 手动触发文档构建

1. 打开 GitHub 仓库页面
2. 点击 **"Actions"** 标签
3. 在左侧选择 **"Docs"** 工作流
4. 点击右侧的 **"Run workflow"** 按钮
5. 选择分支（`main`）
6. 点击绿色的 **"Run workflow"** 按钮
7. 等待构建完成，文档会自动部署

### 示例 3: 发布新版本

```bash
# 1. 更新版本号（在 Cargo.toml 中）
# version = "1.0.1"

# 2. 更新 CHANGELOG.md
# 添加新版本的更新说明

# 3. 提交更改
git add .
git commit -m "准备发布 v1.0.1"
git push origin main

# 4. 创建并推送标签
git tag -a v1.0.1 -m "Release v1.0.1"
git push origin v1.0.1

# 5. 自动触发 Release 工作流
# - 在 Actions 页面可以看到 Release 工作流运行
# - 会自动创建 GitHub Release
# - 会自动发布到 crates.io
```

## 💡 实用技巧

### 1. 如何知道工作流是否在运行？

- 查看仓库首页的 CI 状态徽章
- 进入 Actions 页面，查看最新的运行记录
- 黄色圆点 🟡 = 正在运行

### 2. 工作流失败了怎么办？

1. **查看错误日志**
   - 进入 Actions 页面
   - 点击失败的运行记录
   - 展开失败的 Job 和 Step
   - 查看红色错误信息

2. **常见问题**
   - **测试失败**: 检查测试代码
   - **格式错误**: 运行 `cargo fmt --all`
   - **Clippy 警告**: 运行 `cargo clippy --all-targets --all-features`
   - **构建失败**: 检查依赖和代码语法

3. **修复后重新运行**
   - 修复代码后再次推送
   - 或者点击 "Re-run jobs" 按钮

### 3. 如何禁用某个工作流？

- 暂时禁用：在工作流文件中添加 `if: false` 条件
- 永久删除：删除 `.github/workflows/` 目录下的对应文件

### 4. 如何修改工作流配置？

1. 编辑 `.github/workflows/` 目录下的 YAML 文件
2. 提交并推送到仓库
3. 新的配置会立即生效

### 5. 工作流运行需要多长时间？

- **CI 测试**: 通常 5-15 分钟（取决于测试复杂度）
- **构建 Release**: 通常 3-5 分钟
- **发布到 crates.io**: 通常 5-10 分钟
- **文档构建**: 通常 2-5 分钟

## 🐛 故障排查

### 发布失败

1. **检查 Token**: 确保 `CRATES_IO_TOKEN` 已正确设置
2. **检查邮箱验证**: 
   - 错误信息: `A verified email address is required to publish crates to crates.io`
   - 解决方法: 访问 https://crates.io/settings/profile
   - 设置并验证你的邮箱地址
   - 验证完成后重新运行发布工作流
3. **检查版本**: 确保版本号未被使用
4. **检查依赖**: 确保依赖的包已发布
5. **查看日志**: 在 Actions 页面查看详细错误信息

### CI 失败

1. **格式化错误**: 运行 `cargo fmt --all`
2. **Clippy 错误**: 运行 `cargo clippy --all-targets --all-features`
3. **测试失败**: 检查测试代码和依赖

## 📚 相关文档

- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [crates.io 发布指南](https://doc.rust-lang.org/cargo/reference/publishing.html)
