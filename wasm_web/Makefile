inp := ./www/node_modules/tiny-prng-wasm

all: clean build serve
build:
	wasm-pack build;
	test ! -d $(inp) || rm -rv $(inp);
	cp -rv pkg/ $(inp);
serve:
	cd www && yarn install && yarn start
watch:
	sh -c "while inotifywait -e modify src/*; do make build ; echo >> www/dist/bootstrap.js; done &"
	cd www && yarn install && yarn start
test:
	wasm-pack test --headless --firefox
clean:
	test -d pkg && rm -rv pkg || :
