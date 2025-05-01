## Why

### My Goal

Youtube is stealing much of my time and energy out of my day.
I need to make it conscious when and how I use it as a distraction or as a tool to study.

This is why I wanted to create an app to block by default the website that distract me, in my case `youtube.com`, and allow me to unblock it with a **cli** command and a specified **time**.

## How does it work

### Compile it

1. The prerequisite is having rust installed [Rust install](https://www.rust-lang.org/tools/install)
2. Compile with `Cargo build --release`

### How does it block the websites

It make use of **linux** `/etc/hosts` file to block the domain, in this case by default **YouTube** and require a manual action to unlock it.

- Being on **root** folders it requires _sudo permission_ to execute
  - Consider adding and alias on your `.bashrc`/`.zshrc` config like so:
    - `alias relax="sudo ~/.dotfiles/scripts/scripts/time_to_relax"`
    - the command will be: `relax 10` (relax for 10 minutes, then will resume the blocked domains)

## How to Use it

The next parts will assum you added the alias `relax` to your bash config.

### Setup File - First Run

1. First run it with the command `relax setup youtube.com` and every websites you would like to block distraction from.
   - It assumes you add the domain extension is added, else it won't run.
   - The `www` is not required.
2. Once you created your first setup it will generate the `/etc/hosts` file to block the specified websites and run `sudo dscacheutil -flushcache` so that will apply the mods and the websites are effectively blocked.

### Enjoy Free time

1. Run `relax` followed by the **minutes** of time you want to relax:

```sh
relax 20
```

2. When the time expires, a message will notify you that time is up and the websites will be blocked once again
