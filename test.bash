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

cargo build
# rm -rf test/apvm
# rm -rf test
# mkdir -p test
cp ./target/debug/apvm ./test
cd test
exec ./apvm $@