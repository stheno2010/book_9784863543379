写経
======================

# 目次

- [概要](#概要)
- [環境](#環境)
- [解答](#解答)
- [まとめ](#まとめ)

## 概要
| 書名 | 基礎から学ぶ 組込みRust |
----|---- 
| ISBN-13 | 9784863543379 |

## 環境
- WioTerminal

## 解答
[著者ページ](https://github.com/tomoyuki-nakabayashi/Embedded-Rust-from-Basics)

## まとめ
- Ubuntu 20.04 LTS で始めたが、4章のLチカで、ブートローダーモードにしても、cargo hf2がデバイスを見つけられず失敗する。Wio TerminalのArduinoIDEでのLチカは成功したので、cargo hf2かudev絡みだと思う。
- Windows 10 で、4章のLチカ、成功。ブートローダーモードで一度書き込むと、勝手に通常モードに戻るようで、連続書き込みができない。

