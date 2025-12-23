# Contributing

Thanks for contributing! This project uses pre-commit hooks and license header checks.

---

## Quick Setup

```bash
# Install pre-commit
# (use pipx or pip as you prefer)

# Install hooks via Makefile
make setup

# Or manually:
# Install pre-commit
pipx install pre-commit
# or: pip install pre-commit

# Enable hooks
pre-commit install --hook-type pre-commit --hook-type pre-push
```

## Required Checks

Pre-commit hooks run automatically on commit/push (or use `make lint` to run them all):

See `.pre-commit-config.yaml` for the full list of checks.

You can run them manually:

```bash
make lint
# or: pre-commit run --all-files
```

## GIF Capture (CLI Animation)

```bash
asciinema rec animate.cast -c './tui-banner --text "RUST CLI" --animate-roll 15' --overwrite
agg animate.cast animate.gif
gifsicle --crop 0,0+1030x250 animate.gif > output.gif
```
