# OrCo
[![wakatime](https://wakatime.com/badge/github/InfiniteCoder01/orco.svg)](https://wakatime.com/badge/github/InfiniteCoder01/orco)

OrCo is a compiler toolchain focusing on performace and extensibility
[Developed on streams](https://www.youtube.com/playlist?list=PLvZASPqsD2VjqJ6968gEhoLlCn0i0rqHH)

## Goals

Some note I worte a long time ago. Doesn't really make sence to me, but should make sence for new developers:
> Note for developers:
> > Intermediate Representation is nesesary, because
> > we can't just invoke a backend(f.e. cranelift) and
> > tell it to declare a trait. And we can't just parse
> > a language into an IR, because of LSP support.

## Concerns
Some things might be concerning:
- Span. Spans are probably too heavy
- AST is made of Arc's, instead of centrual storadge and IDs
- Metadata. Proper way would be to have custom AST nodes inherit normal AST nodes, but Rust doesn't have inheritance

## Some guidelines which I'll probably forget soon
- Add `span: Span` filed into structs instead of using `Spanned<Struct>`. This will make code simpler. `Spanned` was added mainly for enums

## Roadmap for next few streams
You can watch me do this live on [Twitch](https://www.twitch.tv/infinitecoder01) and [Youtube](https://www.youtube.com/@InfiniteCoder02/)

Roadmap for now:
- [x] Symbols
- [x] Paths
- [x] Floats
- [x] Frontend-side diagnostics (and diagnostics refactor (and lints))
- [x] Fix cyclic Arc by implementing inner pointers
- [x] Reduce the use of `Spanned<Struct>`
- [x] Make IR first-class (a BIG refactor)
- [x] Remove Ariadne completely (+ lexer abort compilation)
- [x] Move spans to frontend?
- [x] Comptime type hints
- [x] Path as an operator \[cancelled\]
- [x] `orco::Path` borrowing names? \[cancelled\]
- [x] Get metadata traits out of macros
- [x] Reorganize IR Tree to hold references to modules. Maybe local resolve should only be in module?
- [x] Parent modules (`super::`)
- [ ] Fix lazy evaluation:
    - [ ] Extract part of TypeInference struct into something like LocalContext
    - [ ] Rename TypeInference to something like Context and rename all the functions
    - [ ] Remove lifetime from TypeInference/Context struct and make it shareable/cloneable
    - [ ] Isolate LocalContext for all ensure_evaluated
- [] Comptimes in blocks
- [ ] Structs
- [ ] Generics
- [ ] Operator Overloading & Traits
- [ ] Finish the interpreter
- [ ] Unwinding?
- [ ] Effect system?!
- [ ] Pointers
- [ ] Typecasts
- [ ] Arrays
- [ ] While loop
- [ ] C Frontend (and a blog post on it hopefully)
- [ ] Post-typechecking frontend-side checks
- [ ] Rust frontend
- [ ] Self-hosting
