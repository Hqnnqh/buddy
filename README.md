# buddy

**Buddy** is here to keep you company while you work on your computer! It’s an animated, interactive little friend that runs across your screen and responds to your clicks.

![](res/example.gif)

## Prerequisites 
- **Rust**: Make sure you have Rust installed. If not, head over to [Rust’s official website](https://www.rust-lang.org/tools/install) for installation instructions.
- **GTK4**: Buddy uses GTK4 for rendering the character. It can be installed on Linux by running:

```bash
sudo apt install libgtk-4-dev
```

## Installation 
1. Clone the repository to your local machine:

  ```bash
  git clone https://github.com/hannahfluch/buddy.git
  cd buddy
  ```

2. Build the project using cargo
  ```bash
  cargo build --release
  ```

## How to Run 🏃
Simply run the following command to see Buddy in action:

```bash
buddy -h
```
You'll get all the help you need to configure your Buddy.

## Reloading Sprites On-the-Fly
Want to update Buddy's appearance without restarting the program? Buddy can receive signals to reload the sprites:

```bash
kill -SIGUSR1 <pid>
```
> Send SIGUSR1 or SIGUSR2

Replace <pid> with the process ID of the Buddy instance. This will trigger Buddy to reload the sprite animations dynamically

## Configuration ⚙️
Buddy creates a default configuration file upon its first run. This file is located at:

```bash
~/.config/buddy/config.toml
```

### Default Configuration File
The configuration file includes all necessary settings to customize your Buddy's behavior and appearance. However, for Buddy to function, **you must specify a valid sprite path**:
1. Using **command-line arguments**
2. Adding the sprite path to the **configuration file**

## Custom Sprites 🎨

Buddy thrives on customization! Just provide a directory containing different subdirectories for each event type (`idle`, `click`, `run`), and watch your Buddy come to life with your own animations.

## Need to Resize Your Sprites? 🔧

We’ve got you covered! Check out the `res` folder for a handy script to resize your images. Just adjust the scaling factor to match your desired size.
> will be replaced by internal scaling in the future
