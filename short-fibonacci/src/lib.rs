/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    return Vec::new();
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    return vec![0 as u8; count];
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut v = vec![1 as u8,1 as u8];

    let mut a = v[0];
    let mut b = v[1];
    let count = 5;
    for _ in 2..count {
        let c = a+b;
        v.push(c);
        b = a;
        a = c;
    }

    return v;    
}
