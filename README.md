# Multi Window Output

Multi Window Output allows you to have multiple output screens in the same terminal screen. Each time `Screen` is updated, `Screen` empties its buffer and updates the terminal's screen with the `Screen`'s content.

## Screen

Start a new `Screen` with `Screen::new(screen_name, window_name)`, which will create a new screen with name `screen_name` with a single window with name `window_name`. The name of the `Screen` will be displayed at the top of the terminal. Each window's name will be displayed at the bottom of it. The `id` of the default window is `0`.

```rust
let mut screen = Screen::new("My Screen", "My Window");
```

You can slipt the window into multiple windows. Call `append_left_child(id, name)` or `append_down_child(id, name)` methods to split window with `id` vertically or horizontally. `name` will be the name of the split window.
```rust
let new_window_id = screen.append_left_child(0, "Left Window").unwrap();
let other_new_window_id = screen.append_down_child(0, "Down Window").unwrap();
let last_window_id = screen.append_left_child(new_window_id, "Left Left Window").unwrap();
```

To put content on to the screen you can use `Screen::println(&mut screen, id, line)`, `Screen::print(&mut screen, id, line)`, and `Screen::flush(&mut screen, id)`.
```rust
// This will print a new line with "New Line" in the window with id new_window_id and refresh the screen.
screen.println(new_window, "New Line").unwrap();

// This will print a line with "Line" in the window with id new_window_id; however, it will not start a new line nor refresh the screen.
screen.print(new_window, "Line").unwrap();

// This will flush the current line and refresh the screen.
screen.flush(new_window).unwrap();
```
## Bridge

`Bridge` allows you to call the content functions of `Screen` from different locations. This is specially usefull when printing things from different threads. The only downside to using bridge, is that you can't append new children to the `Screen` you pass to `Bridge::new(&screen)`.

To create a bridge.
```rust
let bridge = Bridge::new(screen);
```

To access bridge from multiple locations, use the `Bridge::clone(&bridge)` function.
```rust
let other_bridge = Bridge::clone(&bridge);
```

To put content in the `Screen` that you passed to create the `Bridge`, use `Bridge::println(...)`, `Bridge::print(...)`, and `Bridge::flush(...)`. These work similarly to its `Screen` counter parts.
```rust
// Notice how you can call the println function from different variables
bridge.println(new_window, "New Line").unwrap();
other_bridge.println(new_window, "New New Line").unwrap();
```

Ideally, when you finish using a screen, run `bridge.kill()` to end the screening process.

https://user-images.githubusercontent.com/57689554/214165855-e4569f2d-499e-471d-8d88-159cab0fe3a0.mp4

# TODO 
- Handle special characters when passing to buffer
- Handle when screen is too small
- Allow the pass of a predefined setup
- Optimizations
