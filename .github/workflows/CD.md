# CD 是什么？

**CD** 有两个含义：

- **Continuous Delivery（持续交付）**：代码可以随时部署，但需要手动触发
- **Continuous Deployment（持续部署）**：代码自动部署到生产环境

## CD 的含义

持续交付/持续部署是一种开发实践，核心是：

- 自动化构建和打包
- 自动化发布流程
- 减少人工错误
- 快速交付新版本

## CD 的作用

1. **自动化发布**：自动将代码发布到生产环境或包仓库
2. **版本管理**：自动创建版本标签和发布说明
3. **减少错误**：避免手动操作导致的错误
4. **快速交付**：缩短从开发到发布的周期

## 在项目中的 CD 工作流

项目中有两个 CD 相关的工作流：

### 1. Release 工作流 (`release.yml`)

**触发方式**：推送版本标签（如 `v1.0.1`）或手动触发

**功能**：

#### 1.1 创建 GitHub Release（第 18-86 行）

```yaml
create-release:
  name: Create GitHub Release
  runs-on: ubuntu-latest
  permissions:
    contents: write
```

**步骤**：

1. **获取版本号**（第 30-41 行）
   ```yaml
   - name: Get version from tag
     id: tag
     run: |
       if [ "${{ github.event_name }}" == "workflow_dispatch" ]; then
         VERSION="${{ github.event.inputs.tag }}"
         VERSION=${VERSION#v}
         echo "VERSION=$VERSION" >> $GITHUB_OUTPUT
       else
         echo "VERSION=${GITHUB_REF#refs/tags/v}" >> $GITHUB_OUTPUT
       fi
   ```
   - 从 Git 标签中提取版本号
   - 支持手动触发和自动触发

2. **生成变更日志**（第 57-69 行）
   ```yaml
   - name: Generate changelog
     id: changelog
     run: |
       if [ -f CHANGELOG.md ]; then
         VERSION="${{ steps.tag.outputs.VERSION }}"
         CHANGELOG=$(awk "/^## \[$VERSION\]/,/^## \[/" CHANGELOG.md | sed '$d')
         echo "CHANGELOG<<EOF" >> $GITHUB_OUTPUT
         echo "$CHANGELOG" >> $GITHUB_OUTPUT
         echo "EOF" >> $GITHUB_OUTPUT
       fi
   ```
   - 从 `CHANGELOG.md` 中提取对应版本的更新说明
   - 如果没有 CHANGELOG，使用默认说明

3. **创建 GitHub Release**（第 71-86 行）
   ```yaml
   - name: Create Release
     uses: softprops/action-gh-release@v1
     with:
       tag_name: ${{ steps.tag.outputs.TAG_NAME }}
       name: Release v${{ steps.tag.outputs.VERSION }}
       body: |
         ## Release v${{ steps.tag.outputs.VERSION }}
         ${{ steps.changelog.outputs.CHANGELOG }}
   ```
   - 在 GitHub 上创建 Release
   - 包含版本号和变更日志

#### 1.2 发布到 crates.io（第 88-140 行）

```yaml
publish:
  name: Publish fusion-translator to crates.io
  runs-on: ubuntu-latest
  needs: create-release
```

**步骤**：

1. **验证版本号**（第 125-134 行）
   ```yaml
   - name: Verify package version
     run: |
       PACKAGE_VERSION=$(grep -E '^version\s*=' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
       if [ "$PACKAGE_VERSION" != "${{ steps.tag.outputs.VERSION }}" ]; then
         echo "Error: Package version doesn't match tag version"
         exit 1
       fi
   ```
   - 确保 `Cargo.toml` 中的版本号与标签版本一致
   - 防止版本不匹配导致的发布错误

2. **发布到 crates.io**（第 136-140 行）
   ```yaml
   - name: Publish to crates.io
     run: |
       cargo publish --token ${{ secrets.CRATES_IO_TOKEN }} --no-verify
     env:
       CARGO_REGISTRY_TOKEN: ${{ secrets.CRATES_IO_TOKEN }}
   ```
   - 将包发布到 Rust 官方包仓库
   - 使用 `CRATES_IO_TOKEN` 进行身份验证

### 2. Publish 工作流 (`publish.yml`)

**触发方式**：手动触发（workflow_dispatch）

**功能**：手动发布包到 crates.io

**步骤**：

1. **更新版本号**（第 40-44 行）
   ```yaml
   - name: Update version
     if: github.event.inputs.version != ''
     run: |
       sed -i "s/^version = .*/version = \"${{ github.event.inputs.version }}\"/" Cargo.toml
   ```
   - 根据输入的版本号更新 `Cargo.toml`
   - 支持手动指定版本

2. **验证包**（第 46-48 行）
   ```yaml
   - name: Verify package
     run: |
       cargo package --allow-dirty
   ```
   - 打包并验证包的有效性
   - 检查包是否符合发布要求

3. **发布到 crates.io**（第 50-54 行）
   ```yaml
   - name: Publish to crates.io
     run: |
       cargo publish --token ${{ secrets.CRATES_IO_TOKEN }} --no-verify
   ```
   - 发布包到 crates.io

## 实际工作流程

### 自动发布流程（推荐）

当你准备发布新版本时：

```bash
# 1. 更新 Cargo.toml 中的版本号
# version = "1.0.2"

# 2. 更新 CHANGELOG.md
# 添加新版本的更新说明

# 3. 提交更改
git add .
git commit -m "准备发布 v1.0.2"
git push origin main

# 4. 创建并推送版本标签
git tag -a v1.0.2 -m "Release v1.0.2"
git push origin v1.0.2
```

CD 会自动：

1. ✅ 创建 GitHub Release
2. ✅ 从 CHANGELOG.md 提取发布说明
3. ✅ 验证版本号匹配
4. ✅ 发布到 crates.io
5. ✅ 完成整个发布流程

### 手动发布流程

当你需要手动发布时：

1. 进入 GitHub Actions 页面
2. 选择 "Publish to crates.io" 工作流
3. 点击 "Run workflow"
4. 填写版本号（如 `1.0.2`）
5. 点击 "Run workflow"

CD 会自动：

1. ✅ 更新 `Cargo.toml` 中的版本号
2. ✅ 验证包的有效性
3. ✅ 发布到 crates.io

## Release vs Publish

| 特性 | Release (`release.yml`) | Publish (`publish.yml`) |
|------|------------------------|------------------------|
| **触发方式** | 推送标签（自动）或手动 | 仅手动 |
| **GitHub Release** | ✅ 自动创建 | ❌ 不创建 |
| **版本管理** | 从标签获取 | 手动输入 |
| **变更日志** | ✅ 从 CHANGELOG.md 提取 | ❌ 不处理 |
| **使用场景** | 正式版本发布 | 紧急修复、测试发布 |

## CI vs CD

| 阶段 | CI（持续集成） | CD（持续部署/交付） |
|------|---------------|-------------------|
| **目的** | 验证代码质量 | 发布代码 |
| **触发时机** | 每次推送代码 | 准备发布时 |
| **主要任务** | 测试、检查、构建 | 打包、发布、部署 |
| **工作流文件** | `ci.yml` | `release.yml`, `publish.yml` |

**工作流程**：

```
代码推送 → CI（测试） → ✅ 通过 → CD（发布） → 🚀 上线
                ↓
            ❌ 失败 → 修复代码
```

## 总结

CD 就像自动化的发布助手，当你准备发布新版本时：

- **自动创建 Release**：在 GitHub 上创建版本发布
- **自动提取变更日志**：从 CHANGELOG.md 中提取更新说明
- **自动验证版本**：确保版本号匹配
- **自动发布**：将包发布到 crates.io

这样可以：

- ✅ 减少人工错误
- ✅ 提高发布效率
- ✅ 保持发布流程的一致性
- ✅ 快速交付新功能

**记住**：在发布前，确保：

1. ✅ 代码已通过 CI 测试
2. ✅ 版本号已更新
3. ✅ CHANGELOG.md 已更新
4. ✅ `CRATES_IO_TOKEN` 已配置
