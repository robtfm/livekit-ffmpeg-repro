built against 0.1.3, i didn't get an automated windows build against main working.

other platform builds not tested (ffmpeg should build fine on all platforms, livekit probably needs `rustflags = ["-C", "target-feature=+crt-static"]` on non-win platforms)

to build locally you will need to copy the `package.yml` action steps to get ffmpeg installed.

```
livekit-ffmpeg-repro> cargo run
warning: function `not_used` is never used
 --> src\main.rs:8:10
  |
8 | async fn not_used() {
  |          ^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `livekit-ffmpeg-repro` (bin "livekit-ffmpeg-repro") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target\debug\livekit-ffmpeg-repro.exe`
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ffmpeg::Error(1330794744: Protocol not found)', src\main.rs:5:108
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\livekit-ffmpeg-repro.exe` (exit code: 101)
```