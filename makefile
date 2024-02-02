.DEFAULT_GOAL := help

.PHONY: help
help: ## List all the command helps.
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY: init-pre-commit
init-pre-commit: ## Init pre-commit.
	@pre-commit install
	@pre-commit install --hook-type commit-msg
