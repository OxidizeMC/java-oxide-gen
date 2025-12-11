#[macro_export]
macro_rules! io_data_error {
    ($($arg:tt)*) => {{
        let message: String = format!($($arg)*);
        std::io::Error::new(std::io::ErrorKind::InvalidData, message)
    }};
}

#[macro_export]
macro_rules! io_data_err {
    ($($arg:tt)*) => { Err($crate::io_data_error!($($arg)*)) };
}

#[macro_export]
macro_rules! pretty_path {
    ($path:expr) => {{
        if cfg!(windows) && $path.is_absolute() {
            let p_str: String = $path.to_string_lossy().to_string();
            if let Some(p) = p_str.strip_prefix("\\\\?\\") {
                p.to_string()
            } else if let Some(p) = p_str.strip_prefix("\\\\.\\") {
                p.to_string()
            } else {
                $path.display().to_string()
            }
        } else {
            $path.display().to_string()
        }
    }};
}
