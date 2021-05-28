fn main() {
    windows::build!(
        Windows::Web::Syndication::*,
        Windows::Foundation::*,
        Windows::Foundation::Collections::*,
        Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
    );
}