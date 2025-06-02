# Force Send Sync

Tells the compiler things are Send and/or Sync.

## Reasons not to use this crate

Usecases for this crate are extremly rare. If you have a compiler error which mentions types not being `Send` and `Sync` and you do not understand precisly what is wrong and how to fix it, this crate is unlikely to solve it. It will only turn your compile time problem into a much messier one which might only show itself at runtime, occasionally.

## Reasons to use this crate

* You have a type which is actually `Send` and/or `Sync` but the compiler does not know it. Further you can not implement these yourself, because the code lives Upstream (maybe you could contribute there?).
* You have a really weird situation there safety of these depends on configuration read at runtime and you need a way to promote safety of types.