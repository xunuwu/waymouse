use std::{fmt::Display, thread::sleep, time::Duration};

use clap::{Parser, Subcommand, ValueEnum};
use wayland_client::{
    Connection, Dispatch, EventQueue,
    protocol::{
        wl_pointer::{
            Axis,
            AxisSource::{self},
            ButtonState,
        },
        wl_registry,
    },
};
use wayland_protocols_wlr::virtual_pointer::v1::client::{
    zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1,
    zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1,
};

#[derive(Debug, Clone, Default)]
struct AppState {
    pub virtual_pointer_manager: Option<ZwlrVirtualPointerManagerV1>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppState {
    fn event(
        state: &mut Self,
        proxy: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            if interface.as_str() == "zwlr_virtual_pointer_manager_v1" {
                state.virtual_pointer_manager = Some(
                    proxy.bind::<ZwlrVirtualPointerManagerV1, _, _>(name, version, qhandle, ()),
                );
            }
        }
    }
}

impl Dispatch<ZwlrVirtualPointerManagerV1, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &ZwlrVirtualPointerManagerV1,
        _: <ZwlrVirtualPointerManagerV1 as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        // pointer manager has no events
    }
}

impl Dispatch<ZwlrVirtualPointerV1, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &ZwlrVirtualPointerV1,
        _: <ZwlrVirtualPointerV1 as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        // pointer has no events
    }
}

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Move {
        x: f64,
        y: f64,
    },
    Scroll {
        #[arg(long = "horizontal", short = 'z')]
        horizontal: bool,

        amount: f64,
    },
    Button {
        #[command(subcommand)]
        action: ButtonActions,
    },
}

#[derive(Subcommand)]
enum ButtonActions {
    Click {
        button: MouseButton,

        #[arg(default_value_t = 1, long = "count", short = 'c')]
        count: u32,

        #[arg(default_value_t = 100, long = "delay", short = 'd')]
        delay_ms: u64,
    },
    Down {
        button: MouseButton,
    },
    Up {
        button: MouseButton,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum MouseButton {
    Left,
    Right,
    Middle,
}

impl From<MouseButton> for u32 {
    fn from(val: MouseButton) -> Self {
        match val {
            MouseButton::Left => 272,
            MouseButton::Right => 273,
            MouseButton::Middle => 274,
        }
    }
}

#[derive(Debug)]
enum WaymouseError {
    NoPointerManager,
}

impl Display for WaymouseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WaymouseError::NoPointerManager => write!(
                f,
                "zwlr_virtual_pointer_manager_v1 not found, does your compositor support it?"
            ),
        }
    }
}

impl std::error::Error for WaymouseError {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let connection = Connection::connect_to_env()?;
    let mut queue = {
        let display = connection.display();

        let queue: EventQueue<AppState> = connection.new_event_queue();
        let queue_handle = queue.handle();

        display.get_registry(&queue_handle, ());

        queue
    };

    let mut state = AppState::default();

    queue.roundtrip(&mut state)?;

    let Some(ref pointer_manager) = state.virtual_pointer_manager else {
        return Err(WaymouseError::NoPointerManager)?;
    };

    let pointer = pointer_manager.create_virtual_pointer(None, &queue.handle(), ());

    match cli.command {
        Commands::Move { x, y } => {
            pointer.motion(0, x, y);
        }
        Commands::Scroll { amount, horizontal } => {
            pointer.axis_source(AxisSource::Wheel);
            let axis = match horizontal {
                true => Axis::HorizontalScroll,
                false => Axis::VerticalScroll,
            };
            pointer.axis_stop(0, axis);
            pointer.axis(0, axis, amount);
        }
        Commands::Button { action } => match action {
            ButtonActions::Click {
                button,
                count,
                delay_ms,
            } => {
                for n in 0..count {
                    pointer.button(0, button.into(), ButtonState::Pressed);
                    pointer.button(0, button.into(), ButtonState::Released);

                    // only if not on last iteration
                    if n != count - 1 {
                        pointer.frame();
                        queue.roundtrip(&mut state)?;
                        sleep(Duration::from_millis(delay_ms));
                    }
                }
            }
            ButtonActions::Down { button } => {
                pointer.button(0, button.into(), ButtonState::Pressed);
            }
            ButtonActions::Up { button } => {
                pointer.button(0, button.into(), ButtonState::Released);
            }
        },
    }

    pointer.frame();
    queue.roundtrip(&mut state)?;

    Ok(())
}
