#!/usr/bin/env bash

set -xue

GH_VERSION="1.2.0"

main() {
  setup_gh
}

setup_gh() {
  if command -v gh; then
    echo "gh already installed."
    return
  fi

#  dirname="gh_${GH_VERSION}_linux_amd64"
#  filename="${dirname}".tar.gz
  curl -OL https://github.com/cli/cli/releases/download/v${GH_VERSION}/gh_${GH_VERSION}_linux_amd64.deb
  apt install ./gh_${GH_VERSION}_linux_amd64.deb
#  curl -OL https://github.com/cli/cli/releases/download/v${GH_VERSION}/gh_${GH_VERSION}_linux_amd64.tar.gz
#  tar -xzvf "${filename}"
#  cp "$dirname/bin/gh" ~/bin
#  chmod u+x ~/bin/gh
}

main
