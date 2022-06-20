# prologue-bot

This is a custom Discord Bot specifically built for the Prolonging the Prologue community, written in Rust because I wanted a project to help me learn the language.

Prolonging the Prologue is a community of creatives on Discord, and I'm building this as an attempt to make something helpful but keep it fairly simple.

The bot will be built as multiple modules as features are added and each module will be separated from any other features in the bot.

## Prompt Deck

A customizable prompt deck that allows writers to "draw" random character and worldbuilding prompts out of a predetermined list.
You'll also be able to mix and match decks based on groups of prompts defined in markdown files separated by horizontal rules `---`.

### Prompt Deck Commands

- `/draw`: Randomly selects a card from the current deck.
- `/list-decks`: Prints a list of all available prompt decks.
- `/add-deck`: Adds all prompts from a given prompt deck.
- `/end-prompt-session`: Resets the current deck and ends a prompting session.
- `/start-prompt-session <session-name>`: Starts a prompt session with an optional name, gives an error if there is already a named session in progress.
- `/export-prompt-answers`: Exports all the answers you personally gave (by replying to the bot) in the current or previous session.

### Potential future prompt features

- Multiple sessions
- Sessions saved to a database for later retrieval and persistence when restarting
- Message parsing so you don't have to reply for your answers to be saved.

## Disclaimer

I'm building this mostly for fun in my spare time so maintenance may be sparse and new features slow to develop. I'm also very new to Rust and while I'm trying to do things cleanly I've got a lot to learn. 

## Contributing

If you're interested in helping out contact me in PTP.
