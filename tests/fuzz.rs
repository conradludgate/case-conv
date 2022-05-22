use case_conv::to_lowercase;

fn lowercase(data: &str) {
    let lower = to_lowercase(data);
    assert_eq!(lower, data.to_lowercase());
}

#[test]
fn fuzz1() {
    let input = "\0\0\n\0\0ן\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0ןv\u{5ff}\0ןן\u{7f}\0\0\0\0\0ןן\0ח\0\0ן";
    lowercase(input)
}

#[test]
fn fuzz2() {
    let input = ")))))\t\u{c}\0)))))))))))))))))))))))))!)))))))))))))))))))))))))))))))))))))\0y)),))\0\0\0FF=\0ӟFFF!F\u{7f}\u{7f}\0)";
    lowercase(input)
}
