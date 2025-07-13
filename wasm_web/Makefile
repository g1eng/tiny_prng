inp := ./www/node_modules/tiny-prng-wasm

all: clean build serve
build:
	wasm-pack build;
	test ! -d $(inp) || rm -rv $(inp);
	cp -rv pkg/ $(inp);
serve:
	cd www && yarn install && yarn start
test:
	wasm-pack test --headless --firefox
clean:
	test -d pkg && rm -rv pkg www/node_modules/tiny-prng www/node_modules/tiny-prng-wasm || :
