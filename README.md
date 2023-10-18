# test-signal-xous
This repo contains hacked apart bits of libsignal with the goal of supporting xous for the precursor.

# Goals
We want to provide a client for signal for the [xous](https://github.com/betrusted-io/xous-core) operating system.
There is a a high level crate [presage](https://github.com/whisperfish/presage) which provides what one
needs to write a signal client for a full OS, but we have different constraints. Xous has limited support for libstd and in particular does not have suport tokio.
That crate relies on [libsignal-service-rs](https://github.com/whisperfish/libsignal-service-rs), which we should be able to support on xous. This in turn relies on the rust libraries maintained by
signal themselves [libsignal](https://github.com/signalapp/libsignal). `libsignal` libsignal not present a public api (or even publish on crates.io), as they still seem to consider it 
for internal use. The point of this repo is to determine at what point in the chain from something like `presage` to `libsignal` we need to fork to support the precursor with the goal being to rely on as much 
upstream code as possible.

Currently, this repo contains some bits
[libsignal](https://github.com/signalapp/libsignal).
I am mostly just hacking this apart to learn how everything fits together. Once I have a clear idea, I will make a new `signal-xous` repo.

## Tasks
[ ] Set up a CI pipeline to discover build issues which might crop up with `libsignal`. Ideally, `libsignal-serivce-rs` will fix any issues, but having a carnary to indicate any breaking api changes up coming would be nice, as we might be able to fix the changes and upstream them. 

[ ] CI pipeline for integration tests with `libsignal-service-rs`

[ ] Determine what parts of presage make sense to extract into a xous specific signal client, i.e. Use PDDB for storage and no dep on tokio. Likely, this is none of it as we will likely have hack apart `libsignal-service-rs`.

[x] Use libsignal to encrypt and decrypt a message, verify functionality in renode. Currently, the tested in the binary `test_libsignal_protocol`.

[ ] Use `libsignal` to encrypt and decyrpt a message, verify functionality on hardware.

[ ] use `libsignal-service-rs` to register a device as a client. This will almost certainly not work, but gives us a good way to understand how the pieces fit together. We will try to run this on renode. On the off chance that it ends up begin easy, we will also test on hardware.

[ ] libsignal uses two different versions of [curve25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek). Specifically, They use the standard version in libsignal-protocol create (`curve25519-dalek = { version = "4.0.0", features = ["digest"] }`), and their own fork [https://github.com/signalapp/curve25519-dalek/](https://github.com/signalapp/curve25519-dalek/tree/main), which they use specifically in zkgroup. Their changes seems to boil down to something called [lizard2](https://github.com/signalapp/curve25519-dalek/tree/main/curve25519-dalek/src/lizard). We want to eventually link to the hardware acceleration on the precursor. We need to make sure this fork is compatible. 

[ ] It would be nice to set up a test server using the [Signal-Server](https://github.com/signalapp/Signal-Server), to aide in integration tests. Using one's real account might be quicker to get started, but if this becomes cumbersome, this is the way to go.
