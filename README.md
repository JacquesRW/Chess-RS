# Chess-RS
Chess in Rust
Uses a negamax search with alpha-beta pruning, evaluates positons by performing a further quiescence serach (additional 4 ply) with beta and delta pruning.
Static evaluation of final nodes is material eval, piece square tables and tapers from an opening eval to endgame eval. 

```
Chess-RS
├─ src/
│  ├─ model/
│  │  ├─ engine/
│  │  │  ├─ constants.rs
│  │  │  ├─ eval.rs
│  │  │  ├─ minimax.rs
│  │  │  ├─ mod.rs
│  │  │  ├─ qmovegen.rs
│  │  │  ├─ quiesce.rs
│  │  ├─ board.rs
│  │  ├─ defs.rs
│  │  ├─ fen.rs
│  │  ├─ helper.rs
│  │  ├─ make_move.rs
│  │  ├─ mod.rs
│  │  ├─ movegen.rs
│  │  ├─ moves.rs
│  │  ├─ pieces.rs
│  │  ├─ unmake_move.rs
│  ├─ controller.rs
│  ├─ main.rs
│  ├─ puzzles.rs
├─ .gitignore
├─ cargo.toml
├─ README.md
├─ TODO.txt
```
