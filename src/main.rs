use ramen::{connection::Connection, event::Event}; // There's no actual error here it's an RA bug sorry in advance

pub fn main() {
    let c = match Connection::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error setting up connection: {:?}", e);
            return;
        },
    };

    f(c);

    // let cs = [c.clone(), c.clone(), c.clone(), c.clone(), c.clone()];
    // let t = cs.map(|c| std::thread::spawn(move || f(c)));
    // for x in t {
    //     if x.join().is_err() {
    //         eprintln!("Exiting main thread because join() failed");
    //         return;
    //     }
    // }
}

pub fn f(connection: Connection) {
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
                Event::Resize(t) => {
                    println!("Window resize: {:?}", t);
                },
                #[cfg(feature = "input")]
                Event::KeyboardDown(k) => {
                    println!("Key down: {:?}", k);
                },
                #[cfg(feature = "input")]
                Event::KeyboardRepeat(k) => {
                    println!("Key repeat: {:?}", k);
                },
                #[cfg(feature = "input")]
                Event::KeyboardUp(k) => {
                    println!("Key up: {:?}", k);
                },
                #[cfg(feature = "input")]
                Event::MouseDown(k) => {
                    println!("Mouse down: {:?}", k);
                },
                #[cfg(feature = "input")]
                Event::MouseUp(k) => {
                    println!("Mouse up: {:?}", k);
                },
                #[cfg(feature = "input")]
                Event::MouseMove(t) => {
                    println!("Mouse move: {:?}", t);
                },
                #[cfg(feature = "input")]
                Event::Input(code) => {
                    println!("Input: {}", code);
                },
                #[cfg(feature = "input")]
                Event::MouseEnter => println!("!! Mouse enter"),
                #[cfg(feature = "input")]
                Event::MouseLeave => println!("!! Mouse leave"),
                _ => (),
            }
        }
    }
}
