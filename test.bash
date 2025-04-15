# cargo build
# # rm -rf test/apvm
# rm -rf test
# mkdir -p test
# cp ./target/debug/apvm ./test
# cd test
# exec ./apvm $@


# cargo build
# # rm -rf test/apvm
# # rm -rf test
# # mkdir -p test
# cp ./target/debug/apvm ./test
# cd test
# exec ./apvm $@


just build
rm -rf test
mkdir -p test

cp ./target/linux-amd64/debug/apvm ./test
cd test

eval "$(./apvm env -s bash)"
./apvm env -s bash
./apvm $@
cd ..