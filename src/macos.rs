// Use the objc crate to work with Objective-C objects and runtime
extern crate objc;

// Declare the external functions from CoreGraphics framework
extern "C" {
   // CGPreflightScreenCaptureAccess doesn't take parameters and returns a bool.
   // True if the app either already has screen capture access or if the system
   // version is earlier than 10.15. False otherwise.
   fn CGPreflightScreenCaptureAccess() -> bool;

   // CGRequestScreenCaptureAccess doesn't take parameters and returns a bool.
   // True if the user grants permission or if the app already has permission.
   // False if the user denies permission or if an error occurs.
   fn CGRequestScreenCaptureAccess() -> bool;
}

/// Check if the user has already granted screen capture access or if the system
/// version is earlier than 10.15.
pub fn preflight_access() -> bool {
   unsafe {
      // Safety: Calling an external C function, considered unsafe in Rust
      CGPreflightScreenCaptureAccess()
   }
}

/// Request screen capture access from the user.
pub fn request_access() -> bool {
   unsafe {
      // Safety: Calling an external C function, considered unsafe in Rust
      CGRequestScreenCaptureAccess()
   }
}
