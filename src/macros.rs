// TODO: add more examples
/// The replacement macro that uses core logger functions.
///
/// # Usage
///
/// ```
/// // -snip-
///
/// // In your function :
/// use label_logger::OutputLabel;
/// // -snip-
/// println!(OutputLabel::Info("Compiling"), "the program")
/// println!(_, "information without label")
/// println!(_, "more informations without label")
/// // -snip-
/// println!(OutputLabel::Success("Finished"), "the compilation")
/// ```
///
/// For more see the [examples folder](https://github.com/MrNossiom/label-logger/tree/main/examples)
/// **Note**: this macro replace the builtin println macro
#[macro_export]
macro_rules! println {
	(_, $($arg:tt)*) => {
			$crate::println_label($crate::OutputLabel::None, format!($($arg)*))
	};
	($label:expr, $($arg:tt)*) => {
		$crate::println_label($label, format!($($arg)*))
	};
}

// TODO: document
/// **Note**: this replace the builtin eprintln macro
#[macro_export]
macro_rules! eprintln {
	($($arg:tt)*) => {
		$crate::println_label($crate::OutputLabel::Error, format!($($arg)*))
	};
}

// TODO: add usage example
/// Print the given message with a carriage return at the end
/// Useful for mid-process logging
#[macro_export]
macro_rules! print_r {
	($label:expr, $($arg:tt)*) => {
		$crate::print_r_label($label, format!($($arg)*))
	};
}

// TODO: document
/// Formats your message with the specified label
#[macro_export]
macro_rules! format_label {
	(_, $($arg:tt)*) => {
		$crate::pretty_output($crate::OutputLabel::None, format!($($arg)*), false)
	};
	($label:expr, $($arg:tt)*) => {
		$crate::pretty_output($label, format!($($arg)*), false)
	};
}
