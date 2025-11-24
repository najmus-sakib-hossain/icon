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
pub fn svg_to_react_native(content: &str, name: &str, snippet: bool) -> String {
    let mut svg = svg_to_jsx(content);

    // Replacements map
    let replacements = vec![
        ("svg", "Svg"),
        ("path", "Path"),
        ("g", "G"),
        ("circle", "Circle"),
        ("rect", "Rect"),
        ("line", "Line"),
        ("polyline", "Polyline"),
        ("polygon", "Polygon"),
        ("ellipse", "Ellipse"),
        ("text", "Text"),
        ("tspan", "Tspan"),
        ("textPath", "TextPath"),
        ("defs", "Defs"),
        ("use", "Use"),
        ("symbol", "Symbol"),
        ("linearGradient", "LinearGradient"),
        ("radialGradient", "RadialGradient"),
        ("stop", "Stop"),
    ];

    let mut used_components = Vec::new();

    for (from, to) in &replacements {
        let re_open = Regex::new(&format!(r"<{}([\s>])", from)).unwrap();
        let re_close = Regex::new(&format!(r"</{}>", from)).unwrap();
        
        if re_open.is_match(&svg) || re_close.is_match(&svg) {
            used_components.push(to.to_string());
            svg = re_open.replace_all(&svg, format!("<{}$1", to).as_str()).to_string();
            svg = re_close.replace_all(&svg, format!("</{}>", to).as_str()).to_string();
        }
    }

    // Specific attribute replacements for React Native
    svg = svg.replace("className=", "");
    svg = svg.replace("href=", "xlinkHref=");
    svg = svg.replace("clipPath=", "clipPath="); // Already camelCase from svg_to_jsx?
    // svg_to_jsx handles kebab-case to camelCase, so stroke-width -> strokeWidth is already done.

    // Generate imports
    let mut imports = String::from("import Svg");
    let other_components: Vec<String> = used_components.iter()
        .filter(|c| c.as_str() != "Svg")
        .cloned()
        .collect();
    
    if !other_components.is_empty() {
        imports.push_str(", { ");
        imports.push_str(&other_components.join(", "));
        imports.push_str(" }");
    }
    imports.push_str(" from 'react-native-svg';");

    let code = format!(
        r#"
export function {}(props) {{
  return (
    {}
  )
}}"#,
        name, svg
    );

    if snippet {
        code
    } else {
        format!("import React from 'react';\n{}\n\n{}\nexport default {};", imports, code, name)
    }
}

#[allow(dead_code)]
pub fn svg_to_qwik(content: &str, name: &str, snippet: bool) -> String {
    let svg = svg_to_jsx(content); // Qwik uses JSX-like syntax
    // Inject props and key
    let re_svg = Regex::new(r"<svg (.*?)>").unwrap();
    let svg_with_props = re_svg.replace(&svg, "<svg $1 {...props} key={key}>").to_string();

    let code = format!(
        r#"
export function {}(props: QwikIntrinsicElements['svg'], key: string) {{
  return (
    {}
  )
}}"#,
        name, svg_with_props
    );

    if snippet {
        code
    } else {
        format!("import type {{ QwikIntrinsicElements }} from '@builder.io/qwik'\n{}\nexport default {}", code, name)
    }
}

#[allow(dead_code)]
pub fn svg_to_solid(content: &str, name: &str, snippet: bool) -> String {
    // Solid uses standard SVG attributes (class, not className), so we don't use svg_to_jsx
    // But we might want to clean it up? Icones uses raw svg but injects props.
    let svg = content.to_string();
    
    // Inject props
    let re_svg = Regex::new(r"<svg (.*?)>").unwrap();
    let svg_with_props = re_svg.replace(&svg, "<svg $1 {...props}>").to_string();

    let code = format!(
        r#"
export function {}(props: JSX.IntrinsicElements['svg']) {{
  return (
    {}
  )
}}"#,
        name, svg_with_props
    );

    if snippet {
        code
    } else {
        format!("import type {{ JSX }} from 'solid-js'\n{}\nexport default {}", code, name)
    }
}

#[allow(dead_code)]
pub fn svg_to_astro(content: &str) -> String {
    // Astro is HTML-like
    let svg = content.to_string();
    
    // Inject props
    let re_svg = Regex::new(r"<svg (.*?)>").unwrap();
    let svg_with_props = re_svg.replace(&svg, "<svg $1 {{...props}}>").to_string();

    format!(
        r#"---
const props = Astro.props
---

{}"#,
        svg_with_props
    )
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
