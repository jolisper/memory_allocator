use super::{align, alloc, alloc_size, free, get_header, mem, WordSize};

#[test]
fn test_align() {
    assert_eq!(align(3), 8);
    assert_eq!(align(8), 8);
    assert_eq!(align(12), 16);
    assert_eq!(align(16), 16);
}

#[test]
fn test_alloc_size() {
    assert_eq!(alloc_size(10), 34);
    assert_eq!(alloc_size(2), 26);
    assert_eq!(alloc_size(200), 224);
}

#[test]
fn test_alloc() {
    // --------------------------------------
    // Test case 1: Alignment
    //
    // A request for 3 bytes is aligned to 8.
    //
    let p1 = alloc(3);
    let p1b = get_header(p1);
    assert!(unsafe { (*p1b).size == mem::size_of::<WordSize>() });

    // --------------------------------------
    // Test case 2: Exact amount of aligned bytes
    //
    let p2 = alloc(8);
    let p2b = get_header(p2);
    assert!(unsafe { (*p2b).size == 8 });

    // --------------------------------------
    // Test case 3: Free the object
    //
    free(p2);
    assert!(unsafe { !(*p2b).used });

    // --------------------------------------
    // Test case 4: The block is reused
    //
    // A consequent allocation of the same size reuses
    // the previously freed block.
    let p3 = alloc(8);
    let p3b = get_header(p3);
    assert!(unsafe { (*p3b).size == 8 });
    assert!(p3b == p2b);
}
