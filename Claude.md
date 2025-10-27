i# Claude

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Important Notes

**For Claude**: If you notice you've forgotten key details about this project (like using `uv` instead of `pip`, or commit message specificity), re-read this entire file to refresh context.

**For User**: If you notice Claude has forgotten key workflow details after context compression, ask it to re-read this file.

## Repository Overview



## Repository Structure



## Development Workflow

This repository follows a structured 6-stage programming workflow inspired by design recipe methodology (documented in `Process` file):

1. **Problem Analysis to Data Definitions**: Identify input/output data representation with examples
2. **Signature, Purpose Statement, Header**: Define function signature and stub
3. **Functional Examples**: Create manual examples that will become tests
4. **Function Template**: Translate data definitions into function outline
5. **Function Definition**: Fill in the template
6. **Testing**: Run tests to verify correctness

Time tracking: The workflow uses git commit timestamps to track time spent at each stage.

## Python Development

This project uses **uv** for Python version and dependency management.

### What is uv?

`uv` is a fast, Rust-based Python package manager that:
- Resolves dependencies 10-100x faster than pip
- Automatically manages virtual environments
- Handles Python version management
- Creates lock files for reproducible builds
- Fully compatible with standard `pyproject.toml` format

### Installation

Install uv (one-time setup):
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

Or on macOS with Homebrew:
```bash
brew install uv
```

### Common Commands

**Install dependencies:**
```bash
uv sync
```
This creates a virtual environment and installs all dependencies from `pyproject.toml`.

**Install with dev dependencies:**
```bash
uv sync --all-extras
```

**Run a Python script:**
```bash
uv run scripts/test_yfinance_api.py
```
Automatically uses the project's virtual environment.

**Run a Python module:**
```bash
uv run python -m pytest
```

**Add a new dependency:**
```bash
uv add package-name
```

**Add a dev dependency:**
```bash
uv add --dev package-name
```

**Update dependencies:**
```bash
uv lock --upgrade
uv sync
```

**Run Python REPL with project environment:**
```bash
uv run python
```

### Project Dependencies

**Core dependencies** (defined in `pyproject.toml`):
- `yfinance>=0.2.0` - Yahoo Finance market data
- `yoptions>=0.1.0` - Options Greeks calculation

**Dev dependencies** (optional, installed with `--all-extras`):
- `pytest>=8.0.0` - Testing framework
- `black>=24.0.0` - Code formatter
- `ruff>=0.1.0` - Fast Python linter

### Python Version

This project requires **Python 3.12 or higher**.

The `.python-version` file specifies the exact version. `uv` will automatically use the correct Python version when running commands.

### Virtual Environment

`uv` automatically creates and manages a virtual environment in `.venv/`.

You don't need to manually activate it - `uv run` handles this automatically.

### IDE Setup

**VS Code**: Install the Python extension. It should automatically detect the `.venv/` environment created by uv.

**PyCharm**: Set the Python interpreter to `.venv/bin/python`.




## Common Patterns

### Workflow

Create a Summaryyyymmdd.md file for every new workday, and update it with progress regularly. Add it to the commit after updating and then do the commit.

Use the summary to track work done, issues discovered, and good practices learned.

Update Claude.md when you have learned a new good practice.

### Testing Philosophy

- Write comprehensive tests covering edge cases (empty arrays, zeros, boundary conditions)
- Test naming: `test_<feature>_<case><expected_result>` (e.g., `test_array_oneT` for True result)
- Use combinatorial testing: test sign combinations (+/+, +/-, -/-, -/+) and value ranges (low/low, low/high, high/low, high/high)

### Code Quality Standards

Common errors to avoid:
- Use `False/True` not `FALSE/TRUE` (Python booleans)
- Watch for edge case "thinkos" - assumptions about cases that aren't needed
- Always use version control with meaningful commits

### CI/CD with GitHub Actions

This repository uses GitHub Actions for continuous integration.


### Pre-commit Hooks

This repository uses pre-commit hooks to enforce code quality before commits:

**Initial setup** (one-time):
```bash
pip install pre-commit
pre-commit install
```

**What runs automatically on commit**:
- **Black**: Auto-formats Python code
- **Ruff**: Lints and auto-fixes issues

If hooks fail, they will:
1. Auto-fix formatting issues where possible
2. Block the commit if there are unfixable issues
3. Allow you to review changes and re-attempt the commit

**Manual execution** (optional):
```bash
# Run on all files
pre-commit run --all-files

# Run on staged files only
pre-commit run
```

The pre-commit hooks serve as the first line of defense, with GitHub Actions as a backup for any changes that bypass local hooks.