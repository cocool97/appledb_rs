#! /bin/bash

set -e

WEB_DIR=web

cd $WEB_DIR
yarnpkg build
cd -