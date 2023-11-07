## 2023-11-07, Version v0.1.8
### Commits
- [[`c79651f9a6`](https://github.com/dogue/jottem/commit/c79651f9a6482939294fe26cbb58740c0aa3abc6)] release: version 0.1.8 (Dogue)
- [[`f7c8c88de9`](https://github.com/dogue/jottem/commit/f7c8c88de98a2cc24146ce377290c72d592957d5)] feat: update modified time only if file changed on disk (Dogue)
- [[`42c1da5c48`](https://github.com/dogue/jottem/commit/42c1da5c485a6d06322952ce339bcc6798ebab28)] fix: edit existing note instead of returning an error when trying to create a note that already exists (Dogue)
- [[`33aba20a9c`](https://github.com/dogue/jottem/commit/33aba20a9c1e14f779ceb64799b53db59d64bafe)] Merge branch 'main' of github.com:dogue/jottem (Dogue)
- [[`b378434ce4`](https://github.com/dogue/jottem/commit/b378434ce4c815927a3d53917b753a8ebddbb6f4)] chore: cargo-dist and oranda ci setup (Dogue)
- [[`8e6577de80`](https://github.com/dogue/jottem/commit/8e6577de80bcb96623e5f502bd9113fd4b7894ab)] trim imports in lib.rs (Dogue)
- [[`9925160af2`](https://github.com/dogue/jottem/commit/9925160af2bef81d2e2d51912e21766713104f3e)] create utils and tags modules to trim fat from lib.rs (Dogue)
- [[`6a68459a39`](https://github.com/dogue/jottem/commit/6a68459a3935f011c63a085494887654eafddf74)] resize logo in readme (Dogue)
- [[`7e1598a38d`](https://github.com/dogue/jottem/commit/7e1598a38d64c45e98768706956587ee3d69436a)] Update README.md (Dogue)
- [[`9ddb005d4b`](https://github.com/dogue/jottem/commit/9ddb005d4b75cdbed55b1ea352cf4e0f9a6d5103)] add logo (Dogue)
- [[`599bcbc55f`](https://github.com/dogue/jottem/commit/599bcbc55fe06b36da9456466df8aa5abb89551a)] Update README.md (Dogue)
- [[`5576c01792`](https://github.com/dogue/jottem/commit/5576c0179230eb9bf9fd2c1dad7882c0bed6efa8)] Update README.md (Dogue)

### Stats
```diff
 .github/workflows/release.yml | 203 +++++++++++++++++++++++++++++++++++++++++++-
 .github/workflows/web.yml     |  98 +++++++++++++++++++++-
 .gitignore                    |   3 +-
 Cargo.lock                    |   2 +-
 Cargo.toml                    |  20 +++-
 README.md                     | 126 +++++----------------------
 jottem.png                    | Bin 0 -> 265328 bytes
 src/file.rs                   |   4 +-
 src/lib.rs                    | 192 +++--------------------------------------
 src/main.rs                   |  12 ++-
 src/tags.rs                   |  31 +++++++-
 src/utils.rs                  | 148 +++++++++++++++++++++++++++++++-
 tests/jottem.rs               |   6 +-
 13 files changed, 560 insertions(+), 285 deletions(-)
```


