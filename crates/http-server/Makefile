.PHONY: default

default:
	cargo r -- start

release:
	cargo b --release

dist:
	make -C ../file-explorer-ui dist

run: dist
	cargo r -- start
