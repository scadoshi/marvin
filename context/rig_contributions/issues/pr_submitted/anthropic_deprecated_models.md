# Anthropic Deprecated Models in Rig

**Date discovered**: 2026-02-05
**Status**: Fixed via PR to Rig repository
**GitHub Issue**: [#1370](https://github.com/0xPlaygrounds/rig/issues/1370)

## Summary

While building scotbot and testing the `/model` command, discovered that several of Rig's pre-defined Anthropic model constants pointed to deprecated or retired models, causing 404 errors from Anthropic's API.

## The Problem

### Symptoms
When trying to use certain model constants from `rig::providers::anthropic::completion`, requests failed with:

```
Error: CompletionError: HttpError: Invalid status code 404 Not Found with message:
{
  "type":"error",
  "error":{
    "type":"not_found_error",
    "message":"model: claude-3-7-sonnet-latest"
  }
}
```

### Affected Models

| Rig Constant | Model String | Status | Issue |
|--------------|--------------|--------|-------|
| `CLAUDE_3_5_SONNET` | `claude-3-5-sonnet-latest` | ❌ **Retired** Oct 28, 2025 | 404 error |
| `CLAUDE_3_7_SONNET` | `claude-3-7-sonnet-latest` | ⚠️ **Deprecated**, retires Feb 19, 2026 | 404 error |
| `CLAUDE_3_5_HAIKU` | `claude-3-5-haiku-latest` | ⚠️ Unclear (possibly redirecting) | Works but unreliable |
| `CLAUDE_4_OPUS` | `claude-opus-4-0` | ⚠️ Outdated alias | Works but not current |
| `CLAUDE_4_SONNET` | `claude-sonnet-4-0` | ⚠️ Outdated alias | Works but not current |

### Working Models
| Rig Constant | Status |
|--------------|--------|
| `CLAUDE_3_5_HAIKU` (as haiku 3) | ✅ Works |
| `CLAUDE_4_OPUS` (as opus 4.0) | ✅ Works |
| `CLAUDE_4_SONNET` (as sonnet 4.0) | ✅ Works |

## Root Cause

Anthropic regularly deprecates and retires older model versions:

1. **Claude 3.5 Sonnet models retired** on October 28, 2025
   - `claude-3-5-sonnet-20240620` - RETIRED
   - `claude-3-5-sonnet-20241022` - RETIRED
   - `-latest` alias no longer resolves

2. **Claude 3.7 Sonnet deprecated** on October 28, 2025, retires February 19, 2026
   - `claude-3-7-sonnet-20250219` - Still accessible but deprecated
   - `-latest` alias may not work reliably

3. **Rig's constants were not updated** to reflect these changes

## Current Active Models (February 2026)

Per [Anthropic's official documentation](https://docs.anthropic.com/en/docs/about-claude/models):

| Model | API ID | Alias |
|-------|--------|-------|
| **Claude Opus 4.6** | `claude-opus-4-6` | `claude-opus-4-6` |
| **Claude Sonnet 4.5** | `claude-sonnet-4-5-20250929` | `claude-sonnet-4-5` |
| **Claude Haiku 4.5** | `claude-haiku-4-5-20251001` | `claude-haiku-4-5` |

## The Fix

### Changes Made to Rig

Updated `/Users/scottyrayfermo/Developer/rig/rig/rig-core/src/providers/anthropic/completion.rs`:

**Before:**
```rust
pub const CLAUDE_4_OPUS: &str = "claude-opus-4-0";
pub const CLAUDE_4_SONNET: &str = "claude-sonnet-4-0";
pub const CLAUDE_3_7_SONNET: &str = "claude-3-7-sonnet-latest";
pub const CLAUDE_3_5_SONNET: &str = "claude-3-5-sonnet-latest";
pub const CLAUDE_3_5_HAIKU: &str = "claude-3-5-haiku-latest";
```

**After:**
```rust
/// `claude-opus-4-6` completion model (latest Opus)
pub const CLAUDE_OPUS_4_6: &str = "claude-opus-4-6";
/// `claude-sonnet-4-5-20250929` completion model (latest Sonnet)
pub const CLAUDE_SONNET_4_5: &str = "claude-sonnet-4-5-20250929";
/// `claude-haiku-4-5-20251001` completion model (latest Haiku)
pub const CLAUDE_HAIKU_4_5: &str = "claude-haiku-4-5-20251001";
```

### Files Updated
- **Model constants**: `src/providers/anthropic/completion.rs`
- **Examples** (14 files): All updated to use new constants
- **Tests**: `src/agent/prompt_request/streaming.rs`
- **Documentation**: `src/providers/anthropic/mod.rs`

**Total**: 17 files changed

## Impact

### Before Fix
Users would encounter:
- Runtime 404 errors when using deprecated model constants
- Confusion about which models actually work
- No way to access the latest models (Opus 4.6, Sonnet 4.5, Haiku 4.5)

### After Fix
Users can:
- Use current, supported Anthropic models via clear constants
- Avoid 404 errors from deprecated models
- Access the latest Claude 4.6 and 4.5 models

## Lessons Learned

### About Anthropic's API
- Model strings use dated versions (e.g., `claude-sonnet-4-5-20250929`)
- `-latest` aliases may not be reliable for all model families
- Anthropic provides 60+ days notice before retiring models
- Active models have clear deprecation timelines in their docs

### About Contributing to Open Source
1. **Research thoroughly** - Verified model status through official docs and deprecation notices
2. **Document the problem** - Created clear issue with reproduction steps
3. **Follow conventions** - Used conventional commit format (`fix:` prefix)
4. **Test comprehensively** - Ran `cargo fmt`, `cargo clippy`, `cargo test`
5. **Update everything** - Constants, examples, tests, documentation
6. **Reference issues** - Used "Fixes #1370" to auto-link and close issue

### About Rig's Architecture
- Model constants live in `providers/anthropic/completion.rs`
- Examples extensively use model constants (makes updates important)
- Rig uses a monorepo structure with integration crates
- Contributing guidelines are thorough and well-documented

## References

- **Anthropic Models Overview**: https://docs.anthropic.com/en/docs/about-claude/models
- **Anthropic Model Deprecations**: https://docs.anthropic.com/en/docs/about-claude/model-deprecations
- **Rig Repository**: https://github.com/0xPlaygrounds/rig
- **Issue #1370**: https://github.com/0xPlaygrounds/rig/issues/1370
- **PR Branch**: `fix/update-deprecated-anthropic-models`

## Applying to Scotbot

After the PR is merged and a new Rig version is released, scotbot should:
1. Update `Cargo.toml` to use the new Rig version
2. Update `src/chat/mod.rs` to use new constant names:
   ```rust
   use rig::providers::anthropic::completion::{
       CLAUDE_OPUS_4_6,    // was CLAUDE_4_OPUS
       CLAUDE_SONNET_4_5,  // was CLAUDE_4_SONNET
       CLAUDE_HAIKU_4_5,   // was CLAUDE_3_5_HAIKU
   };
   ```
3. Update model options list in `ChatState::new()`

## Key Takeaway

Always verify that third-party library constants and API strings are current, especially for rapidly evolving AI/ML APIs. What works today may be deprecated tomorrow.
