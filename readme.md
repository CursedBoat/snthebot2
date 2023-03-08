# SNTHEBOT2

![](https://cdn.discordapp.com/emojis/1002963217083011134.webp?size=96&quality=lossless)

Literally just my old Discord bot, except re-written in Rust.



------------

### Features
- Rust's guessing game, except now in Discord 👍
- A "say" command that deletes your message after the command is invoked
- A command where you can kill your fellow server members
- Search for a user's top play in Osu!std because no-one plays any other mode
- Reddit command which returns a random post from the specified subreddit
- Other dumb commands (see [src/commands](https://github.com/CursedBoat/snthebot2/tree/master/src/commands "src/commands") for more)

----------
## Building from source
**NOTE: I HAVE NOT TESTED THE PROJECT IN LINUX**

Make sure you have insalled Rust (I prefer [rustup](https://rustup.rs/ "rustup"))
1. Clone the repo:
``git clone https://github.com/CursedBoat/snthebot2.git ``

2. Create a `.env` containing your Discord token, Osu client secret and prefix. The file must be in the root directory (e.g. the folder containing src)
**Example:**
```.env
DISCORD_TOKEN=example
PREFIX=~
OSU_CLIENT_SECRET=example
```

3. Build the project:
``cargo build --release``

4. Run the executable from ``./target/release``
Note: the application will panic if a ``.env`` file containing the required information is not found.


## Q&A
Q: Why?  
A: I do not know.

Q: What if I don't want to build from source?  
A: A pre-built binary for Windows is provided in the [releases](https://github.com/CursedBoat/snthebot2/releases "releases")

Q: Binaries for Linux?  
A: What are you a Windows user? Build from source smh
