use std::ffi::c_void;

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
    JoyButton(JoyButtonEvent),
    JoyAxisMotion(JoyAxisMotionEvent),
    JoyBallMotion(JoyBallMotionEvent),
    JoyHatMotion(JoyHatMotionEvent),
    AudioDeviceAdded(AudioDeviceEvent),
    AudioDeviceRemoved(AudioDeviceEvent),
    RenderDeviceReset(TimeStamp),
    RenderTargetsReset(TimeStamp),
    User(UserEvent),
    Unknown(UnknownEvent),
}

impl From<SdlEvent> for Event {
    // Partial implementation for the important events
    // TODO: Implement the rest aswell
    fn from(event: SdlEvent) -> Self {
        match event {
            SdlEvent::Quit { timestamp } => Self::Quit(timestamp),
            SdlEvent::AppTerminating { timestamp } => Self::AppTerminating(timestamp),
            SdlEvent::AppLowMemory { timestamp } => Self::AppLowMemory(timestamp),
            SdlEvent::AppWillEnterBackground { timestamp } => {
                Self::AppWillEnterBackground(timestamp)
            }
            SdlEvent::AppDidEnterBackground { timestamp } => Self::AppDidEnterBackground(timestamp),
            SdlEvent::AppWillEnterForeground { timestamp } => {
                Self::AppWillEnterForeground(timestamp)
            }
            SdlEvent::AppDidEnterForeground { timestamp } => Self::AppDidEnterBackground(timestamp),
            SdlEvent::Display {
                timestamp,
                display_index,
                display_event,
            } => Self::Display(DisplayEvent {
                timestamp,
                display_index,
                display_event,
            }),
            SdlEvent::Window {
                timestamp,
                window_id,
                win_event,
            } => Self::Window(WindowEvent {
                timestamp,
                window_id,
                win_event,
            }),
            SdlEvent::KeyDown {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
            } => Self::Key(KeyEvent {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
                direction: PressDirection::Down,
            }),
            SdlEvent::KeyUp {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
            } => Self::Key(KeyEvent {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
                direction: PressDirection::Up,
            }),
            SdlEvent::TextEditing {
                timestamp,
                window_id,
                text,
                start,
                length,
            } => Self::Text(TextEvent {
                timestamp,
                window_id,
                text,
                offset: Some(SizedOffset { start, length }),
            }),
            SdlEvent::TextInput {
                timestamp,
                window_id,
                text,
            } => Self::Text(TextEvent {
                timestamp,
                window_id,
                text,
                offset: None,
            }),
            SdlEvent::MouseMotion {
                timestamp,
                window_id,
                which,
                mousestate,
                x,
                y,
                xrel,
                yrel,
            } => Self::MouseMotion(MouseMotionEvent {
                timestamp,
                window_id,
                which,
                mousestate,
                x,
                y,
                xrel,
                yrel,
            }),
            SdlEvent::MouseButtonDown {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => Self::MouseClick(MouseClickEvent {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
                direction: PressDirection::Down,
            }),
            SdlEvent::MouseButtonUp {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => Self::MouseClick(MouseClickEvent {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
                direction: PressDirection::Up,
            }),
            SdlEvent::MouseWheel {
                timestamp,
                window_id,
                which,
                x,
                y,
                direction,
            } => Self::MouseWheel(MouseWheelEvent {
                timestamp,
                window_id,
                which,
                x,
                y,
                direction,
            }),
            SdlEvent::JoyAxisMotion {
                timestamp,
                which,
                axis_idx,
                value,
            } => Self::JoyAxisMotion(JoyAxisMotionEvent {
                timestamp,
                which,
                axis_idx,
                value,
            }),
            SdlEvent::JoyBallMotion {
                timestamp,
                which,
                ball_idx,
                xrel,
                yrel,
            } => Self::JoyBallMotion(JoyBallMotionEvent {
                timestamp,
                which,
                ball_idx,
                xrel,
                yrel,
            }),
            SdlEvent::JoyHatMotion {
                timestamp,
                which,
                hat_idx,
                state,
            } => Self::JoyHatMotion(JoyHatMotionEvent {
                timestamp,
                which,
                hat_idx,
                state,
            }),
            SdlEvent::JoyButtonDown {
                timestamp,
                which,
                button_idx,
            } => Self::JoyButton(JoyButtonEvent {
                timestamp,
                which,
                button_idx,
                direction: PressDirection::Down,
            }),
            SdlEvent::JoyButtonUp {
                timestamp,
                which,
                button_idx,
            } => Self::JoyButton(JoyButtonEvent {
                timestamp,
                which,
                button_idx,
                direction: PressDirection::Up,
            }),
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
            SdlEvent::DropFile {
                timestamp,
                window_id,
                filename,
            } => Self::DropFile(DropFileEvent {
                timestamp,
                window_id,
                filename,
            }),
            SdlEvent::DropText {
                timestamp,
                window_id,
                filename,
            } => Self::DropText(DropFileEvent {
                timestamp,
                window_id,
                filename,
            }),
            SdlEvent::DropBegin {
                timestamp,
                window_id,
            } => Self::Drop(DropEvent {
                timestamp,
                window_id,
                stage: DropStage::Begin,
            }),
            SdlEvent::DropComplete {
                timestamp,
                window_id,
            } => Self::Drop(DropEvent {
                timestamp,
                window_id,
                stage: DropStage::Complete,
            }),
            SdlEvent::AudioDeviceAdded {
                timestamp,
                which,
                iscapture,
            } => Self::AudioDeviceAdded(AudioDeviceEvent {
                timestamp,
                which,
                iscapture,
            }),
            SdlEvent::AudioDeviceRemoved {
                timestamp,
                which,
                iscapture,
            } => Self::AudioDeviceRemoved(AudioDeviceEvent {
                timestamp,
                which,
                iscapture,
            }),
            SdlEvent::RenderTargetsReset { timestamp } => Self::RenderTargetsReset(timestamp),
            SdlEvent::RenderDeviceReset { timestamp } => Self::RenderDeviceReset(timestamp),
            SdlEvent::User {
                timestamp,
                window_id,
                type_,
                code,
                data1,
                data2,
            } => Self::User(UserEvent {
                timestamp,
                window_id,
                type_,
                code,
                data1,
                data2,
            }),
            SdlEvent::Unknown { timestamp, type_ } => Self::Unknown(UnknownEvent {
                timestamp: Some(timestamp),
                type_: Some(type_),
            }),
            _ => Self::Unknown(UnknownEvent {
                timestamp: None,
                type_: None,
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayEvent {
    pub timestamp: TimeStamp,
    pub display_index: i32,
    pub display_event: sdl2::event::DisplayEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub win_event: SdlWindowEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub keycode: Option<sdl2::keyboard::Keycode>,
    pub scancode: Option<sdl2::keyboard::Scancode>,
    pub keymod: sdl2::keyboard::Mod,
    pub repeat: bool,
    pub direction: PressDirection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub text: String,
    pub offset: Option<SizedOffset>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SizedOffset {
    pub start: i32,
    pub length: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseWheelEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub which: u32,
    pub x: i32,
    pub y: i32,
    pub direction: sdl2::mouse::MouseWheelDirection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropFileEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub filename: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropStage {
    Begin,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub stage: DropStage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JoyButtonEvent {
    pub timestamp: TimeStamp,
    pub which: u32,
    pub button_idx: u8,
    pub direction: PressDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JoyAxisMotionEvent {
    pub timestamp: TimeStamp,
    pub which: u32,
    pub axis_idx: u8,
    pub value: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JoyBallMotionEvent {
    pub timestamp: TimeStamp,
    pub which: u32,
    pub ball_idx: u8,
    pub xrel: i16,
    pub yrel: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JoyHatMotionEvent {
    pub timestamp: TimeStamp,
    pub which: u32,
    pub hat_idx: u8,
    pub state: sdl2::joystick::HatState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioDeviceEvent {
    pub timestamp: TimeStamp,
    pub which: u32,
    pub iscapture: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserEvent {
    pub timestamp: TimeStamp,
    pub window_id: u32,
    pub type_: u32,
    pub code: i32,
    pub data1: *mut c_void,
    pub data2: *mut c_void,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownEvent {
    pub timestamp: Option<TimeStamp>,
    pub type_: Option<u32>,
}
