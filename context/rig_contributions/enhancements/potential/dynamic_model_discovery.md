***ALREADY REPORTED***


# Dynamic Model Discovery via Anthropic's List Models API

**Status**: Enhancement idea
**Related to**: Issue #1370 (deprecated model constants)

## Problem

Currently, Rig uses hardcoded model constants that become outdated when Anthropic deprecates or releases new models. This requires manual framework updates and causes 404 errors for users when models are retired.

## Potential Solution

Anthropic provides a `/v1/models` API endpoint that returns the current list of available models. This could be used to provide dynamic model discovery or validation.

---

## API Specification

### Endpoint
```
GET https://api.anthropic.com/v1/models
```

### Request Headers
```http
anthropic-version: 2023-06-01
x-api-key: YOUR_API_KEY
```

### Query Parameters (Optional)
- `limit` (number): Items per page (default: 20, max: 1000)
- `after_id` (string): Cursor for next page
- `before_id` (string): Cursor for previous page

---

## Example Request

```bash
curl https://api.anthropic.com/v1/models \
  -H "anthropic-version: 2023-06-01" \
  -H "x-api-key: $ANTHROPIC_API_KEY"
```

---

## Example Response

```json
{
  "data": [
    {
      "id": "claude-opus-4-6",
      "created_at": "2026-02-05T00:00:00Z",
      "display_name": "Claude Opus 4.6",
      "type": "model"
    },
    {
      "id": "claude-sonnet-4-5-20250929",
      "created_at": "2025-09-29T00:00:00Z",
      "display_name": "Claude Sonnet 4.5",
      "type": "model"
    },
    {
      "id": "claude-haiku-4-5-20251001",
      "created_at": "2025-10-01T00:00:00Z",
      "display_name": "Claude Haiku 4.5",
      "type": "model"
    },
    {
      "id": "claude-opus-4-5-20251101",
      "created_at": "2025-11-01T00:00:00Z",
      "display_name": "Claude Opus 4.5",
      "type": "model"
    }
  ],
  "has_more": false,
  "first_id": "claude-opus-4-6",
  "last_id": "claude-opus-4-5-20251101"
}
```

### Response Fields

- **`id`**: Exact string to use in API requests (e.g., `"claude-opus-4-6"`)
- **`created_at`**: ISO 8601 timestamp of model release
- **`display_name`**: Human-readable name
- **`type`**: Always `"model"`
- **`has_more`**: Whether more results exist
- **`first_id`** / **`last_id`**: Pagination cursors

---

## Potential Benefits

1. **Future-proof**: Reduces need for constant updates when models change
2. **Validation**: Could validate model IDs before API calls
3. **Discovery**: Users could enumerate available models programmatically
4. **Backwards compatible**: Could coexist with existing constants

---

## References

- **Anthropic API Docs**: https://docs.anthropic.com/en/api/models/list
- **Related Issue**: #1370
