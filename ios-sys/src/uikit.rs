//! UIKit framework bindings
//!
//! Minimal bindings to UIKit framework types and constants.
//! These are commonly needed for iOS UI hooking and development.

use crate::foundation::{CGFloat, CGPoint, CGRect, CGSize, NSInteger};
use crate::objc::id;

// UIEdgeInsets
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UIEdgeInsets {
    pub top: CGFloat,
    pub left: CGFloat,
    pub bottom: CGFloat,
    pub right: CGFloat,
}

impl UIEdgeInsets {
    pub fn new(top: CGFloat, left: CGFloat, bottom: CGFloat, right: CGFloat) -> Self {
        UIEdgeInsets {
            top,
            left,
            bottom,
            right,
        }
    }

    pub fn zero() -> Self {
        UIEdgeInsets::new(0.0, 0.0, 0.0, 0.0)
    }
}

// UIOffset
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UIOffset {
    pub horizontal: CGFloat,
    pub vertical: CGFloat,
}

// UIApplicationState
pub type UIApplicationState = NSInteger;
pub const UIApplicationStateActive: UIApplicationState = 0;
pub const UIApplicationStateInactive: UIApplicationState = 1;
pub const UIApplicationStateBackground: UIApplicationState = 2;

// UIViewAnimationCurve
pub type UIViewAnimationCurve = NSInteger;
pub const UIViewAnimationCurveEaseInOut: UIViewAnimationCurve = 0;
pub const UIViewAnimationCurveEaseIn: UIViewAnimationCurve = 1;
pub const UIViewAnimationCurveEaseOut: UIViewAnimationCurve = 2;
pub const UIViewAnimationCurveLinear: UIViewAnimationCurve = 3;

// UIViewAnimationOptions
pub type UIViewAnimationOptions = NSInteger;
pub const UIViewAnimationOptionLayoutSubviews: UIViewAnimationOptions = 1 << 0;
pub const UIViewAnimationOptionAllowUserInteraction: UIViewAnimationOptions = 1 << 1;
pub const UIViewAnimationOptionBeginFromCurrentState: UIViewAnimationOptions = 1 << 2;
pub const UIViewAnimationOptionRepeat: UIViewAnimationOptions = 1 << 3;
pub const UIViewAnimationOptionAutoreverse: UIViewAnimationOptions = 1 << 4;
pub const UIViewAnimationOptionOverrideInheritedDuration: UIViewAnimationOptions = 1 << 5;
pub const UIViewAnimationOptionOverrideInheritedCurve: UIViewAnimationOptions = 1 << 6;
pub const UIViewAnimationOptionAllowAnimatedContent: UIViewAnimationOptions = 1 << 7;
pub const UIViewAnimationOptionShowHideTransitionViews: UIViewAnimationOptions = 1 << 8;
pub const UIViewAnimationOptionOverrideInheritedOptions: UIViewAnimationOptions = 1 << 9;

// UIViewContentMode
pub type UIViewContentMode = NSInteger;
pub const UIViewContentModeScaleToFill: UIViewContentMode = 0;
pub const UIViewContentModeScaleAspectFit: UIViewContentMode = 1;
pub const UIViewContentModeScaleAspectFill: UIViewContentMode = 2;
pub const UIViewContentModeRedraw: UIViewContentMode = 3;
pub const UIViewContentModeCenter: UIViewContentMode = 4;
pub const UIViewContentModeTop: UIViewContentMode = 5;
pub const UIViewContentModeBottom: UIViewContentMode = 6;
pub const UIViewContentModeLeft: UIViewContentMode = 7;
pub const UIViewContentModeRight: UIViewContentMode = 8;
pub const UIViewContentModeTopLeft: UIViewContentMode = 9;
pub const UIViewContentModeTopRight: UIViewContentMode = 10;
pub const UIViewContentModeBottomLeft: UIViewContentMode = 11;
pub const UIViewContentModeBottomRight: UIViewContentMode = 12;

// UIControlState
pub type UIControlState = NSInteger;
pub const UIControlStateNormal: UIControlState = 0;
pub const UIControlStateHighlighted: UIControlState = 1 << 0;
pub const UIControlStateDisabled: UIControlState = 1 << 1;
pub const UIControlStateSelected: UIControlState = 1 << 2;
pub const UIControlStateFocused: UIControlState = 1 << 3;

// UIControlEvents
pub type UIControlEvents = NSInteger;
pub const UIControlEventTouchDown: UIControlEvents = 1 << 0;
pub const UIControlEventTouchDownRepeat: UIControlEvents = 1 << 1;
pub const UIControlEventTouchDragInside: UIControlEvents = 1 << 2;
pub const UIControlEventTouchDragOutside: UIControlEvents = 1 << 3;
pub const UIControlEventTouchDragEnter: UIControlEvents = 1 << 4;
pub const UIControlEventTouchDragExit: UIControlEvents = 1 << 5;
pub const UIControlEventTouchUpInside: UIControlEvents = 1 << 6;
pub const UIControlEventTouchUpOutside: UIControlEvents = 1 << 7;
pub const UIControlEventTouchCancel: UIControlEvents = 1 << 8;
pub const UIControlEventValueChanged: UIControlEvents = 1 << 12;

// UIColor constants
extern "C" {
    pub static UIColorBlackColor: id;
    pub static UIColorWhiteColor: id;
    pub static UIColorRedColor: id;
    pub static UIColorGreenColor: id;
    pub static UIColorBlueColor: id;
    pub static UIColorClearColor: id;
}

// Common UIKit notification names (declared in actual framework)
pub const UIAPPLICATION_DID_FINISH_LAUNCHING: &[u8] = b"UIApplicationDidFinishLaunchingNotification\0";
pub const UIAPPLICATION_WILL_TERMINATE: &[u8] = b"UIApplicationWillTerminateNotification\0";
pub const UIAPPLICATION_DID_ENTER_BACKGROUND: &[u8] = b"UIApplicationDidEnterBackgroundNotification\0";
pub const UIAPPLICATION_WILL_ENTER_FOREGROUND: &[u8] = b"UIApplicationWillEnterForegroundNotification\0";
