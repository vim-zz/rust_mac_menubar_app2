
use objc2::msg_send;
use objc2::runtime::NSObject;
use objc2::{sel, ClassType};
use objc2_foundation::NSString;
use objc2_app_kit::{NSApplication, NSMenu, NSMenuItem, NSStatusBar};

fn main() {
    unsafe {
        // Get the shared application instance
        let app: *const NSApplication = msg_send![NSApplication::class(), sharedApplication];

        // Use NSApplicationActivationPolicyAccessory (1) to hide the dock icon
        let _: bool = msg_send![app, setActivationPolicy: 1_i64];

        // Create status bar item
        let status_bar: *const NSStatusBar = msg_send![NSStatusBar::class(), systemStatusBar];
        let status_item: *const NSObject = msg_send![status_bar, statusItemWithLength: -1.0f64];

        // Get the button and set its title
        let button: *const NSObject = msg_send![status_item, button];
        let title = NSString::from_str("☕");
        let _: () = msg_send![button, setTitle: title.as_ref() as *const NSString];

        // Create menu
        let menu: *const NSMenu = msg_send![NSMenu::class(), alloc];
        let menu: *const NSMenu = msg_send![menu, init];

        // Create Quit menu item
        let quit_item: *const NSMenuItem = msg_send![NSMenuItem::class(), alloc];
        let quit_item: *const NSMenuItem = msg_send![quit_item, init];

        let quit_title = NSString::from_str("Quit");
        let _: () = msg_send![quit_item, setTitle: quit_title.as_ref() as *const NSString];
        let _: () = msg_send![quit_item, setAction: sel!(terminate:)];
        let _: () = msg_send![quit_item, setTarget: app];

        let _: () = msg_send![menu, addItem: quit_item];
        let _: () = msg_send![status_item, setMenu: menu];
        let _: () = msg_send![app, run];
    }
}
