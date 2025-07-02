# git-wrapper-rs

`git-wrapper-rs` 是一个用 Rust 编写的 Git 命令包装器，旨在解决在 Windows 环境下，特别是使用 MSYS2 原生 Git 时，由于路径格式不兼容导致 VSCode 中 GitLens 等工具报错的问题。它通过自动将 `git rev-parse` 和 `git ls-files` 等命令的 Unix 风格路径输出转换为 Windows 风格路径来解决此问题。

## 功能

- 包装 `git` 命令。
- 自动将 `git rev-parse` 和 `git ls-files` 的 Unix 风格路径输出转换为 Windows 风格路径。

## 构建

确保您已安装 Rust 编程语言和 Cargo 包管理器。

```bash
cargo build --release
```

这将在 `target/release/` 目录下生成可执行文件。

## 运行

您可以直接运行生成的可执行文件，并像使用 `git` 命令一样传递参数：

```bash
target/release/git-wrapper-rs <git-command> [args...]
```

例如：

```bash
target/release/git-wrapper-rs rev-parse --show-toplevel
```

### 作为 Git 别名使用

为了更方便地使用，您可以将其配置为 Git 的别名。例如，在您的 `.gitconfig` 文件中添加以下内容：

```ini
[alias]
    wp = !"/path/to/your/git-wrapper-rs"
```

然后您就可以这样使用：

```bash
git wp rev-parse --show-toplevel
```

请将 `/path/to/your/git-wrapper-rs` 替换为实际的可执行文件路径。

## 依赖

- `cygpath` (用于路径转换，通常在 Cygwin 或 MSYS2 环境中提供)

## 许可证

[待定，可在此处添加您的许可证信息]