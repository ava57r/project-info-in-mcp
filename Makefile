FEATURE_SETS := \
	default \
	qdrant_store \
	fastembed \
	"fastembed,lance_store" \
	"fastembed,qdrant_store"

.PHONY: check check-default check-qdrant check-fastembed check-all help

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

default: check-all

check: ## Check default features
	cargo check --features default --all-targets

check-default: check

check-qdrant: ## Check qdrant_store without default
	cargo check --no-default-features --features qdrant_store --all-targets

check-fastembed: ## Check fastembed without default
	cargo check --no-default-features --features fastembed --all-targets

check-all: ## Check all feature combinations (like CI)
	@for set in $(FEATURE_SETS); do \
		echo "==> Checking: $$set"; \
		if [ "$$set" = "default" ]; then \
			cargo check --features default --all-targets || exit 1; \
		else \
			cargo check --no-default-features --features "$$set" --all-targets || exit 1; \
		fi; \
	done
	@echo "All feature combinations passed."
