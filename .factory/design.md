# Visual system: survey the toolchain

## Direction

Topographic cartography treats a repository as terrain that must be surveyed before an agent crosses it. Contour lines show discovered layers. Survey pins mark language servers, formatters, and validation commands. The interface uses map-sheet margins and compact field notes instead of a generic software dashboard.

The visual system is intentionally single-mode. A warm paper field keeps terminal output and dense evidence legible. Dark ink, not glowing gradients, carries hierarchy.

## Palette

| Token | Value | Use |
| --- | --- | --- |
| `--paper` | `#F4F0E6` | Page background; a field-map stock |
| `--paper-deep` | `#E7E0D0` | Recessed map panels |
| `--ink` | `#17231D` | Primary text and terminal ground |
| `--ink-soft` | `#4B5B51` | Secondary copy (7.1:1 on paper) |
| `--contour` | `#7B4024` | Elevation lines and structural accents |
| `--pine` | `#1D674D` | Primary actions and ready states |
| `--pine-dark` | `#124535` | Hover and strong labels |
| `--amber` | `#9A5D00` | Warnings with a text label |
| `--danger` | `#A1392C` | Failed checks with a text label |
| `--snow` | `#FFFCF5` | High-contrast inset surfaces |

## Type

The display face is **Fraunces**, self-hosted as a small WOFF2 subset, because its uneven serif forms resemble hand-set survey titles. Body and controls use the system sans stack for fast, neutral reading. Terminal evidence uses the system monospace stack. The scale is 48/38/28/21/17/14px and remains at least 16px for reading copy.

## Spacing and shape

Spacing follows an 8px base: 8, 16, 24, 32, 48, 64, 96. Major sections use broad 80–112px intervals. Panels have clipped map corners rather than rounded SaaS cards. One-pixel rules resemble folded survey sheets. Buttons are compact field labels with 4px corners and 44px minimum targets.

## Layout grammar

The first screen is an asymmetric two-column survey sheet: job and action on the left, a generated contour landscape behind a real terminal recording on the right. Below it, one continuous route line links the three steps. Results use a capability matrix rather than feature cards. At 390px, the terminal follows the copy and map marginalia becomes a quiet background.

## Motion

On first view, a survey line draws once in 700ms and result rows enter from their originating terminal edge in 180ms. No motion loops. With `prefers-reduced-motion: reduce`, drawing is removed and state changes are instant. Scroll behavior is native.

## Asset plan and provenance

- `site/public/topographic-survey.webp`: original generated editorial topographic terrain. Prompt: “An abstract topographic survey map of a code repository as layered terrain, precise rust contour lines on warm ivory archival paper, four small dark green survey markers connected by a thin route, subtle paper grain, flat screen-print editorial illustration, wide landscape composition, generous quiet areas, no letters, no numbers, no logos, no interface, no gradients, no watermark.” Generated on 2026-09-02 with `/opt/fleet/lib/gen-image.sh` using the factory image deployment. Converted locally to WebP; original prompt receipt is kept beside the source during generation and summarized here.
- `site/public/og-image.webp`: locally cropped and composed from the same original art, with no essential text embedded.
- Contour dividers, wordmark mark, favicon, status marks, and route line: original inline SVG/CSS geometry authored for this product.

All artwork is original to this repository. There are no stock assets or third-party runtime images.
