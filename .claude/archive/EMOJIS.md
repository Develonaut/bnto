# 🍱 Approved Emojis for Bnto

## Usage Guidelines

Use these sushi-themed emojis consistently across:
- CLI output and help text
- Documentation (README, guides, .claude/ docs)
- Log messages
- Error messages
- Progress indicators

## Approved Emoji Set

```go
// Sushi-themed emojis for general use
var Sushi = []string{
    "🍣", // sushi
    "🍙", // onigiri
    "🥢", // chopsticks
    "🍥", // fish cake
    "🍱", // bnto box - PRIMARY ICON
    "🍜", // ramen
    "🍡", // dango
    "🍢", // oden
    "🦐", // shrimp
    "🦑", // squid
    "🐟", // fish
    "🍤", // fried shrimp
    "🥟", // dumpling
    "🥡", // takeout box
    "🍶", // sake
    "🍵", // teacup
    "🥠", // fortune cookie
    "🧋", // bubble tea
}
```

## Primary Icon

**🍱 Bento Box** - Use this as the primary icon for:
- App logo/branding
- CLI welcome messages
- Documentation headers
- GitHub repo icon

## Suggested Usage

- **🍱 Success:** "Bnto served successfully!"
- **🍙 Neta:** Reference to individual workflow nodes
- **👨‍🍳 Itamae:** Reference to orchestration engine
- **🥢 Preparation:** Building, preparing workflows
- **🍣 Execution:** Running workflows
- **🦐 Data:** Input/output data
- **🍵 Logging:** Progress updates, tailing
- **🥡 Storage:** File operations, saving bntos

## Examples

```
✓ 🍱 Bnto served in 1.2s
✓ 🍙 3 neta executed successfully
✗ ⚠️  Blender render failed (product-23.png)
⟳ 🍵 Rendering product 45/100... 67% complete
```

---

**DO NOT** use non-sushi emojis except for universal symbols:
- ✓ Checkmark (success)
- ✗ X mark (failure)
- ⚠️  Warning
- ⟳ Loading/progress
- ℹ️  Information
