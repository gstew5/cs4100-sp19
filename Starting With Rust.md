# Starting with Rust

Rust has multiple toolchains allowing for cross-compilation and switching between the stable, beta, and nightly releases.
To manage this, they wrote a tool called rustup that allows you to manage your toolchains. This is what we will use to install Rust.

## Installing Rust

To install rustup, first run the following command: `curl https://sh.rustup.rs -sSf | sh`. Next run: `rustup install stable` to
install the stable rust toolchain. This step will take a few minutes because it installs the rust compiler, the package manager,
and adds the rust source (used by racer and other tools your text editor may need for syntax highlighting) and documentation. You
may want to run `rustup component add rls-preview rust-analysis rust-src` to add full Rust Language Server options (Vim users will
need this).

## Accessing Rust Documentation

Rustup installs an offline version of the Rust Book. To access it, just run `rustup doc --book` and it will open your browser to
the documentation.

## Setting up Your Editor

Here are the steps for setting up some of the more common text editors to work with Rust. If you don't see your editor, let
Tim Steinberger (ts409415@ohio.edu) know and this document will be updated to include the necessary steps. There is a (slightly
outdated) list of editors and IDEs with Rust support (and what features they support for Rust) available [here](areweideyet.com).

### Atom

Install the ide-rust package. More information on this package can be found [here](https://atom.io/packages/ide-rust).

### Emacs

Install rust-mode. The easiest way to install it is to configure MELPA with `M-x package install rust-mode`. If you do not want to
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

