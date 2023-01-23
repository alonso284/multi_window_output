use multi_window_output::*;


fn main(){
    let (tx, rx) = std::sync::mpsc::channel();
    let mut screen = Screen::new(&format!("Name"));
    
    let tx_clone = tx.clone();
    let id = screen.append_down_child(0).unwrap();
    std::thread::spawn( move || {
        for _ in 0..5 {
            tx_clone.send(id).unwrap();
            std::thread::sleep(std::time::Duration::new(2, 0));
        }
    });


    let tx_clone = tx.clone();
    let id = screen.append_left_child(0).unwrap();
    std::thread::spawn( move || {
        for _ in 0..5 {
            tx_clone.send(id).unwrap();
            std::thread::sleep(std::time::Duration::new(2, 0));
        }
    });
    drop(tx);

    for r in rx.iter() {
        screen.println(r, &format!("Received message from {}", r));
    }



}

