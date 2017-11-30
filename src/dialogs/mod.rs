//! Module containing all dialogs

/// The "main" dialog, i.e. the application window
pub mod main;
/// Single menu item
pub mod sidebar_menu_item;
/// Ribbon bar
pub mod ribbon_bar;
/// Ribbon bar items
pub mod ribbon_bar_items;
/// Search bar
pub mod search_bar;
/// Navigation bar with clickable items +  dropdown
pub mod navigation_bar;
/// Navigation buttons: Previous (history), Next (history), 
pub mod navigation_buttons;
/// Status bar (number of items, image or tree view)
pub mod status_bar;
/// Collapsible category
pub mod collapsible_category;
/// Help dialog
pub mod help_dialog;
/// Context menu
pub mod context_menu;

// --- Composite widgets, made up of the widgets above

/// Favorite folder
pub mod favourites;
/// Libraries (collections of folders that belong together)
pub mod libraries;
/// Remote server dialog
pub mod remote_servers;
/// Trash page
pub mod trash_dialog;
/// Search page
pub mod search_results;
/// System page
pub mod system_info;
