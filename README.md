Telly
============
 [![Current Version](https://img.shields.io/badge/version-0.1.0-green.svg)](https://github.com/IgorAntun/node-chat) 

This is a Rust command line application powered by rust-curl , made to simplify the use of apis bot telegram.

![Telly Preview](http://i.imgur.com/43qFdkN.png)

---
## Buy me a coffee

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this 

<a href="https://www.buymeacoffee.com/andrearapoA" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

---

## Features
- Send message a specific recipient from bot telegram.

---

## Setup

If you don't have rust installed on your machine, you can do it with this command:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Clone this repo to your desktop and run `cargo run` and  `cargo build --release` to install all the dependencies and compile a release version.

---

## Usage


Telly needs an .ini configuration file, containing the following information:

```ini
[DEFAULT]
api_key = apiKeyBotTelegram
basic = https://api.telegram.org/botnumber:
recipient = chatId
```



```shell
telly config.ini "hello world"
```


---

## License
>You can check out the full license [here](https://github.com/andrearaponi/Telly/master/LICENSE)

This project is licensed under the terms of the **MIT** license.