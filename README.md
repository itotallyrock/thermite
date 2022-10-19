# Thermite Chess

A rust chess engine, with high hopes.

## Goals

Starting out, what do I want to get out of thermite.

These aren't all necessarily obtainable, but can still be objectives to work towards.

- Similar to [Stockfish](https://github.com/official-stockfish/Stockfish)
- Support UCI
- Fast search >500k nodes/s
- Efficient search: order moves, prune, cutoff, reduce depth
- NNUE based evaluation
- Have rankings on CCRL and lichess
- Be good at chess, ideally [>2200 ELO](https://ccrl.chessdom.com/ccrl/4040/), which might be used to mark v1.0

## Priorities

I mainly want to keep these as core values while I work through the [roadmap](#Roadmap).

These are subject to change as the project evolves, but should serve as the general direction.

1. Correctness
   1. Making sure it behaves like it should before resorting to shortcuts that might not always be correct
   2. Use lots of tests.
2. Maintainability
   1. This project should be running for years
   2. Write tons of documentation
   3. Design should be modular and easy to modify using granular features
3. Performance
   1. Bunch of benchmarks

## Roadmap
- [x] Finish planning phase 
- [ ] Build out core thermite types
- [ ] Add a bunch of tests and benchmarks
- [ ] Integrate with a CI service
  - `cargo-hack` for running with full feature-set
  - `clippy`
  - `test`
    - Ideally report on code coverage
  - `bench`
    - Ideally keep artefacts for tracking benchmark improvements or regressions
- [ ] Build move generation
- [ ] Finish perft binary
- [ ] Integrate with CD to build a release perft binary
  - Ideally automatically uploaded to github releases
- [ ]  Ensure fully correct move generation and board manipulation through tons of unit tests and comparing to stockfish
  - Try to use a proptest or fuzzer to create FENs to run through thermite perft and compare against stockfish's perft
- [ ] Add concept of 
- [ ] Implement rudimentary material evaluation
- [ ] Implement alpha-beta search
- [ ] Create a standalone search binary for human testing
- [ ] Create a ton of tests for finding efficient checkmates or known best moves
- [ ] Create a ton of benchmarks for a wide variety of chess positions
  - Ideally do this before implementing the majority of search enhancements to gauge their benefit
- [ ] Improve evaluation using piece square table
- [ ] Use a transposition table
- [ ] Take advantage of iterative deepening
- [ ] Utilize quiescence search
- [ ] Basic move ordering
  - PV move, killer moves, history heuristic, SEE for captures
- [ ] Quiescence move ordering
- [ ] Search using null window
- [ ] Prune futile moves
- [ ] Include opening book
- [ ] Implement MultiPV (evaluate the best N root moves to a high depth)
- [ ] Start utilizing all the threads for searching
- [ ] Create UCI library
- [ ] Marry UCI library with thermite search to create the root thermite binary
- [ ] Use CD to build release binaries
- [ ] Build out NNUE board evaluation