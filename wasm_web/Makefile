demodir := ./demo
inp := $(demodir)/node_modules/tiny-prng-wasm

all: clean build serve
build:
	cargo build --release --target wasm32-unknown-unknown \
	&& wasm-bindgen --typescript --out-dir pkg  ../target/wasm32-unknown-unknown/release/tiny_prng_wasm.wasm \
	&& test ! -d $(inp) || rm -rv $(inp); \
	[ -d $(inp) ] || mkdir -vp $(inp) \
	&& cp .package.json pkg/package.json \
	&& cp LICENSE_APACHE LICENSE_MIT README.md pkg/ \
	&& cp -rv pkg/ $(inp);
serve:
	cd $(demodir) && yarn install && yarn start
test:
	wasm-pack test --headless --firefox
clean:
	test -d pkg && rm -rv pkg $(inp) || :
