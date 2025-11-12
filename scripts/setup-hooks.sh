#!/bin/bash

# Setup Git hooks for the project
# 为项目设置 Git 钩子

set -e

echo "[步骤] 正在设置 Git 钩子 / Setting up Git hooks..."

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Configure git to use .githooks directory
git config core.hooksPath .githooks

# Make the pre-commit hook executable
chmod +x "$PROJECT_ROOT/.githooks/pre-commit"

echo "[完成] Git 钩子配置成功 / Git hooks configured successfully!"
echo ""
echo "[信息] 已安装以下钩子 / The following hook has been installed:"
echo "  - pre-commit: 运行代码格式检查、静态分析、clippy 和安全审计 / Runs format check, static analysis, clippy, and security audit"
echo ""
echo "[提示] 这些检查将在每次提交前自动运行 / These checks will run automatically before each commit"
echo "[警告] 如果检查失败，提交将被中止 / If any check fails, the commit will be aborted"
echo ""
echo "[跳过] 要绕过钩子（不推荐）/ To bypass hooks (not recommended):"
echo "  git commit --no-verify"
