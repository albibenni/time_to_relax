## Why

Youtube is stealing so much of my time and energy.
I need to make it conscious when and how I use it as a distraction or tool to study.

## How does it work

### Compile it

1. The prerequisite is having rust installed [Rust install](https://www.rust-lang.org/tools/install)
2. Compile with `Cargo build --release`

### How does it block the websites

It make use of **linux** `/etc/hosts` file to block the domain, in this case by default **YouTube** and require a manual action to unlock it.

- Being on **root** folders it requires _sudo permission_ to execute
  - Consider adding and alias on your `.bashrc`/`.zshrc` config like so:
    - `alias relax="sudo ~/.dotfiles/scripts/scripts/rust_can_make_you_focus coding"`
    - the command will be: `relax 10` (relax for 10 minutes, then will resume the blocked domains)
