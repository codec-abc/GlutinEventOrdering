extern crate glutin;

fn main() {
    let window = glutin::WindowBuilder::new().build().unwrap();
    let _ = unsafe { window.make_current() };

    for event in window.wait_events() {
        let _ = window.swap_buffers();

        match event {
            glutin::Event::Closed => break,
            glutin::Event::KeyboardInput(element_state, scan_code, maybe_virtual_key_code) => {

                match element_state {
                    glutin::ElementState::Pressed => println!("keydown event"),
                    glutin::ElementState::Released => println!("keyup event")
                }

                let virtual_key_code_as_str = match maybe_virtual_key_code {
                    None => "None".to_owned(),
                    Some(a) => format!("{:?}", a)
                };

                println!("key scancode is {}, key virtual key code is {}", scan_code, virtual_key_code_as_str);
            } ,
            glutin::Event::ReceivedCharacter(c) => {
                println!("text input event");
                println!("character is {}", c);
            } ,
            _ => ()
        }
    }
}