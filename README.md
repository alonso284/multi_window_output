# Multi Window Output


# TODO 
- Handle special characters when passing to buffer;
- Make screen buffer an owned variable
- Handle when screen is too small
- Documentation
- Optimizations

What do I want it to do?
- The user should be able to use it as a normal println!() macros, by doing something like `windows.println(format!("Some text")`
- Allwo the user to have multiple screens and change them from the terminal executing of from the program automatically with something like `screen1.display()` or typing `switch screen1`
- Allow the user to go back to defult screen, the one that is one by default.
- Allow the user to make a screen or window put its output on a file
- Allow user to merge screens and separate them


- Each screen should have a name
- Each screen has a focus window (possibly none) (A pointer to the window) that highlight the window
- Each screen has a Window or Split screen

- Each window has a parant Screen, which is the outmost screen
- Each window will have a `buffer[Option<str>;1024]` which will funtion as a queue (First in First Out) which will hold the strings that it outputs
- Function window.print(&str) will append the contents of a the &str to the last str
- Function window.flush() will refresh the parents output
- Function window.println(&str) will take in a str splice and copy it to the buffer, begin a new line, and flush

- The buffer type will have a `queue: [Option<str>;1024]`, `ptr_start: usize` which will indicate where the first is and `ptr_end: usizez whici=h will indicate where the last like is
- Funcion `buffer.push(str)` will put an str on the `queue` and will overwrite the start if the queue is full

src
	lib.rs
	Screen
		Screen.rs
		Window
			Window.rs
			Buffer
				Buffer.rs
