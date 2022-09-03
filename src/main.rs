use ramen::event::Event;

pub fn main() {
    let t = [std::thread::spawn(f)];
    for x in t {
        if x.join().is_err() {
            eprintln!("Exiting main thread because join() failed");
            return;
        }
    }
}

pub fn f() {
    let connection = match ramen::connection::Connection::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error setting up connection: {:?}", e);
            return;
        },
    };
    let mut window = match connection.into_builder().title("simple window, חלון הומו טיפש,彼の死を心から願っています🙏").build() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error building window: {:?}", e);
            return;
        },
    };
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
