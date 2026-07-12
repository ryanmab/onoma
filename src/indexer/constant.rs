/// The maximum number of files to be assigned to a thread when indexing
/// a directory.
///
/// This primarily helps prevent hitting the upper limit of files which can
/// be open concurrently by an OS.
///
/// But, there is a tradeoff. Too few files per chunk we'll hit the OS's limit
/// for the number of open files concurrently, but, too many files and indexing
/// won't make good use of concurrency and be noticeably slow to index large
/// directories.
pub const MAX_FILES_PER_INDEX_CHUNK: u8 = 200;
