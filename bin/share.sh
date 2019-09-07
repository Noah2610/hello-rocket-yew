# shellcheck source=./util.sh
_dir="$( dirname "$0" )"
[ -f "${_dir}/util.sh" ] || bash "${_dir}/download-util.sh" || exit 1
source "${_dir}/util.sh"
unset _dir

PROFILE="debug"
BACKEND="${ROOT}/backend"
BACKEND_BIN="hello-rocket-yew-backend"
FRONTEND="${ROOT}/frontend"
FRONTEND_BIN="hello-rocket-yew-frontend"
STATIC_BACKEND="${BACKEND}/static"
STATIC_FRONTEND="${FRONTEND}/static"
JS_RUNTIME="library-es6"

CARGO_ARGS=
[ "$PROFILE" == "release" ] && CARGO_ARGS="--release"

export RUSTUP_TOOLCHAIN="nightly-2019-09-05"
