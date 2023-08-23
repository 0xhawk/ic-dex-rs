
.PHONY: install
install: clean
	./scripts/install.sh

.PHONY: test
test: 
	./test/demo.sh

.PHONY: clean
clean: 
	dfx stop
	rm -fr .dfx
