use multi_window_output::*;

fn main() {
    let mut screen = Screen::new("Screen_Name", "Window Name");
    let first = screen.append_left_child(0, "Window Name").unwrap();
    let _second = screen.append_down_child(first, "Window Name").unwrap();
    let third = screen.append_down_child(0, "Window Name").unwrap();
    let fourth = screen.append_left_child(third, "Window Name").unwrap();
    let _fifth = screen.append_down_child(fourth, "Window Name").unwrap();

    let sender = Bridge::new(screen);
    sender.println(0, "Sending from bridge").expect("Problems");

    let other_bridge = Bridge::clone(&sender);
    let t1 = std::thread::spawn( move || {
        other_bridge.println(4, "inside this one").expect("Problems");
    });
    //
    //
    let other_bridge = Bridge::clone(&sender);
    let t2 = std::thread::spawn( move || {
        other_bridge.println(2, "Hello World from one").expect("Problems");
        other_bridge.println(3, "Hello World from one").expect("Problems");
        other_bridge.println(4, "Hello World from one").expect("Problems");
    });



    sender.println(0, "tttSending from bridge").expect("Problems");
    sender.println(3, "tttSending from bridge").expect("Problems");
    sender.println(1, "tttSending from bridge").expect("Problems");
    // t1.join().unwrap();
    // t2.join().unwrap();
    //

    sender.println(0, "qqqSending from bridge").expect("Problems");
    t1.join().unwrap();
    t2.join().unwrap();
    std::thread::sleep(std::time::Duration::new(5, 0));
}
