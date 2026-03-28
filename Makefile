all:

.PHONY: install
install:
	cargo install --path .

.PHONY: acceptance-test
acceptance-test: install
	./test_01.sh
