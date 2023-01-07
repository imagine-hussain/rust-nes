use sdl2::event::Event as SdlEvent;
use sdl2::event::WindowEvent as SdlWindowEvent;

type TimeStamp = u32;
pub enum Event {
    Quit(TimeStamp),
    AppTerminating(TimeStamp),
    AppLowMemory(TimeStamp),
    AppDidEnterBackground(TimeStamp),
    AppWillEnterBackground(TimeStamp),
    AppWillEnterForeground(TimeStamp),
    AppDidEnterForeground(TimeStamp),
    Display(DisplayEvent),
    Window(WindowEvent),
    Key(KeyEvent),
    Text(TextEvent),
    MouseMotion(MouseMotionEvent),
    MouseClick(MouseClickEvent),
    MouseWheel(MouseWheelEvent),
    ClipboardUpdate(TimeStamp),
    DropFile(DropFileEvent),
    DropText(DropFileEvent),
    Drop(DropEvent),
    Unknown(Option<TimeStamp>),
}

impl From<SdlEvent> for Event {
    fn from(event: SdlEvent) -> Self {
        match event {
            SdlEvent::Quit { timestamp } => Self::Quit(timestamp),
            SdlEvent::AppTerminating { timestamp } => Self::AppTerminating(timestamp),
            SdlEvent::AppLowMemory { timestamp } => Self::AppLowMemory(timestamp),
            SdlEvent::AppWillEnterBackground { timestamp } => Self::AppWillEnterBackground(timestamp),
            SdlEvent::AppDidEnterBackground { timestamp } => Self::AppDidEnterBackground(timestamp),
            SdlEvent::AppWillEnterForeground { timestamp } => Self::AppWillEnterForeground(timestamp),
            SdlEvent::AppDidEnterForeground { timestamp } => Self::AppDidEnterBackground(timestamp),
            SdlEvent::Display { timestamp, display_index, display_event } => Self::Display(DisplayEvent { timestamp, display_index, display_event }),
            SdlEvent::Window { timestamp, window_id, win_event } => Self::Window(WindowEvent { timestamp, window_id, win_event}),
            SdlEvent::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat } => Self::Key(KeyEvent { timestamp, window_id, keycode, scancode, keymod, repeat, direction: PressDirection::Down}),
            SdlEvent::KeyUp { timestamp, window_id, keycode, scancode, keymod, repeat } => Self::Key(KeyEvent { timestamp, window_id, keycode, scancode, keymod, repeat, direction: PressDirection::Up}),
            SdlEvent::TextEditing { timestamp, window_id, text, start, length } => Self::Text(TextEvent { timestamp, window_id, text, offset: Some(SizedOffset {start, length}) }),
            SdlEvent::TextInput { timestamp, window_id, text } => Self::Text(TextEvent { timestamp, window_id, text, offset: None }),
            SdlEvent::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => Self::MouseMotion(MouseMotionEvent { timestamp, window_id, which, mousestate, x, y, xrel, yrel }),
            SdlEvent::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => Self::MouseClick(MouseClickEvent { timestamp, window_id, which, mouse_btn, clicks, x, y, direction: PressDirection::Down }),
            SdlEvent::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => Self::MouseClick(MouseClickEvent { timestamp, window_id, which, mouse_btn, clicks, x, y, direction: PressDirection::Up }),
            SdlEvent::MouseWheel { timestamp, window_id, which, x, y, direction } => Self::MouseWheel(MouseWheelEvent { timestamp, window_id, which, x, y, direction }),
            // SdlEvent::JoyAxisMotion { timestamp, which, axis_idx, value } => todo!(),
            // SdlEvent::JoyBallMotion { timestamp, which, ball_idx, xrel, yrel } => todo!(),
            // SdlEvent::JoyHatMotion { timestamp, which, hat_idx, state } => todo!(),
            // SdlEvent::JoyButtonDown { timestamp, which, button_idx } => todo!(),
            // SdlEvent::JoyButtonUp { timestamp, which, button_idx } => todo!(),
            // SdlEvent::JoyDeviceAdded { timestamp, which } => todo!(),
            // SdlEvent::JoyDeviceRemoved { timestamp, which } => todo!(),
            // SdlEvent::ControllerAxisMotion { timestamp, which, axis, value } => todo!(),
            // SdlEvent::ControllerButtonDown { timestamp, which, button } => todo!(),
            // SdlEvent::ControllerButtonUp { timestamp, which, button } => todo!(),
            // SdlEvent::ControllerDeviceAdded { timestamp, which } => todo!(),
            // SdlEvent::ControllerDeviceRemoved { timestamp, which } => todo!(),
            // SdlEvent::ControllerDeviceRemapped { timestamp, which } => todo!(),
            // SdlEvent::FingerDown { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => todo!(),
            // SdlEvent::FingerUp { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => todo!(),
            // SdlEvent::FingerMotion { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => todo!(),
            // SdlEvent::DollarGesture { timestamp, touch_id, gesture_id, num_fingers, error, x, y } => todo!(),
            // SdlEvent::DollarRecord { timestamp, touch_id, gesture_id, num_fingers, error, x, y } => todo!(),
            // SdlEvent::MultiGesture { timestamp, touch_id, d_theta, d_dist, x, y, num_fingers } => todo!(),
            SdlEvent::ClipboardUpdate { timestamp } => Self::ClipboardUpdate(timestamp),
            SdlEvent::DropFile { timestamp, window_id, filename } => Self::DropFile(DropFileEvent { timestamp, window_id, filename }),
            SdlEvent::DropText { timestamp, window_id, filename } => Self::DropText(DropFileEvent { timestamp, window_id, filename }),
            SdlEvent::DropBegin { timestamp, window_id } => Self::Drop(DropEvent { timestamp, window_id, stage: DropStage::Begin }),
            SdlEvent::DropComplete { timestamp, window_id } => Self::Drop(DropEvent { timestamp, window_id, stage: DropStage::Complete }),
            // SdlEvent::AudioDeviceAdded { timestamp, which, iscapture } => todo!(),
            // SdlEvent::AudioDeviceRemoved { timestamp, which, iscapture } => todo!(),
            // SdlEvent::RenderTargetsReset { timestamp } => todo!(),
            // SdlEvent::RenderDeviceReset { timestamp } => todo!(),
            // SdlEvent::User { timestamp, window_id, type_, code, data1, data2 } => todo!(),
            // SdlEvent::Unknown { timestamp, type_ } => todo!(),
            _ => Self::Unknown(None),
        }
    }
}

pub struct DisplayEvent {
    pub timestamp: TimeStamp,
    pub display_index: i32,
    pub display_event: sdl2::event::DisplayEvent
}

pub struct WindowEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub win_event: SdlWindowEvent,
}

pub enum PressDirection {
    Up,
    Down,
}

pub struct KeyEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub keycode: Option<sdl2::keyboard::Keycode>,
    pub scancode: Option<sdl2::keyboard::Scancode>,
    pub keymod: sdl2::keyboard::Mod,
    pub repeat: bool,
    pub direction: PressDirection,
}

pub struct TextEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub text: String,
    pub offset: Option<SizedOffset>,
}

pub struct SizedOffset  {
    pub start: i32,
    pub length: i32,
}

pub struct MouseMotionEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub which: u32,
    pub mousestate: sdl2::mouse::MouseState,
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
}

pub struct MouseClickEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub which: u32,
    pub mouse_btn: sdl2::mouse::MouseButton,
    pub clicks: u8,
    pub x: i32,
    pub y: i32,
    pub direction: PressDirection,
}

pub struct MouseWheelEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub which: u32,
    pub x: i32,
    pub y: i32,
    pub direction: sdl2::mouse::MouseWheelDirection,
}

pub struct DropFileEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub filename: String,
}

pub enum DropStage {
    Begin,
    Complete
}

pub struct DropEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub stage: DropStage,
}


