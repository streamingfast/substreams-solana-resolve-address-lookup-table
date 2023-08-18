.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package: build
	substreams pack ./substreams.yaml

.PHONE: stream
stream:
	substreams run substreams.yaml map_address_lookup_table_resolver -e mainnet.sol.streamingfast.io:443 -t +1000