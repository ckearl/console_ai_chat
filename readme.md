# Console AI Chat

## Description

This is a simple CLI interface that allows you to chat with either chat-GPT or Claude.ai. The reasoning behind this project was that I needed an excuse to learn Rust and I was tired of using web interfaces for LLMs for the specific use case of one off questions. This project is not meant to be a replacement for the web interfaces, but rather a quick way to get a response from an LLM without having to open a browser.

## Setup

To run this program you will need to have Rust installed on your machine. Find the installation instructions [here](https://www.rust-lang.org/tools/install).

Additionally, you will need to have a .env file in the root directory of the project. You should define the following global environment variables in the .env file: `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`. You will need to sign up for an account with both [OpenAI](https://chatgpt.com/) and [Anthropic](https://claude.ai/), navigate to each respective API documentation pages, and register for individual keys. Put the keys in the .env file in the following format:

```bash
ANTHROPIC_API_KEY="your_key_here"
OPENAI_API_KEY="your_key_here"
```

Note that you aren't required to use both platforms, but you will need to have at least one of them set up to use the program.

## Usage

To run the program you just need to run the .toml file through cargo.

```bash
cargo run -- -platform -prompting-styling "What ever you want to ask"
```

## Breakdown

The command has two flags that you can use:
    1. The first flag (noted with `-platform` above) determines what AI you want to use. With the options being either chat-GPT or Claude.ai, you can call chat-GPT with the `-gpt` or Claude with `-cl`.
    2. The second flag (noted with `-prompting-styling` above) is a prompt modying flag. This flag is used to alter the style of response that the AI will return. The options are:
        - `s`: Meaning "short," this flag will limit the response to one paragraph or less.
        - `c`: Meaning "command," this flag will return an ordered list of steps to complete a specific task. This is especially useful for technical questions, such as "How do init a git repository?"
        - _ (No flag): If you don't include a flag, the AI will return a response in the default style.

Make sure to include the flags in the order shown above and your prompt body should be in quotes.
