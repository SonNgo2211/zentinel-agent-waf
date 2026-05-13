//! Shannon Entropy Analysis
//!
//! Calculates the entropy of input strings to detect obfuscation,
//! encryption, and high-density patterns common in attacks.

/// Shannon entropy of a string
pub fn calculate_entropy(data: &str) -> f32 {
    if data.is_empty() {
        return 0.0;
    }

    let len = data.len() as f32;
    let mut counts = [0usize; 256];

    for byte in data.as_bytes() {
        counts[*byte as usize] += 1;
    }

    let mut entropy = 0.0f32;
    for count in counts.iter() {
        if *count > 0 {
            let p = *count as f32 / len;
            entropy -= p * p.log2();
        }
    }

    entropy
}

/// Normalizes entropy to a 0.0 - 1.0 range
/// 8.0 is the maximum entropy for 8-bit bytes.
pub fn normalized_entropy(data: &str) -> f32 {
    let entropy = calculate_entropy(data);
    (entropy / 8.0).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_basic() {
        // Low entropy
        assert!(calculate_entropy("aaaaa") < 1.0);
        
        // High entropy (diverse characters)
        assert!(calculate_entropy("abcdefghijklmnopqrstuvwxyz") > 4.0);
    }

    #[test]
    fn test_obfuscation_detection() {
        // Normal SQL
        let normal = calculate_entropy("SELECT * FROM users");
        
        // Obfuscated SQL (Base64 or random-looking)
        let obfuscated = calculate_entropy("U0VMRUNUICogRlJPTSB1c2Vycw==");
        
        assert!(obfuscated > normal);
    }
}
