# Markdown Localizer in Rust 

✍️ Some blog provider (for example: [YuQue](https://www.yuque.com/)) is really useful and convenient in daily use.

🤔 However you might actually want to save your post on github with a static blog repo
(to make sure you are not going to lose them when blog provider stop their services).

🥰 A Markdown localizer will help you to wipe out these worries. And I'm going to using this tools myself in daily.

## Alternate

When this repo is not ready, the following tools are suggested. 
1. 👍 [Markdown Image Localizer](https://github.com/TenviLi/markdown-image-localizer)
2. 😅 Nothing here currently

## Install

From source:
```sh
cargo install --path .
```

## Example Usage

```sh
cd examples
markdown-localizer-rs localize-image -s example.md
```