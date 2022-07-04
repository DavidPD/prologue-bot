# prologue-bot

This is a custom Discord Bot built for the Prolonging the Prologue Creative Community: a group of storytellers exploring writing, illustration, game development, acting, and more.

I wrote the bot using Rust to learn the language while making something helpful and fairly simple.

## Built With

- [Rust](https://www.rust-lang.org/)
- [Poise](https://lib.rs/crates/poise)

## Features

### Prompt Deck

The Bot's customizable deck allows writers to "draw" random character and worldbuilding prompts out of a predetermined list. You can also mix and match decks using prompt groups. These groups are stored in separate files where each prompt is separating by horizontal rules (`---`).

#### Prompt Deck Commands

- `/draw`: Randomly selects a card from the current deck.
- `/list_prompt_decks`: Prints a list of all available prompt decks.
- `/add_prompt_deck`: Adds all prompts from a given prompt deck.
- `/start_prompt_session <session-name>`: Starts a prompt session with an optional name. Gives an error if there is already a named session in progress.
- `/end_prompt_session`: Resets the current deck and ends a prompting session.

#### Tentative Roadmap

- Multiple sessions
- Sessions saved to a database for later retrieval and persistence when restarting
- Message tracking so the bot can send you a list of your own replies after a prompt session
- A database of cards that can be added to dynamically

## Disclaimer

I'm building this mostly for fun in my spare time so maintenance may be sparse and new features slow to develop.

## Contributing

I'm not looking for code contributions at the moment.

If you would like to contribute prompts feel free to fork the repository and make a PR, or contact one of the Prolonging the Prologue mods.
