#/usr/bin/env sh

# set -eo pipefail
PROTO_FOLDER=$(dirname "$0")
echo $PROTO_FOLDER

echo "Generating Rust proto code"
(cd $PROTO_FOLDER && buf mod update)
(cd $PROTO_FOLDER && buf generate --template buf.gen.rust.yaml)