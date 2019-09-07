# shellcheck source=./util.sh
_dir="$( dirname "$0" )"
[ -f "${_dir}/util.sh" ] || bash "${_dir}/download-util.sh" || exit 1
source "${_dir}/util.sh"
unset _dir

# https://unix.stackexchange.com/q/32907 (comments)
# Escape path for use with sed
function sed_escape {
  echo "$1" | sed -r 's/([\$\.\*\/\[\\^])/\\\1/g' | sed 's/[]]/\[]]/g'
}

PROFILE="debug"
BACKEND="${ROOT}/backend"
BACKEND_BIN="hello-rocket-yew-backend"
FRONTEND="${ROOT}/frontend"
FRONTEND_BIN="hello-rocket-yew-frontend"
STATIC_BACKEND="${BACKEND}/static"
STATIC_FRONTEND="${FRONTEND}/static"
JS_RUNTIME="standalone"
WASM_TARGET="wasm32-unknown-unknown"

cargo_arg_extra=
[ "$PROFILE" == "release" ] && cargo_arg_extra="--release"

CARGO_ARGS=
[ "$PROFILE" == "release" ] && CARGO_ARGS="--release"

export RUSTUP_TOOLCHAIN="nightly-2019-09-05"
