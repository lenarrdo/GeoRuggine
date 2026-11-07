# Ratatui

Ratatui applications operate according to the following flow:

1. **Set up a terminal**: this is done automatically by calling the function `ratatui::run`, which passes a preconfigured Terminal into your callback (there are alternative options that allow for more control, but I guess it's completely unnecessary for my use case).
2. **Set up the application's event loop**, which is basically a `while` loop that goes on until the requests to exit the application. The `while` loop should call `Terminal::draw`, to render the current UI state into a `Frame`.
3. **Handle user input** (events).
4. **Draw each frame**.

The following is an extremely minimal example of a working Ratatui application.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| frame.render_widget("Hello World!", frame.area()))?;
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}
```

## Frames

A `Frame` is a consistent view into the terminal state for **rendering a single frame**.

The main event loop in a Ratatui application calls the `.draw(|frame| { /* ... */ })` function. That frame you see there should not be seen as a persistent container that holds widgets over time, but rather as a **single, temporary drawing surface for the current frame tick**.

- On each iteration of the app loop, `terminal.draw(|frame| { /* ... */ })` passes a **fresh `&mut Frame`** into your closure.
- Widgets are rendered using the `frame.render_widget()` function.
- Once the closure finishes, Ratatui computes the difference (diffing) between the previous screen buffer and the new `Frame`, sends only the changed characters to the terminal, and discards the `Frame`.

Because **widgets are ephemeral**, you don't "update" widgets in Ratatui you **update your App State**, and on the next terminal.draw(), you **pass that updated state into new Widget instances**.

### Updating state: example

Let's put into practice what we've learned so far by modifying the previous application to make it render a dynamic string instead of a static one. We are going to use a counter that gets incremented each second as an example.

My first attempt looked something like this:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        let mut counter = 0;
        loop {
            terminal
                .draw(|frame| frame.render_widget(Text::from(counter.to_string()), frame.area()))?;

            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }

            counter += 1;

            thread::sleep(Duration::from_secs(1));
        }
    })
}
```

Unfortunately this doesn't work. The reason is that `crossterm::event::read()` is a **blocking call**, meaning that the event loop only renders the initial state of the counter (i.e., the number `0`) and waits indefinitely for the user to input something. Another issue of this solution is the `thread::sleep()` at the end. If the user presses a key while the thread is sleeping, the input will be executed once the thread wakes up, introducing a small delay.

This new solution fixes the previous problems, by calling the `crossterm::event::poll` function which periodically checks whether there are any available events. In this way, we don't risk blocking the main thread by making it wait for an event that may not even come.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        let mut counter = 0;
        loop {
            terminal
                .draw(|frame| frame.render_widget(Text::from(counter.to_string()), frame.area()))?;

            let event_available = crossterm::event::poll(Duration::from_secs(1))?;
            if event_available {
                if event::read()?.is_key_press() {
                    break Ok(());
                }
            }

            counter += 1;
        }
    })
}
```
