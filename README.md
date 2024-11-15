# buddy

**Buddy** is here to keep you company while you work on your computer! It’s an animated, interactive little friend that floats around your screen, responding to your clicks and bringing smiles to your day. 😊

## Features 🌟
- **Interactive Animations**: Your Buddy reacts to clicks and moves around on the screen.
- **Custom Sprites**: Load your own sprites for animations! Buddy adapts to any character you want.
- **Adjustable Settings**: Set your Buddy’s size, animation speed, movement speed, and more!
- **Configurable Events**: Make Buddy surprise you with random events on click!

## Prerequisites 🛠️
To run Buddy, you’ll need the following:

- **Rust**: Make sure you have Rust installed. If not, head over to [Rust’s official website](https://www.rust-lang.org/tools/install) for installation instructions.
- **GTK4**: Buddy uses GTK4 for rendering the character. You can install it on Linux by running:

```bash
sudo apt install libgtk-4-dev
```

## Installation ⚙️
1. Clone the repository to your local machine:

  ```bash
  git clone https://github.com/Hqnnqh/buddy.git
  cd buddy
  ```

2. Build the project using cargo
  ```bash
  cargo build --release
  ```

## How to Run 🏃‍♀️
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

## Custom Sprites 🎨

Buddy thrives on customization! Just provide a directory containing different subdirectories for each event type (`idle`, `click`, `run`), and watch your Buddy come to life with your own animations.

## Need to Resize Your Sprites? 🔧

We’ve got you covered! Check out the `res` folder for a handy script to resize your images. Just adjust the scaling factor to match your desired size.
