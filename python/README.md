# agentic-codebase

Semantic code compiler for AI agents.

## Installation

```bash
pip install agentic-codebase
```

## Quick Start

```python
import agentic_codebase

print(agentic_codebase.__version__)
```

## Development

```bash
# Build the native library
cargo build --release

# Install in dev mode
pip install -e "python/[dev]"

# Run tests
pytest python/tests/ -v
```

## License

MIT
