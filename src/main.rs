use multi_window_output::*;

fn main(){
    let mut screen = Screen::new();
    let id_1 = screen.append_left_child(0).unwrap();
    screen.println(id_1, &format!("Hello World")).unwrap();


}
