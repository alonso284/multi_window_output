use multi_window_output::*;


fn main(){
    let (tx, rx) = std::sync::mpsc::channel();
    let mut screen = Screen::new(&format!("Name"));
    let first = screen.append_left_child(0).unwrap();
    let second = screen.append_down_child(first).unwrap();
    let third = screen.append_down_child(0).unwrap();
    let fourth = screen.append_left_child(third).unwrap();
    let fifth = screen.append_down_child(fourth).unwrap();
    let vec:Vec<usize> = vec![0,1,2,3,4,5];

    for id in vec {
        let tx_clone = tx.clone();
        std::thread::spawn(move || {
            for _ in 0..50 {
                tx_clone.send(id).unwrap();
                std::thread::sleep(std::time::Duration::new(id as u64, 0));
            }
        });
    }
    drop(tx);

    let mut terman:Vec<usize> = vec![0,0,0,0,0,0];
    for r in rx.iter() {
        screen.println(r, &format!("Message long long long long, very long long like super long from {} number {}", r, terman[r]));
        terman[r] += 1;
    }
}

