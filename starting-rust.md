# Starting with Rust

Rust has multiple toolchains. For this course, we'll be using a specific version (1.31.1). The easiest way to install Rust 1.31.1 is to first install `rustup`, the Rust version management tool. The directions below should work both on Linux and in Windows (assuming you've first installed the [Windows Linux Subsystem](https://docs.microsoft.com/en-us/windows/wsl/install-win10)). More on installing Rust is available from [the Rust website](https://www.rust-lang.org/tools/install).

## Install Rustup

```
curl https://sh.rustup.rs -sSf | sh
```

If you're uncomfortable running a script directly from the internet, you can first inspect the script by visiting the site [https://rustup.rs/](https://rustup.rs/).

## Install Rust `1.31.1`

Once you've successfully installed `rustup`, you can install version `1.31.1` of the Rust compiler simply by doing:

```
rustup install 1.31.1
```

This step will take a few minutes (it installs the Rust compiler, the package manager, and documentation). You may want to run `rustup component add rls-preview rust-analysis rust-src` to add full Rust Language Server options (Vim users will need this).

# Accessing Rust Documentation

`rustup` installs an offline version of the Rust Book. To access it, run `rustup doc --book` to load the documentation in your browser. You can access the Rust documentation online at [https://doc.rust-lang.org/](https://doc.rust-lang.org/).

# Setting up Your Editor

Below, we've documented steps for setting up Rust in the most common text editors. If you don't see your editor, let
Tim Steinberger (ts409415@ohio.edu) know so he can update this document. There is a (slightly
outdated) list of editors and IDEs with Rust support (and what features they support for Rust) available [here](areweideyet.com).

### Atom

Install the `ide-rust` package. More information on this package can be found [here](https://atom.io/packages/ide-rust).

### Emacs

Install `rust-mode`. The easiest way to install it is to configure MELPA with `M-x package install rust-mode`. If you do not want to
use MELPA you can add
```
(add-to-list 'load-path "/path/to/rust-mode/")
(autoload 'rust-mode "rust-mode" nil t)
(add-to-list 'auto-mode-alist '("\\.rs\\'" . rust-mode))
```
To your .emacs file. More information on rust-mode (including configuring MELPA) is available
[here](https://github.com/rust-lang/rust-mode).

### Sublime

Install the `Rust Enhanced` package. More information on the package can be found [here](https://github.com/rust-lang/rust-enhanced).

### Vim/Neovim

Install the [vim-lsp](https://github.com/prabirshrestha/vim-lsp), [asyncomplete](https://github.com/prabirshrestha/asyncomplete.vim),
[asyncomplete-lsp](https://github.com/prabirshrestha/asyncomplete-lsp.vim), and [async](https://github.com/prabirshrestha/async.vim)
plugins using your preferred package manager (requires vim8 or neovim).

Add the following to your .vimrc (or .config/nvim/init.vim for neovim users).
```
if executable('rls')
    au User lsp_setup call lsp#register_server({
        \ 'name': 'rls',
        \ 'cmd': {server_info->['rustup', 'run', 'nightly', 'rls']},
        \ 'root_uri':{server_info->lsp#utils#path_to_uri(lsp#utils#find_nearest_parent_file_directory(lsp#utils#get_buffer_path(), 'Cargo.toml'))},
        \ 'whitelist': ['rust'],
        \ })
endif
```
