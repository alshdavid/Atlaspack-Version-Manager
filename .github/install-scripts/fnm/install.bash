#!/bin/bash

out_dir="$HOME/.local"

rm -rf $out_dir/fnm
mkdir $out_dir/fnm
mkdir $out_dir/fnm/bin
mkdir $out_dir/fnm/fnm_dir

wget -q https://github.com/Schniz/fnm/releases/latest/download/fnm-linux.zip -O $out_dir/fnm/fnm.zip
unzip -q $out_dir/fnm/fnm.zip -d $out_dir/fnm/bin
rm -rf $out_dir/fnm/fnm.zip

chmod +x $out_dir/fnm/bin/fnm
ln -s $out_dir/fnm/bin/fnm $out_dir/fnm/bin/nvm

eval "$(fnm env)"
nvm use