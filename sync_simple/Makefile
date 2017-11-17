WHAT  := proto server client

all:
	for target in $(WHAT); do \
		echo "building $$target..."; \
		pushd $$target && cargo build && popd; \
	done
	
