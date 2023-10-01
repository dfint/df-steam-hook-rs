use crate::hooks;

pub fn install() {
  let _listener = std::thread::spawn(move || {
    static mut BUTTON_F2: bool = false;
    static mut BUTTON_F3: bool = false;
    static mut BUTTON_CTRL: bool = false;

    static mut HOOK_ENABLED: bool = true;

    unsafe fn check_event() {
      let result = match (BUTTON_F2, BUTTON_F3, BUTTON_CTRL) {
        (true, _, true) => match HOOK_ENABLED {
          true => {
            HOOK_ENABLED = false;
            log::info!("hooks disabled");
            hooks::disable_all()
          }
          false => {
            HOOK_ENABLED = true;
            log::info!("hooks enabled");
            hooks::enable_all()
          }
        },
        (_, _, _) => Ok(()),
      };

      match result {
        Ok(_) => (),
        Err(err) => log::error!("Error in watchdog {}", err),
      };
    }

    if let Err(error) = rdev::listen(move |event| match event.event_type {
      rdev::EventType::KeyPress(rdev::Key::F2) => unsafe {
        BUTTON_F2 = true;
        check_event();
      },
      rdev::EventType::KeyRelease(rdev::Key::F2) => unsafe { BUTTON_F2 = false },
      rdev::EventType::KeyPress(rdev::Key::F3) => unsafe {
        BUTTON_F3 = true;
        check_event();
      },
      rdev::EventType::KeyRelease(rdev::Key::F3) => unsafe { BUTTON_F3 = false },
      rdev::EventType::KeyPress(rdev::Key::ControlLeft) => unsafe {
        BUTTON_CTRL = true;
        check_event();
      },
      rdev::EventType::KeyRelease(rdev::Key::ControlLeft) => unsafe { BUTTON_CTRL = false },
      _ => (),
    }) {
      log::error!("Unable to start watchdog: {:?}", error)
    }
  });
}
