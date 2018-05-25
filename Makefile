SHELL := /usr/bin/env bash
GOOD_FEATURE_FILES = $(shell find testdata/good -name "*.feature")
BAD_FEATURE_FILES  = $(shell find testdata/bad -name "*.feature")

TOKENS     = $(patsubst testdata/%.feature,acceptance/testdata/%.feature.tokens,$(GOOD_FEATURE_FILES))
ASTS       = $(patsubst testdata/%.feature,acceptance/testdata/%.feature.ast.ndjson,$(GOOD_FEATURE_FILES))
PICKLES    = $(patsubst testdata/%.feature,acceptance/testdata/%.feature.pickles.ndjson,$(GOOD_FEATURE_FILES))
SOURCES    = $(patsubst testdata/%.feature,acceptance/testdata/%.feature.source.ndjson,$(GOOD_FEATURE_FILES))
ERRORS     = $(patsubst testdata/%.feature,acceptance/testdata/%.feature.errors.ndjson,$(BAD_FEATURE_FILES))

RUST_FILES = $(shell find . -name "*.rs")

ifeq ($(CC),i686-w64-mingw32-gcc)
	GHERKIN=target/debug/gherkin.exe
	RUN_GHERKIN=wine $(GHERKIN)
	GHERKIN_GENERATE_TOKENS=target/debug/gherkin_generate_tokens.exe
	RUN_GHERKIN_GENERATE_TOKENS=wine $(GHERKIN_GENERATE_TOKENS)
else
	GHERKIN=target/debug/gherkin
	RUN_GHERKIN=$(GHERKIN)
	GHERKIN_GENERATE_TOKENS=target/debug/gherkin_generate_tokens
	RUN_GHERKIN_GENERATE_TOKENS=$(GHERKIN_GENERATE_TOKENS)
endif

.DELETE_ON_ERROR:

default: .compared
.PHONY: default

.compared: $(TOKENS) $(ASTS) $(PICKLES) $(ERRORS) $(SOURCES)
	touch $@

.built: src/parser.rs gherkin-languages.json $(RUST_FILES) LICENSE Cargo.toml
	cargo build
	cargo test
	touch $@

acceptance/testdata/%.feature.tokens: testdata/%.feature testdata/%.feature.tokens .built
	mkdir -p `dirname $@`
	$(RUN_GHERKIN_GENERATE_TOKENS) $< > $@
	diff --strip-trailing-cr --unified $<.tokens $@
.DELETE_ON_ERROR: acceptance/testdata/%.feature.tokens

# Generate
# acceptance/testdata/%.feature.ast.ndjson: testdata/%.feature .built
# 	mkdir -p `dirname $@`
# 	$(RUN_GHERKIN) --no-source --no-pickles $< | jq --sort-keys --compact-output "." > $<.ast.ndjson
# .DELETE_ON_ERROR: acceptance/testdata/%.feature.ast.ndjson

acceptance/testdata/%.feature.ast.ndjson: testdata/%.feature testdata/%.feature.ast.ndjson .built
	mkdir -p `dirname $@`
	$(RUN_GHERKIN) --no-source --no-pickles $< | jq --sort-keys --compact-output "." > $@
	diff --unified <(jq "." $<.ast.ndjson) <(jq "." $@)

# Generate
# acceptance/testdata/%.feature.pickles.ndjson: testdata/%.feature .built
# 	mkdir -p `dirname $@`
# 	$(RUN_GHERKIN) --no-source --no-ast $< | jq --sort-keys --compact-output "." > $<.pickles.ndjson
# .DELETE_ON_ERROR: testdata/%.feature.pickles.ndjson

acceptance/testdata/%.feature.pickles.ndjson: testdata/%.feature testdata/%.feature.pickles.ndjson .built
	mkdir -p `dirname $@`
	$(RUN_GHERKIN) --no-source --no-ast $< | jq --sort-keys --compact-output "." > $@
	diff --unified <(jq "." $<.pickles.ndjson) <(jq "." $@)

# Generate
# acceptance/testdata/%.feature.source.ndjson: testdata/%.feature .built
# 	mkdir -p `dirname $@`
# 	$(RUN_GHERKIN) --no-ast --no-pickles $< | jq --sort-keys --compact-output "." > $<.source.ndjson
# .DELETE_ON_ERROR: acceptance/testdata/%.feature.source.ndjson

acceptance/testdata/%.feature.source.ndjson: testdata/%.feature testdata/%.feature.source.ndjson .built
	mkdir -p `dirname $@`
	$(RUN_GHERKIN) --no-ast --no-pickles $< | jq --sort-keys --compact-output "." > $@
	diff --unified <(jq "." $<.source.ndjson) <(jq "." $@)

# Generate
# acceptance/testdata/%.feature.errors.ndjson: testdata/%.feature .built
# 	mkdir -p `dirname $@`
# 	$(RUN_GHERKIN) $< | jq --sort-keys --compact-output "." > $<.errors.ndjson
# .DELETE_ON_ERROR: acceptance/testdata/%.feature.ndjson

acceptance/testdata/%.feature.errors.ndjson: testdata/%.feature testdata/%.feature.errors.ndjson .built
	mkdir -p `dirname $@`
	$(RUN_GHERKIN) $< | jq --sort-keys --compact-output "." > $@
	diff --unified <(jq "." $<.errors.ndjson) <(jq "." $@)

src/parser.rs: gherkin.berp gherkin-rust.razor berp/berp.exe
	-mono berp/berp.exe -g gherkin.berp -t gherkin-rust.razor -o $@
	# Remove BOM
	awk 'NR==1{sub(/^\xef\xbb\xbf/,"")}{print}' < $@ > $@.nobom
	mv $@.nobom $@

clean:
	cargo clean --package gherkin
	rm -rf .compared .built acceptance
.PHONY: clean

clobber: clean
	rm -rf src/parser.rs
.PHONY: clobber
