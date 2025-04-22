{{#include ../../include/header016.md}}

# Kakoune

If you are a Kakoune user and you'd like something to be added to this page,
please file a [GitHub Issue][project::cb::issues].

## Rust Language Support (LSP with Rust Analyzer)

You can use [`kakoune-lsp`][project::kakoune-lsp] with `rust-analyzer`.

You want to install just the RA server, without the official VSCode plugin.

You can manage it via `rustup`:

```sh
rustup component add rust-analyzer
```

To set up `kak-lsp`, add the following to `~/.config/kak/kakrc`:

```kak
eval %sh{kak-lsp}
lsp-enable
```

You want to create a keybinding to open the LSP menu, from where you can
access all the various actions/commands that Rust Analyzer can do to assist
you as you code. For example, to bind it to the `R` key:

```kak
map global normal "R" ": enter-user-mode lsp<ret>" -docstring "LSP mode"
```

There are other options that you might like. For example, to automatically

```kak
# automatically highlight the symbol under the cursor to show where it is used
set global lsp_auto_highlight_references true
# automatically show help for function signatures
lsp-auto-signature-help-enable
```

To find out what else is available, type `:lsp-enable` and see what shows
up in the list of autocompletions / suggestions. You can run these commands
manually to control the relevant features, or add them to your `kakrc`,
if you want them to be enabled by default.

## General Rust settings

To set various standard Kakoune editor options for Rust:

```kak
hook global BufSetOption filetype=rust %{
    set buffer tabstop 4
    set buffer indentwidth 4
    set buffer formatcmd 'rustfmt'
    expandtab
    # etc ...
}
```

## Autoformatting

To manually trigger code formatting, use the keybinding from the LSP menu.
For example, if you bound the LSP menu to `R`, you can just press `Rf`.

If you would like to format on save, add the following to your `kakrc`:

```kak
hook global BufSetOption filetype=rust %{
    hook buffer BufWritePre .* lsp-formatting-sync
}
```

Personally, I prefer to instead create a separate command for formatting and
saving, so that it is opt-in and does not happen every time I save a file:

```kak
define-command W %{
    lsp-formatting-sync
    w
}
```

Now I can just type `:W` to save with formatting and `:w` to save without formatting.

## Configuring Rust Analyzer

You might need to tweak various settings for the RA server itself, similar to
how you can do in other editors (like VS Code).

Copy the following default configuration into your `kakrc`:

```kak
hook -group lsp-filetype-rust global BufSetOption filetype=rust %{
    set-option buffer lsp_servers %{
        [rust-analyzer]
        root_globs = ["Cargo.toml"]
        single_instance = true
        [rust-analyzer.experimental]
        commands.commands = ["rust-analyzer.runSingle"]
        hoverActions = true
        [rust-analyzer.settings.rust-analyzer]
        # See https://rust-analyzer.github.io/manual.html#configuration
        # cargo.features = []
        check.command = "clippy"
        [rust-analyzer.symbol_kinds]
        Constant = "const"
        Enum = "enum"
        EnumMember = ""
        Field = ""
        Function = "fn"
        Interface = "trait"
        Method = "fn"
        Module = "mod"
        Object = ""
        Struct = "struct"
        TypeParameter = "type"
        Variable = "let"
    }
}
```

(this was obtained from the output of the `kak-lsp` command, which is
evaluated to set up `kak-lsp`)

You can now customize it to your liking. The various options for RA go under
the `[rust-analyzer.settings.rust-analyzer]` section.
