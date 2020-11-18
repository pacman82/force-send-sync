# Force Send Sync

Tells the compiler things are Send and/or Sync.

## Reasons not to use this crate

If you should not know what `Send` and `Sync` means this crate is likely not what you are looking
for. Usecases for this are extremly rare. This crate is unlikely to solve your problem and much more
likely to turn the problem you have into a much messier one.

## Reasons to use this crate

* You have a type which `Send` and/or `Sync` but the compiler does not know it. Further you can not
implement these yourself, because the code lives Upstream (maybe you could contribute there?).
* You have a really weird situation there safety of these depends on configuration read at runtime
and you need a way to promote safety of types.