# prusti-polyfill
A polyfill crate that allows people to compile your project with vanilla cargo, even though you use prusti macros/directives.

More specifically, things like #[trusted] will be defined as do-nothing macros unless ```--features verified``` is supplied to the build command, in which case it will refer to the appropriate prusti macro.

If you can think of better ways to do this, I appreciate feedback.

## Usage:
Replace the usual prusti header:
```
extern crate prusti_contracts;
use prusti_contracts::*;
```
with just:
```
prusti_polyfill::prusti_polyfill_init!();
```
Also, your Cargo.toml must have prusti-polyfill as a dependency (through git for now), and have a verified feature in your Cargo.toml, as in
```
[features]
default = []
verified = []
```
Then, when you compile, you can either do:
```cargo-prusti --features verified``` to verify using prusti, or
```cargo build``` to build, with no prusti needed.


## Motivation:
I'm starting a different project that I intend to be written in rust and be completely verified with prusti. However, I want my project to be able to be compiled either as a dependency or simply run by other developers without needing them to have prusti installed and configured. Prusti can't be added as a dependency like a crate.


## Other:
Here's the function I use to run prusti. I'm on an M1 mac, and the architecture difference gives me difficulty building from source or using the precompiled binaries, so I use the docker container. This isn't really related to this package, per se, but I'm hoping someone will see it and it will benefit them. It handles mounting the volumes for the docker container, preserves coloring, and filters out SLF4J warnings.
```
cargoprusti () {
	dir=$( while : ; do
	[ -f "Cargo.toml" ]  && echo "$PWD" && break
	[ $PWD != / ] && cd .. || break; done )
	docker run -t --platform linux/amd64 \
		-v $dir:/home/user \
		-v ~/.cargo/git:/usr/local/cargo/git \
		-v ~/.cargo/env:/usr/local/cargo/env \
		-v ~/.cargo/registry:/usr/local/cargo/registry \
		viperproject/prusti:latest cargo-prusti \
		--features verified "$@" \
		2>&1 | egrep -v SLF4J:
}
```












