#!/usr/bin/env bash
set -euo pipefail

ENV_NAME="${1:-noa}"
PY_VER="${PYTHON_VERSION:-3.12}"

if [ -z "${NOA_ROOT:-}" ]; then
  echo "NOA_ROOT is not set. Source .noa-env first." >&2
  exit 1
fi

NOA_CONDA="${NOA_CONDA:-$NOA_OPT/conda}"
NOA_CONDA_ENV="${NOA_CONDA_ENV:-$NOA_CONDA/envs/$ENV_NAME}"
MICROMAMBA="$NOA_CONDA/micromamba"

mkdir -p "$NOA_CONDA"

if [ ! -x "$MICROMAMBA" ]; then
  echo "micromamba not found/executable at: $MICROMAMBA" >&2
  echo "Place micromamba there (chmod +x), then re-run." >&2
  exit 1
fi

"$MICROMAMBA" create -y -p "$NOA_CONDA_ENV" -c conda-forge "python=$PY_VER" jupyterlab ipykernel

echo "[OK] conda-forge env ready: $NOA_CONDA_ENV"
echo "To activate:"
echo "  source $NOA_ROOT/.noa-env"
echo "  noa_conda_activate"
