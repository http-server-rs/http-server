.PHONY: default test

default: release

build:
	make -C ./src/file-explorer-ui dist
	make -C ./src/http-server build

release:
	make -C ./src/file-explorer-ui release
	make -C ./src/http-server release

run:
	make -C ./src/http-server run

ui-build:
	make -C ./src/file-explorer-ui release

ui-dev:
	@echo Starting File Explorer UI…
	make -C ./src/file-explorer-ui dev

ui-fmt:
	make -C ./src/file-explorer-ui fmt

ui-release:
	make -C ./src/file-explorer-ui release

test:
	make -C ./test run
