# Default values
VERSION_TYPE ?= patch
COMMIT_MESSAGE ?= "Release $(shell git describe --tags --abbrev=0 2>/dev/null || echo "v1.0.0")"
REMOTE ?= origin
BRANCH ?= master

# Get current version or default to 1.0.0
CURRENT_VERSION := $(shell git describe --tags --abbrev=0 2>/dev/null || echo "v1.0.0")
CURRENT_VERSION_NUM := $(shell echo $(CURRENT_VERSION) | sed 's/v//')

# Calculate new version based on type
ifeq ($(VERSION_TYPE),major)
    NEW_VERSION_NUM := $(shell echo $(CURRENT_VERSION_NUM) | awk -F. '{print $$1+1 ".0.0"}')
else ifeq ($(VERSION_TYPE),minor)
    NEW_VERSION_NUM := $(shell echo $(CURRENT_VERSION_NUM) | awk -F. '{print $$1 "." $$2+1 ".0"}')
else
    NEW_VERSION_NUM := $(shell echo $(CURRENT_VERSION_NUM) | awk -F. '{print $$1 "." $$2 "." $$3+1}')
endif

NEW_VERSION := v$(NEW_VERSION_NUM)

# Tasks
.PHONY: help commit release patch minor major

help:
	@echo "Available commands:"
	@echo "  make commit COMMIT_MESSAGE='your message' - Commit changes"
	@echo "  make release VERSION_TYPE=patch|minor|major COMMIT_MESSAGE='your message' - Commit and tag"
	@echo "  make patch - Quick patch release"
	@echo "  make minor - Quick minor release"
	@echo "  make major - Quick major release"
	@echo ""
	@echo "Current version: $(CURRENT_VERSION)"
	@echo "Next version: $(NEW_VERSION)"

commit:
	@echo "Committing changes..."
	git add .
	git commit -m "$(COMMIT_MESSAGE)"
	@echo "✅ Changes committed"

release: commit
	@echo "Creating tag $(NEW_VERSION)..."
	git tag $(NEW_VERSION)
	@echo "Pushing to $(REMOTE)/$(BRANCH)..."
	git push $(REMOTE) $(BRANCH)
	git push $(REMOTE) $(NEW_VERSION)
	@echo "✅ Release $(NEW_VERSION) completed!"

patch: VERSION_TYPE=patch
patch: release

minor: VERSION_TYPE=minor
minor: release

major: VERSION_TYPE=major
major: release

# Development helpers
build:
	cargo build

test:
	cargo test

test-verbose:
	cargo test -- --nocapture
	
test-coverage:
	cargo tarpaulin --out Html

check:
	cargo check

clean:
	cargo clean

# Show current status
status:
	@echo "Current version: $(CURRENT_VERSION)"
	@echo "Next $(VERSION_TYPE) version: $(NEW_VERSION)"
	@echo "Uncommitted changes:"
	@git status --porcelain || echo "No changes"
