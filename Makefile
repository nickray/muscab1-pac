YAML = musca-b1.yaml
SVD = musca-b1.svd.patched

prepare: patch generate
	cargo build

patch:
	svd patch $(YAML)

# Generates PAC source code from (patched) SVD
generate:
	rm -rf src
	mkdir src
	svd2rust -i ./$(SVD)
	form -i lib.rs -o src/ && rm lib.rs
	cargo fmt

# External documentation
fetch-docs:
	mkdir -p ref
	curl -sk https://static.docs.arm.com/101312/0000/arm_musca_b1_test_chip_and_board_technical_reference_manual_101312_0000_00_en.pdf \
		-o ref/technicalreferencemanual-muscab1.pdf

# Maintenance
VERSION := $(shell grep version Cargo.toml|head -1|cut -d' ' -f 3|tr -d '"')
tag:
	git tag -a $(VERSION) -m"v$(VERSION)"

version:
	echo $(VERSION)

