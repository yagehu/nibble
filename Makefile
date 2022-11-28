.PHONY: dev
dev:
	terraform -chdir=infra/dev/server apply -auto-approve

.PHONY: dev-down
dev-down:
	terraform -chdir=infra/dev/server destroy -auto-approve
