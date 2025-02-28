#/usr/bin/env sh

# set -eo pipefail
PROTO_FOLDER=$(dirname "$0")
echo $PROTO_FOLDER

echo "Generating Rust proto code"
buf generate --template buf.gen.rust.yaml

# we need to remove the super::super:: and .v1:: from the generated code since
# we just mod with the path of the generated files
find src/gen -type f -name "*.rs" -exec sed -i 's/super::super::/super::/g; s/::v1::/::/g' {} +