//! Integration tests for Foundation type bindings

use ios_sys::foundation::{CGPoint, CGRect, CGSize, NSRange};

#[test]
fn test_cgpoint() {
    let point = CGPoint { x: 10.0, y: 20.0 };
    assert_eq!(point.x, 10.0);
    assert_eq!(point.y, 20.0);
}

#[test]
fn test_cgsize() {
    let size = CGSize {
        width: 100.0,
        height: 200.0,
    };
    assert_eq!(size.width, 100.0);
    assert_eq!(size.height, 200.0);
}

#[test]
fn test_cgrect() {
    let rect = CGRect {
        origin: CGPoint { x: 0.0, y: 0.0 },
        size: CGSize {
            width: 320.0,
            height: 480.0,
        },
    };
    assert_eq!(rect.origin.x, 0.0);
    assert_eq!(rect.origin.y, 0.0);
    assert_eq!(rect.size.width, 320.0);
    assert_eq!(rect.size.height, 480.0);
}

#[test]
fn test_nsrange() {
    let range = NSRange {
        location: 5,
        length: 10,
    };
    assert_eq!(range.location, 5);
    assert_eq!(range.length, 10);
}

#[test]
fn test_foundation_types_are_copy() {
    // Verify these types implement Copy (should compile)
    let point1 = CGPoint { x: 1.0, y: 2.0 };
    let point2 = point1; // This should copy, not move
    let _ = point1; // point1 should still be usable

    let size1 = CGSize {
        width: 1.0,
        height: 2.0,
    };
    let size2 = size1;
    let _ = size1;

    let range1 = NSRange {
        location: 0,
        length: 5,
    };
    let range2 = range1;
    let _ = range1;
}
