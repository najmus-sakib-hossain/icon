use regex::{Regex, Captures};

#[allow(dead_code)]
pub fn extract_styles(content: &str) -> (String, String) {
    let re = Regex::new(r"(?si)<style[^>]*>(.*?)</style>").unwrap();
    let mut styles = Vec::new();
    let template = re.replace_all(content, |caps: &Captures| {
        styles.push(caps[1].to_string());
        ""
    }).to_string();
    
    (template, styles.join("\n"))
}

#[allow(dead_code)]
pub fn svg_to_jsx(content: &str) -> String {
    let mut jsx = content.to_string();
    
    // Replace class= with className=
    let re_class = Regex::new(r"class=").unwrap();
    jsx = re_class.replace_all(&jsx, "className=").to_string();
    
    // Replace kebab-case attributes with camelCase
    let re_attr = Regex::new(r"([a-z]+-[a-z0-9-]+)=").unwrap();
    jsx = re_attr.replace_all(&jsx, |caps: &Captures| {
        let attr = &caps[1];
        // Skip data-* and aria-* attributes
        if attr.starts_with("data-") || attr.starts_with("aria-") {
            return format!("{}=", attr);
        }
        
        let parts: Vec<&str> = attr.split('-').collect();
        let mut camel = parts[0].to_string();
        for part in parts.iter().skip(1) {
            let mut chars = part.chars();
            if let Some(f) = chars.next() {
                camel.push(f.to_uppercase().next().unwrap());
                camel.push_str(chars.as_str());
            }
        }
        format!("{}=", camel)
    }).to_string();

    // Replace comments <!-- --> with {/* */}
    jsx = jsx.replace("<!--", "{/*").replace("-->", "*/}");
    
    jsx
}

#[allow(dead_code)]
pub fn to_pascal_case(s: &str) -> String {
    let s = s.replace("-", " ");
    s.split_whitespace()
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
}
