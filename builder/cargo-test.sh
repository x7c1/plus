#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

to_features_option() {
  if [[ $1 = "./apps/s3api/Cargo.toml" ]]; then
    echo "--features=sabi-s3/x86_support"
  elif [[ $1 = "./libs/aws/sabi-s3/Cargo.toml" ]]; then
    echo "--features=x86_support"
  else
    echo ""
  fi
}

get_manifests() {
  find ./ -type f -name "Cargo.toml" \
    | grep -v "\./Cargo.toml" \
    | sort
}

for manifest in $(get_manifests) ; do
  echo ">> $manifest"
  cargo test \
    --manifest-path=$manifest \
    $(to_features_option $manifest) \
    ${BUILD_MODE} \
    --target="${TARGET_X86}" \
    -- --nocapture
done
