use slug::slugify;

/// Generate a URL-friendly slug from a string
/// Django: utils/slug.py - generate_unique_slug
pub fn generate_slug(text: &str) -> String {
    slugify(text)
}

/// Generate slug with counter for uniqueness
pub fn generate_slug_with_counter(text: &str, counter: i32) -> String {
    if counter == 0 {
        generate_slug(text)
    } else {
        format!("{}-{}", generate_slug(text), counter)
    }
}
