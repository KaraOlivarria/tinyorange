# Examples

Demonstration books showing the `.tx` format and repository layout from
[docs/txt/spec.md](../docs/txt/spec.md) and
[docs/txt/format.md](../docs/txt/format.md).

- [`algebra-1-demo/`](algebra-1-demo/) — one chapter of an Algebra I book
  exercising the core block vocabulary: objectives, definitions, worked
  examples, exercises with teacher-audience answers, a figure with required
  alt text, cross-references, glossary terms, and standards alignment.

These files are the format spec's test fixtures in spirit: when txt's
parser lands, they become actual golden tests (`txt build` must accept
them; deliberate-error variants must be rejected with good messages).

Status note: txt does not exist yet — these files are hand-authored to the
v0.1 format draft and will evolve with it.
