// Wrapper header for bindgen
// This file tells bindgen which ElleKit headers to generate bindings for

// Main ElleKit C API
#include "../submodules/ellekit/ellekitc/include/ellekit.h"

// Note: We only include ellekit.h because it already includes:
// - mach.h (Mach kernel primitives)
// - dyld.h (Dynamic linker structures)
// - sandbox.h (Sandbox utilities)
