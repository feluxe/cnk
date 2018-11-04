
# cnk


[![asciicast](https://asciinema.org/a/zkOvrIAvCJ0tAselyDglWNZdS.png)](https://asciinema.org/a/zkOvrIAvCJ0tAselyDglWNZdS)


### Description

"cnk" short for "chunk" is an Unix tool that allows splitting text (from stdin) into readable chunks that fit on one screen. You can use the ENTER key to go to the next chunk.


### Install

You need to have Rust/Cargo installed.

    cd /tmp

    git clone https://github.com/feluxe/cnk.git

    cd cnk

    cargo build --release

    cp target/release/cnk ~/bin/



### Test

    git clone https://github.com/feluxe/cnk.git

    cd cnk

    cargo build

    python tests/gen.py 500 | ./target/debug/cnk





