# Environment

- WSL2 Ubuntu20.04

```
❯ RUST_BACKTRACE=full cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/flume-sandbox`
thread 'tokio-runtime-worker' panicked at 'assertion failed: chan.queue.is_empty()', /home/hatoo/.cargo/git/checkouts/flume-32c877a2c1c920aa/3ed0f31/src/lib.rs:349:13
stack backtrace:
   0:     0x559a784da450 - std::backtrace_rs::backtrace::libunwind::trace::hdf911925cfd1a062
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/../../backtrace/src/backtrace/libunwind.rs:96
   1:     0x559a784da450 - std::backtrace_rs::backtrace::trace_unsynchronized::h1a4f248df23e18fe
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/../../backtrace/src/backtrace/mod.rs:66
   2:     0x559a784da450 - std::sys_common::backtrace::_print_fmt::hf3ec23fc59b676bd
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/sys_common/backtrace.rs:79
   3:     0x559a784da450 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h00fdb223b1b833ac
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/sys_common/backtrace.rs:58
   4:     0x559a784f427c - core::fmt::write::h1857a60b204f1b6a
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/core/src/fmt/mod.rs:1082
   5:     0x559a784d83a7 - std::io::Write::write_fmt::ha851958ed267cb97
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/io/mod.rs:1514
   6:     0x559a784dc790 - std::sys_common::backtrace::_print::hf25dbce5d8adc35c
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/sys_common/backtrace.rs:61
   7:     0x559a784dc790 - std::sys_common::backtrace::print::h40243408d050ccf7
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/sys_common/backtrace.rs:48
   8:     0x559a784dc790 - std::panicking::default_hook::{{closure}}::ha452abcc85cd3549
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/panicking.rs:208
   9:     0x559a784dc4dc - std::panicking::default_hook::h637245b92cbb00b1
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/panicking.rs:227
  10:     0x559a784dcdf3 - std::panicking::rust_panic_with_hook::h2f4c96dfd8ba524a
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/panicking.rs:577
  11:     0x559a7849662c - std::panicking::begin_panic::{{closure}}::h606c23af479f678e
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:506
  12:     0x559a78495689 - std::sys_common::backtrace::__rust_end_short_backtrace::hbd8908a8e6b71c6a
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs:153
  13:     0x559a78496557 - std::panicking::begin_panic::hfa5e38b29e81c0d4
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:505
  14:     0x559a7842bbb5 - flume::Shared<T>::send::hf25ab5bf2e7b0679
                               at /home/hatoo/.cargo/git/checkouts/flume-32c877a2c1c920aa/3ed0f31/src/lib.rs:349
  15:     0x559a78417991 - <flume::async::SendFuture<T> as core::future::future::Future>::poll::h081a59e4b58a2545
                               at /home/hatoo/.cargo/git/checkouts/flume-32c877a2c1c920aa/3ed0f31/src/async.rs:134
  16:     0x559a7841be64 - flume_sandbox::main::{{closure}}::{{closure}}::h6028d7f33503b8e3
                               at /home/hatoo/flume-sandbox/src/main.rs:10
  17:     0x559a784166c7 - <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::hcbfda76310ee35a2
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/mod.rs:79
  18:     0x559a78419058 - tokio::runtime::task::core::Core<T,S>::poll::{{closure}}::hd1bc1929ebc235ff
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/core.rs:173
  19:     0x559a78424bf7 - tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut::h6f224bc1be73cc9c
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/loom/std/unsafe_cell.rs:14
  20:     0x559a78418ca3 - tokio::runtime::task::core::Core<T,S>::poll::h9e31a0dc2d78e097
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/core.rs:158
  21:     0x559a7842368c - tokio::runtime::task::harness::Harness<T,S>::poll::{{closure}}::hf8c9b22f276b72b5
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/harness.rs:107
  22:     0x559a78425c00 - core::ops::function::FnOnce::call_once::hc5208e355fc4cd99
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
  23:     0x559a7841e28a - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hcaae4cac9399c602
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:308
  24:     0x559a7842971d - std::panicking::try::do_call::ha24292ef9c46397d
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:381
  25:     0x559a78429a4d - __rust_try
  26:     0x559a784292d5 - std::panicking::try::h264e7cf5c70146d6
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:345
  27:     0x559a7841e2ba - std::panic::catch_unwind::h3716fda3f4e215c1
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:382
  28:     0x559a78422fb8 - tokio::runtime::task::harness::Harness<T,S>::poll::hd4b38f4c2f9881fa
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/harness.rs:89
  29:     0x559a7841c8e0 - tokio::runtime::task::raw::poll::h7699ea701a214153
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/raw.rs:104
  30:     0x559a7849249f - tokio::runtime::task::raw::RawTask::poll::ha0dff6cfa7aa93e2
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/raw.rs:66
  31:     0x559a7847af91 - tokio::runtime::task::Notified<S>::run::h25a09d5eca5df34b
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/mod.rs:169
  32:     0x559a784836a2 - tokio::runtime::thread_pool::worker::Context::run_task::{{closure}}::hb9b13cfc6188260a
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/thread_pool/worker.rs:353
  33:     0x559a7847151c - tokio::coop::with_budget::{{closure}}::he83593a1d6d72159
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/coop.rs:127
  34:     0x559a784888be - std::thread::local::LocalKey<T>::try_with::hd831ce8458c00f45
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:265
  35:     0x559a7848668e - std::thread::local::LocalKey<T>::with::h2db49b903fcc9a74
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:241
  36:     0x559a78483504 - tokio::coop::with_budget::he1ebabba6de80db9
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/coop.rs:120
  37:     0x559a78483504 - tokio::coop::budget::hc46eec664ed6a267
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/coop.rs:96
  38:     0x559a78483504 - tokio::runtime::thread_pool::worker::Context::run_task::h76d576a993be78cc
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/thread_pool/worker.rs:352
  39:     0x559a78482ec2 - tokio::runtime::thread_pool::worker::Context::run::h1f07696dbaf62439
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/thread_pool/worker.rs:324
  40:     0x559a78482cb3 - tokio::runtime::thread_pool::worker::run::{{closure}}::h26d5e78ca755605f
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/thread_pool/worker.rs:309
  41:     0x559a78470846 - tokio::macros::scoped_tls::ScopedKey<T>::set::h4a96e2cc7e494f25
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/macros/scoped_tls.rs:63
  42:     0x559a78482bad - tokio::runtime::thread_pool::worker::run::h3e40f6085451ec9d
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/thread_pool/worker.rs:306
  43:     0x559a78482a1b - tokio::runtime::thread_pool::worker::Launch::launch::{{closure}}::hd300aae43cfd1efd
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/thread_pool/worker.rs:285
  44:     0x559a78444fbe - <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll::h7412971d9240bbc7
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/blocking/task.rs:41
  45:     0x559a784531fe - tokio::runtime::task::core::Core<T,S>::poll::{{closure}}::hc5e0ab865a9fa1b9
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/core.rs:173
  46:     0x559a78450efb - tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut::h9dbe36995557bf41
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/loom/std/unsafe_cell.rs:14
  47:     0x559a7845303e - tokio::runtime::task::core::Core<T,S>::poll::had169c7299e5444f
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/core.rs:158
  48:     0x559a784733f5 - tokio::runtime::task::harness::Harness<T,S>::poll::{{closure}}::hb40a7ea8d65e1ed8
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/harness.rs:107
  49:     0x559a78475110 - core::ops::function::FnOnce::call_once::h8b837f4fba75053e
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
  50:     0x559a78463acb - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::ha166a77715f1035a
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:308
  51:     0x559a78474b9f - std::panicking::try::do_call::h5aa6692bffc981bb
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:381
  52:     0x559a7847b96d - __rust_try
  53:     0x559a78474852 - std::panicking::try::h01c6c0bf59fc11de
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:345
  54:     0x559a784645ab - std::panic::catch_unwind::h4e1cea01fd0eb6e0
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:382
  55:     0x559a78472ee5 - tokio::runtime::task::harness::Harness<T,S>::poll::h15589dc6e8d7b403
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/harness.rs:89
  56:     0x559a78492592 - tokio::runtime::task::raw::poll::hc2d4d4061c568439
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/raw.rs:104
  57:     0x559a7849249f - tokio::runtime::task::raw::RawTask::poll::ha0dff6cfa7aa93e2
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/raw.rs:66
  58:     0x559a7847b001 - tokio::runtime::task::Notified<S>::run::h9ceecc967449a82e
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/task/mod.rs:169
  59:     0x559a784939af - tokio::runtime::blocking::pool::Inner::run::h18f860a24dcea44c
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/blocking/pool.rs:230
  60:     0x559a784936fe - tokio::runtime::blocking::pool::Spawner::spawn_thread::{{closure}}::{{closure}}::h53ace86f024839f1
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/blocking/pool.rs:210
  61:     0x559a7845d685 - tokio::runtime::context::enter::h16aeb3e677fdb656
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/context.rs:72
  62:     0x559a7845271f - tokio::runtime::handle::Handle::enter::hc4899c2e6ecc5752
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/handle.rs:76
  63:     0x559a7849378f - tokio::runtime::blocking::pool::Spawner::spawn_thread::{{closure}}::heac28a3d8abfedf5
                               at /home/hatoo/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/runtime/blocking/pool.rs:209
  64:     0x559a784516f0 - std::sys_common::backtrace::__rust_begin_short_backtrace::hb923d65a19d5350b
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs:137
  65:     0x559a7843d0f3 - std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}}::hee7c2c0f72833035
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:458
  66:     0x559a78463aa3 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h279cc0a13779094f
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:308
  67:     0x559a78474caf - std::panicking::try::do_call::hb215a53f218e40e9
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:381
  68:     0x559a7847b96d - __rust_try
  69:     0x559a78474a14 - std::panicking::try::h655aa1f44692733e
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:345
  70:     0x559a784645f3 - std::panic::catch_unwind::h54d0e4317351af6a
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:382
  71:     0x559a7843cedb - std::thread::Builder::spawn_unchecked::{{closure}}::h31a661a8c02c75b6
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:457
  72:     0x559a78474f5f - core::ops::function::FnOnce::call_once{{vtable.shim}}::h9c1f7fec30fee67a
                               at /home/hatoo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
  73:     0x559a784df1ca - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hd50938a7cbd66861
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/alloc/src/boxed.rs:1042
  74:     0x559a784df1ca - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hbdaa88962d74e8ba
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/alloc/src/boxed.rs:1042
  75:     0x559a784df1ca - std::sys::unix::thread::Thread::new::thread_start::h2e71bf448b68bc62
                               at /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/sys/unix/thread.rs:87
  76:     0x7fb0eadf4609 - start_thread
  77:     0x7fb0ead00103 - clone
  78:                0x0 - <unknown>
recv end
```