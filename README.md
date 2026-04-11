# cuda-narrative

Narrative construction — agents build explanations from events, memory, and goals with themes and similarity search (Rust)

Part of the Cocapn cognitive layer — how agents think, decide, and learn.

## What It Does

### Key Types

- `NarrativeElement` — core data structure
- `NarrativeArc` — core data structure
- `NarrativeTemplate` — core data structure
- `NarrativeEngine` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-narrative.git
cd cuda-narrative

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_narrative::*;

// See src/lib.rs for full API
// 10 unit tests included
```

### Available Implementations

- `NarrativeArc` — see source for methods
- `NarrativeTemplate` — see source for methods
- `NarrativeEngine` — see source for methods

## Testing

```bash
cargo test
```

10 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: cognition
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates

- [cuda-confidence-cascade](https://github.com/Lucineer/cuda-confidence-cascade)
- [cuda-deliberation](https://github.com/Lucineer/cuda-deliberation)
- [cuda-reflex](https://github.com/Lucineer/cuda-reflex)
- [cuda-goal](https://github.com/Lucineer/cuda-goal)
- [cuda-fusion](https://github.com/Lucineer/cuda-fusion)
- [cuda-attention](https://github.com/Lucineer/cuda-attention)
- [cuda-emotion](https://github.com/Lucineer/cuda-emotion)
- [cuda-learning](https://github.com/Lucineer/cuda-learning)
- [cuda-skill](https://github.com/Lucineer/cuda-skill)

## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
