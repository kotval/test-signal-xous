# test-signal-xous
This repo contains hacked apart bits of libsignal with the goal of supporting xous for the precursor.

# Goals
We want to provide a client for signal for the [xous](https://github.com/betrusted-io/xous-core) operating system.
There is a a high level crate [presage](https://github.com/whisperfish/presage) which provides what one
needs to write a signal client for a full OS, but we have different constraints. Xous has limited support for libstd and in particular does not have suport tokio.
That crate relies on [libsignal-service-rs](https://github.com/whisperfish/libsignal-service-rs), which we should be able to support on xous. Currently, this repo contains some bits
[libsignal](https://github.com/signalapp/libsignal).
I am mostly just hacking this apart to learn how everything fits together as libsignal does not present a public api (or even publish on crates.io), as they still seem to consider it 
for internal use.

