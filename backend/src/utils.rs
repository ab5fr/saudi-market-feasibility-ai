/// Utility functions for the backend

/// Generate a unique CUID-style identifier
pub fn generate_id(prefix: &str) -> String {
    // TODO: Implement proper CUID2 generation
    // For now, use a simple UUID-based approach
    let uuid = uuid::Uuid::new_v4().to_string();
    let short_uuid = &uuid.replace("-", "")[..16];
    format!("{}_{}", prefix, short_uuid)
}

/// Sanitize user input to prevent injection attacks
pub fn sanitize_input(input: &str) -> String {
    input
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#x27;")
        .trim()
        .to_string()
}

/// Validate Saudi phone number format
pub fn validate_saudi_phone(phone: &str) -> bool {
    // Saudi phone numbers typically:
    // - Start with +966 or 05
    // - Followed by 8 digits
    let phone = phone.replace(" ", "").replace("-", "");
    
    if phone.starts_with("+966") && phone.len() == 13 {
        return phone[4..].chars().all(|c| c.is_ascii_digit());
    }
    
    if phone.starts_with("05") && phone.len() == 10 {
        return phone[2..].chars().all(|c| c.is_ascii_digit());
    }
    
    false
}

/// Format currency in Saudi Riyals
pub fn format_sar(amount: f64) -> String {
    format!("SAR {:.2}", amount)
}

/// Truncate text to a maximum length with ellipsis
pub fn truncate(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    }
}
