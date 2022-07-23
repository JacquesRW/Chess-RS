# Chess-RS
Command line chess in Rust.

Uses a negamax search with alpha-beta pruning, MVVLVA move ordering and iterative deepening. It evaluates positons by performing a further quiescence serach (additional depth but only considering captures, etc) with beta and delta pruning.
Static evaluation of final nodes is material eval, piece square tables and tapers from an opening eval to endgame eval. 

It is quite good.

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
│  ├─ main.rs
│  ├─ test.rs
├─ .gitignore
├─ cargo.toml
├─ README.md
├─ TODO.txt
```
