# RTLS

## Architecture

```
┌───────────┐           ┌─────────────┐
│ Emulator  │WebSocket  │ Server      │
│           ├──────────►│ ┌─────────┐ │
│           │ 2794      │ │Engine   │ │
└───────────┘           └─┴─────────┴─┘
                              ▲
┌───────────┐                 │
│  Front    │ WebSocket       │
│ ┌───────┐ ├─────────────────┘
│ │WASM   │ │ 2794
└─┴───────┴─┘
```

Todo:
- [ ] Update front-end to use raw webpack, without wasm to simplify dependencies.
