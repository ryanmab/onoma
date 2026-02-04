use notify_debouncer_mini::DebouncedEvent;

use crate::watcher;

#[allow(missing_docs)]
#[doc(hidden)]
pub type Result<T> = std::result::Result<T, watcher::Error>;

#[allow(missing_docs)]
pub type Event = std::result::Result<Vec<DebouncedEvent>, notify::Error>;
