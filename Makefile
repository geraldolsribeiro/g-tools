all:

.PHONY: install
install:
	cargo install --path .

.PHONY: publish
publish:
	cargo publish --allow-dirty

.PHONY: acceptance-test
acceptance-test: install
	./test_01.sh
