use ramen::event::Event;

pub fn main() {
    let t = [std::thread::spawn(f), std::thread::spawn(f)];
    for x in t {
        x.join().expect("????");
    }
}

pub fn f() {
    let connection = ramen::connection::Connection::new().unwrap();
    let mut window = connection.into_builder().title("simple window, חלון הומו טיפש,彼の死を心から願っています🙏").build().expect("Couldn't build window");
    'program: loop {
        window.poll_events();
        for event in window.events() {
            match event {
                Event::CloseRequest(c) => {
                    println!("Closed with close reason {:?}", c);
                    break 'program;
                },
                Event::Focus(b) => {
                    println!("Window focus state: {}", b);
                },
                _ => (),
            }
        }
    }
}
