# discord-homebrew-app

# Building

This uses [vivlim's ctru-rs](https://github.com/vivlim/ctru-rs). In order to compile this, one must follow that tutorial first, and then clone this to `app`.

This uses `nightly-2021-03-25-x86_64-unknown-linux-gnu` (run `rustup override set nightly-2021-03-25-x86_64-unknown-linux-gnu`).

In order to compile, I use a script, `make-script.sh`, to build it and copy it to an SD card. But, it just wraps around the Makefile.

To add it to your 3DS, move the `.3dsx` file that you compiled or that is included to `/3ds` on your SD card. Then, open the Homebrew browser and find the app.

# Features

-   Sending and receiving messages
-   Choosing where the proxy to connect to
-   "Spam" by spamming select
-   Sometimes doesn't crash
-   Beautiful* interface (*interface may not be beautiful)
-   Can handle all of ASCII and sometimes a few UTF8 characters can be sent
-   Doesn't get your account banned by not letting you use a user account
-   "Fast"
