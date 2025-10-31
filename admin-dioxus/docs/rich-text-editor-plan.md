# Rich Text Editor for Dioxus — Full‑Featured Plan (AST‑First)

This is a single, full‑feature implementation plan (no v1/v2 split). We will build an AST‑first, plugin‑extensible editor with a WYSIWYG surface, robust commands, and tight integration with our media store. The editor persists sanitized HTML for current APIs while owning a canonical JSON document model internally for correctness, history, and extensibility.

Primary goals (initial release is “complete”):
- Inline styles: bold, italic, underline, strikethrough, code, and text highlight.
- Typography: headings (H1–H4), paragraph, blockquote, code block, horizontal rule.
- Lists: bulleted, numbered, and task/checkbox lists with indent/outdent.
- Alignment: left, center, right, justify per block; text size and weight via curated classes.
- Links: add/edit/remove on text; wrap images with links (open in new tab option).
- Images: insert via existing media picker/upload; alt, caption, alignment, width presets.
- Embeds: safe iframe support for YouTube and X; strict host allowlist.
- Paste/clipboard: sanitize, normalize, auto‑link URLs, preserve common structure from Google Docs/Word where safe.
- Keyboard shortcuts, bubble menu, slash menu, toolbar; undo/redo history; selection mapping.
- Autosave via store and local draft fallback.

Non‑goals: collaborative editing/OT, comments/track changes, themeable custom fonts beyond system + Tailwind classes, arbitrary script embeds.


## Architecture Overview (AST‑first)

- Canonical document model in Rust (`Doc`, `Block`, `Inline`, `MarkSet`), stored in memory and serialized to JSON for history and draft.
- Renderer: Dioxus components map AST → DOM; selection state maps DOM ranges ↔ AST positions using stable `data-nodeid` and offset mapping.
- Command engine: pure functions produce Transactions (ops) that mutate the model; renderer reconciles; history records transactions with coalescing.
- HTML IO: `doc_to_html` and `html_to_doc` provide lossless or near‑lossless round‑trip within our supported node set; persisted `Post.content` is sanitized HTML.
- Plugin system: node/mark registries and command registration hooks to add features (images, embeds, tasks, tables later) without core changes.


## Data Model (AST)

Types (Rust):

```rust
// src/types.rs or src/types/editor.rs
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Doc(pub Vec<Block>);

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Block {
    pub id: String,
    pub kind: BlockKind,
    pub align: Option<BlockAlign>,
    pub attrs: serde_json::Value, // extensible: heading level, language, etc.
    pub children: Vec<Inline>,    // for paragraphs/headings/blockquote/list items
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BlockKind {
    Paragraph,
    Heading { level: u8 },
    BulletList { items: Vec<Block> },      // items are ListItem blocks
    OrderedList { items: Vec<Block> },     // items are ListItem blocks
    TaskList { items: Vec<Block> },        // items are TaskItem blocks
    ListItem,                              // generic li container
    TaskItem { checked: bool },
    Quote,
    CodeBlock { language: Option<String>, code: String },
    Image { src: String, alt: String, title: Option<String>, width: Option<u32>, height: Option<u32>, caption: Option<String> },
    Embed { provider: EmbedProvider, url: String, title: Option<String>, width: Option<u32>, height: Option<u32> },
    Rule,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EmbedProvider { Youtube, X, Generic }

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BlockAlign { Left, Center, Right, Justify }

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Inline {
    Text { text: String, marks: MarkSet, link: Option<Link> },
    HardBreak,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarkSet {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub code: bool,
    pub size: Option<TextSize>,
    pub highlight: Option<String>, // e.g., bg-yellow-200 token key
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TextSize { Small, Normal, Lead }

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Link { pub href: String, pub title: Option<String>, pub target_blank: bool }
```

Storage plan:
- Editor owns the AST as source of truth. DOM is a render target and selection source.
- Persist sanitized HTML to `Post.content` for API compatibility; also persist JSON draft to `localStorage` for resilience.
- Utilities: `doc_to_html(Doc) -> String` and `html_to_doc(&str) -> Doc` for import/export.


## Security & Sanitization

- At input and before save: sanitize HTML using a whitelist (tags, attributes, protocols) with `ammonia` on the Rust side.
- Links: enforce `rel="noopener noreferrer"` when `target="_blank"`.
- Images: allow only `src`, `alt`, `title`, `width`, `height`, `class`.
- Embeds: allow `iframe` with a strict host whitelist and attribute allowlist. For YouTube use `https://www.youtube.com/embed/{id}`; for X use `https://twitframe.com/show?url={tweet_url}`; disallow arbitrary iframe `src` by default.
- Paste filtering: strip styles and unknown tags; convert b/i/u/s to semantic tags; map inline styles to classes when reasonable.


## UI/UX Design

- Top toolbar (sticky above editor) for block and inline controls.
- Bubble menu shown on text selection for quick formatting/linking.
- Slash command (`/`) at paragraph start for inserting blocks: heading levels, list, quote, code, image, embed, rule.
- Drag‑and‑drop images onto the surface (uses existing upload flow) with live progress placeholders.
- Resize/align images via property popover (width presets and alignment in initial release); optional drag handles in a follow‑up.
- Content area styled with Tailwind Typography (`prose prose-neutral`) for WYSIWYG feel.


## Integration Points (project‑specific)

- Store: use `use_post()` for autosave: debounce 3–5s of inactivity, post `PostAutosavePayload { post_id, content: sanitized_html, updated_at }`.
- Media: use `use_media()` to open a picker modal listing `Media` (with upload slot), insert selected `media.file_url` with `alt` and optional caption.
- UI primitives: reuse `ui/shadcn` components for Dialog, Popover, Dropdown, Button, Icons.
- Container wiring: embed `Editor` inside `src/containers/blog_form/blog_form.rs`, replace the `textarea`/plain input for `content` with the editor; keep `BlogForm` content synchronized.


## Commands & Features (initial release)

- Inline marks: Bold (Cmd/Ctrl+B), Italic (Cmd/Ctrl+I), Underline (Cmd/Ctrl+U), Strikethrough, Code (Cmd/Ctrl+E).
- Text size: small/normal/lead via `text-sm`, `text-base`, `text-lg` on inline or block context; prefer block‑level size on paragraphs/headings.
- Block types: Paragraph, H1–H4, Quote, Code Block, Horizontal Rule.
- Lists: Bullet/Numbered/Task; `Tab` to indent, `Shift+Tab` outdent; `Enter` to create next item; toggle task checkbox.
- Align: left/center/right/justify on block wrappers (`text-left|center|right|justify`).
- Links: add/edit/unlink on selection; when an image is selected, wrap image node in `<a>…</a>`.
- Images: insert via picker, set alt text, optional caption (`<figure><img/><figcaption/></figure>`), alignment presets.
- Embeds: YouTube (accept normal or share URLs → normalize to embed URL), X (accept tweet URL → twitframe); generic iframe for a whitelisted set.
- Clear formatting: remove marks from selection; convert to paragraph.
- Clipboard: paste text/HTML → sanitize; auto‑link plain URLs.
- Autosave: throttle + debounce; local draft fallback in `localStorage` with `draft:post:{id}`.


## Files & Modules

New module tree:

```
src/components/editor/
  mod.rs
  editor.rs               // Root component; contenteditable surface + state wiring
  toolbar.rs              // Top toolbar
  bubble_menu.rs          // Selection bubble
  commands.rs             // Exec helpers (DOM operations, toggles)
  keymap.rs               // Keyboard shortcuts
  selection.rs            // Selection helpers
  sanitize.rs             // HTML whitelist via ammonia
  serialize.rs            // html<->doc (progressive fidelity)
  media_picker.rs         // Dialog using use_media() list + upload
  link_popover.rs         // Add/edit link UI
  embed_dialog.rs         // URL → embed block (YT/X)
  styles.css              // Editor‑specific overrides (caret, placeholders)
```

Wiring:
- Export `pub mod editor;` in `src/components/mod.rs`.
- Replace content field usage in `src/containers/blog_form/blog_form.rs` to mount `<Editor value=... on_change=... />`.


## HTML Policy (sanitizer allowlist)

Allowed tags: `p, h1, h2, h3, h4, blockquote, ul, ol, li, pre, code, strong, em, u, s, a, img, figure, figcaption, hr, br, iframe`.
Allowed attributes:
- `a[href|title|target|rel]` protocols: `http, https, mailto`.
- `img[src|alt|title|width|height|class]` protocols: `http, https, data:image/*` (optional, can disable).
- `iframe[src|width|height|allow|allowfullscreen|loading|title]` with host whitelist.
- global `class` for Tailwind utilities; no `style` attributes.


## Embeds

- YouTube: detect from `youtube.com/watch?v=...` or `youtu.be/...` → `https://www.youtube.com/embed/{id}`. Set `allow` to `accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share` and `allowfullscreen`.
- X: accept `https://twitter.com/{user}/status/{id}` or `https://x.com/...` → iframe `https://twitframe.com/show?url={encoded_url}`.
- Generic: consider `maps.google.com` and others in a later phase; otherwise block unknown hosts.


## Keyboard Shortcuts & A11y

- Cmd/Ctrl+B/I/U/E; Cmd/Ctrl+K open link dialog; Cmd/Ctrl+Shift+7/8 toggle ordered/unordered list; Cmd/Ctrl+Alt+1..4 set heading levels; Esc to close popovers.
- Focus management via roving tabindex in toolbar; ARIA labels for buttons; `aria-pressed` for toggle states.
- Screen reader: ensure meaningful labels on embeds/images; expose captions; `role="textbox"` with `aria-multiline="true"` on editor surface.


## Paste, Drag & Drop

- Paste: intercept `paste` event, read `text/plain` and `text/html`; prefer HTML sanitized; map inline styles → semantic where possible; auto‑link URLs.
- Drop: accept image files; call `use_media().upload(...)` and insert a temporary placeholder that updates to final URL on success.


## Undo/Redo & History

- Canonical, model‑level history with bounded size and coalescing (group typing bursts, merge adjacent mark toggles). Native browser history is suppressed inside the editable surface to avoid divergence.


## Performance

- Debounce heavy DOM→AST conversion and autosave; throttle measuring.
- Keep selection read/writes minimal; batch DOM mutations per command.
- Avoid large inline styles; prefer Tailwind classes.


## Testing Strategy

- Unit tests (wasm-bindgen test or headless) for:
  - sanitizer policy (`sanitize.rs`),
  - YouTube/X URL parsing and normalization,
  - html<->doc serialization helpers on small samples.
- Manual QA checklist per feature (toolbar toggles, lists, link/image/embed flows, paste) in `docs`.
- E2E (later): playwright‑style scripts if we introduce CI browser tests.

## Workstreams (all delivered in the initial release)

- Core AST, schema, serialization, and history engine.
- Renderer and selection mapping with stable node IDs.
- Commands and keymaps for marks, blocks, lists, alignment, rules, code, tasks.
- Toolbar, bubble menu, slash menu with a11y.
- Link popover, media picker dialog integration, embed dialog with URL normalization.
- Sanitization and paste pipeline; auto‑link and markdown-ish input rules.
- Autosave + draft recovery; Sonner feedback hooks for save states.


## Open Questions

- Do we want to store both HTML and JSON AST in the backend later? Currently we will store sanitized HTML only (for compatibility) and keep AST only in autosave/local drafts.
- Heading scope: allow H1 inside post body or reserve H1 for page title? Default to H2–H4 if title becomes the H1.
- Image CDN transforms (width/quality) — do we need size presets mapped to CDN parameters?
- Additional embed providers: Vimeo, Loom, CodePen — prioritize via feedback.


## Implementation Notes (project fit)

- Tailwind Typography plugin is already included; the editor surface will use `prose` plus utility overrides for selected states and placeholders.
- Reuse `ui/shadcn` Dialog/Popover for picker and link/embed UIs; reuse `components/portal_v2.rs` for portals.
- Media store (`src/store/media/*`) already supports upload and list; we’ll compose a `MediaPickerDialog` wrapper for editor use.
- Autosave endpoint exists; we’ll debounce calls and show a subtle “Autosaved” indicator using our Sonner toasts if desired.


## Acceptance (initial release)

- Author can: type text; set headings and alignment; style inline text; create bullet/numbered/task lists; insert links; insert images via media picker; insert YouTube/X embeds; paste from Word/Google Docs with sensible formatting.
- Saved `Post.content` contains sanitized HTML with only allowed tags/attrs; no broken links or insecure iframes.
- Undo/redo powered by the model history works across all operations; keyboard shortcuts match expectations; a11y labels exist and focus flows are correct.
- BlogForm integrates seamlessly; existing list/detail screens render content correctly.
