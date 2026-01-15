//! Data models for browser automation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use chromiumoxide::cdp::browser_protocol::network::CookieSameSite;

/// ARIA tree node with @eN reference ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriaNode {
    /// Element reference ID (e.g., "@e1", "@e2")
    pub ref_id: String,
    /// ARIA role (e.g., "button", "textbox", "link")
    pub role: String,
    /// Accessible name
    #[serde(default)]
    pub name: Option<String>,
    /// Node value (for inputs)
    #[serde(default)]
    pub value: Option<String>,
    /// Whether the element is focusable
    #[serde(default)]
    pub focusable: bool,
    /// Whether the element is focused
    #[serde(default)]
    pub focused: bool,
    /// Child nodes
    #[serde(default)]
    pub children: Vec<AriaNode>,
}

/// ARIA tree snapshot response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriaSnapshot {
    /// Page URL
    pub url: String,
    /// Page title
    pub title: String,
    /// Root ARIA nodes
    pub nodes: Vec<AriaNode>,
    /// Total element count
    pub element_count: usize,
}

/// Screenshot response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotResult {
    /// Base64-encoded PNG data (if no path specified)
    #[serde(default)]
    pub data: Option<String>,
    /// File path (if path was specified)
    #[serde(default)]
    pub path: Option<String>,
    /// Image dimensions
    pub width: u32,
    pub height: u32,
}

/// Navigation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationResult {
    /// Final URL after navigation
    pub url: String,
    /// Page title
    pub title: String,
    /// HTTP status code
    #[serde(default)]
    pub status: Option<u16>,
}

/// Browser session info.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Session ID
    pub id: String,
    /// Current URL
    #[serde(default)]
    pub url: Option<String>,
    /// Whether this is the active session
    pub active: bool,
}

/// Saved auth state info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedState {
    /// State name
    pub name: String,
    /// Domain(s) the state applies to
    pub domains: Vec<String>,
    /// When the state was saved
    pub saved_at: String,
}

/// Serializable cookie for auth state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableCookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    #[serde(default)]
    pub expires: Option<f64>,
    #[serde(default)]
    pub secure: bool,
    #[serde(default)]
    pub http_only: bool,
    #[serde(default)]
    pub same_site: Option<CookieSameSite>,
}

/// Local storage snapshot for a single origin.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalStorageState {
    #[serde(default)]
    pub origin: String,
    #[serde(default)]
    pub items: HashMap<String, String>,
}

/// Auth state snapshot with cookies and localStorage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthState {
    #[serde(default)]
    pub cookies: Vec<SerializableCookie>,
    #[serde(default)]
    pub local_storage: LocalStorageState,
    #[serde(default)]
    pub saved_at: String,
}

/// Click result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickResult {
    /// Whether click was successful
    pub success: bool,
    /// Element that was clicked (for debugging)
    #[serde(default)]
    pub element: Option<String>,
}

/// Fill result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FillResult {
    /// Whether fill was successful
    pub success: bool,
    /// Value that was filled
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aria_node_serialization() {
        let node = AriaNode {
            ref_id: "@e1".to_string(),
            role: "button".to_string(),
            name: Some("Submit".to_string()),
            value: None,
            focusable: true,
            focused: false,
            children: vec![],
        };

        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("@e1"));
        assert!(json.contains("button"));
        assert!(json.contains("Submit"));

        let parsed: AriaNode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.ref_id, "@e1");
        assert_eq!(parsed.role, "button");
        assert_eq!(parsed.name, Some("Submit".to_string()));
    }

    #[test]
    fn test_aria_node_deserialization_with_defaults() {
        let json = r#"{"ref_id": "@e5", "role": "link"}"#;
        let node: AriaNode = serde_json::from_str(json).unwrap();

        assert_eq!(node.ref_id, "@e5");
        assert_eq!(node.role, "link");
        assert_eq!(node.name, None);
        assert_eq!(node.value, None);
        assert!(!node.focusable);
        assert!(!node.focused);
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_navigation_result_serialization() {
        let result = NavigationResult {
            url: "https://example.com/page".to_string(),
            title: "Example Page".to_string(),
            status: Some(200),
        };

        let json = serde_json::to_string(&result).unwrap();
        let parsed: NavigationResult = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.url, "https://example.com/page");
        assert_eq!(parsed.title, "Example Page");
        assert_eq!(parsed.status, Some(200));
    }

    #[test]
    fn test_screenshot_result_with_path() {
        let result = ScreenshotResult {
            data: None,
            path: Some("/tmp/screenshot.png".to_string()),
            width: 1920,
            height: 1080,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("/tmp/screenshot.png"));
        assert!(json.contains("1920"));
        assert!(json.contains("1080"));
    }

    #[test]
    fn test_screenshot_result_with_base64() {
        let result = ScreenshotResult {
            data: Some("iVBORw0KGgo...".to_string()),
            path: None,
            width: 800,
            height: 600,
        };

        let json = serde_json::to_string(&result).unwrap();
        let parsed: ScreenshotResult = serde_json::from_str(&json).unwrap();

        assert!(parsed.data.is_some());
        assert!(parsed.path.is_none());
    }

    #[test]
    fn test_session_info() {
        let session = SessionInfo {
            id: "session-abc".to_string(),
            url: Some("https://example.com".to_string()),
            active: true,
        };

        let json = serde_json::to_string(&session).unwrap();
        let parsed: SessionInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, "session-abc");
        assert!(parsed.active);
    }

    #[test]
    fn test_auth_state_serialization() {
        let state = AuthState {
            cookies: vec![SerializableCookie {
                name: "session".to_string(),
                value: "abc123".to_string(),
                domain: ".example.com".to_string(),
                path: "/".to_string(),
                expires: Some(1700000000.0),
                secure: true,
                http_only: true,
                same_site: None,
            }],
            local_storage: LocalStorageState {
                origin: "https://example.com".to_string(),
                items: {
                    let mut map = HashMap::new();
                    map.insert("token".to_string(), "xyz".to_string());
                    map
                },
            },
            saved_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&state).unwrap();
        let parsed: AuthState = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.cookies.len(), 1);
        assert_eq!(parsed.cookies[0].name, "session");
        assert_eq!(
            parsed.local_storage.items.get("token"),
            Some(&"xyz".to_string())
        );
    }

    #[test]
    fn test_click_result() {
        let result = ClickResult {
            success: true,
            element: Some("button#submit".to_string()),
        };

        let json = serde_json::to_string(&result).unwrap();
        let parsed: ClickResult = serde_json::from_str(&json).unwrap();

        assert!(parsed.success);
        assert_eq!(parsed.element, Some("button#submit".to_string()));
    }

    #[test]
    fn test_fill_result() {
        let result = FillResult {
            success: true,
            value: "test@example.com".to_string(),
        };

        let json = serde_json::to_string(&result).unwrap();
        let parsed: FillResult = serde_json::from_str(&json).unwrap();

        assert!(parsed.success);
        assert_eq!(parsed.value, "test@example.com");
    }
}
