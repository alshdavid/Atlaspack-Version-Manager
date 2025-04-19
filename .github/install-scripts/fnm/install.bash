#!/bin/bash

out_dir="$HOME/.local"

rm -rf $out_dir/fnm
mkdir -p $out_dir/fnm
mkdir $out_dir/fnm/bin
mkdir $out_dir/fnm/fnm_dir

wget -q https://github.com/Schniz/fnm/releases/latest/download/fnm-linux.zip -O $out_dir/fnm/fnm.zip
unzip -q $out_dir/fnm/fnm.zip -d $out_dir/fnm/bin
rm -rf $out_dir/fnm/fnm.zip

chmod +x $out_dir/fnm/bin/fnm
export PATH="$out_dir/fnm/bin/fnm:$PATH"

eval "$(fnm env)"
fnm use