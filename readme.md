# macOS Haptics

This was a simple script to help me learn how Haptic Feedback works on macOS with the Force Touch trackpads.

> macOS 10.11+ is required for this to work, along with a trackpad that supports Force Touch.

It's kinda cool! My hope was that I could use this to make that oh-so-nice scroll wheel that happens on iOS, on my MacBook, in a web browser with the html `select` element. Unfortunately, it [doesn't seem](https://bugs.chromium.org/p/chromium/issues/detail?id=568727) like the chromium team [wants us](https://bugs.chromium.org/p/chromium/issues/detail?id=628924) to [have fun](https://bugs.chromium.org/p/chromium/issues/detail?id=788567), so you'll have to move your fingers up your trackpad while running this script instead.

It's pretty simle to run:

```bash
cargo clean && cargo run
```

Note that you do need to run `cargo clean` if you make changes, as sometimes the ObjectiveC code isn't re-linked, and you could run into some messy errors (not to mention the fact that the code won't be updated).

Hope you have as much fun with this as I did :)
