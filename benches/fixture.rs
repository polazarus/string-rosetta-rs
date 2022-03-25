pub const SAMPLES: &[&str] = &[
    // Empty handling
    "",
    // Barely used
    "1",
    // kstring's max small-string size
    "123456789012345",
    // Boundary conditions for most small-string optimizations
    "1234567890123456789012",
    "12345678901234567890123",
    "123456789012345678901234",
    "1234567890123456789012345",
    // Small heap
    "1234567890123456789012345678901234567890123456789012345678901234",
    // Large heap
    "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
];