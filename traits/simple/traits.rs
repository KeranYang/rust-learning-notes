fn main() {
    simple_trait_example();
}

fn simple_trait_example() {
    let article = Article {
        headline: String::from("Breaking News"),
        content: String::from("Rust is awesome!"),
    };
    println!("Summary: {}", article.summarize());
    println!("Default Summary: {}", article.default_summary());
    notify(&article);
    let news = News {
        headline: String::from("Local News"),
        location: String::from("Somewhere"),
        author: String::from("Someone"),
        content: String::from("More Rust news!"),
    };
    // This works because News also implements Summary
    // and this method accepts multiple types implementing Summary.
    notify_two_items_v0(&article, &news);
    // This won't work because article and news are of different types.
    // notify_two_items_v1(&article, news); // -> compile error
}

// a simple trait definition
pub trait Summary {
    fn summarize(&self) -> String;

    // Note: default implementation for a method
    // if a type implements Summary but does not provide its own implementation of default_summary,
    // this default implementation will be used.
    // default method can call other methods in the trait.
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

// implement the Summary trait for a struct
pub struct Article {
    pub headline: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

pub struct News {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// traits as parameters example
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Note: two parameters implementing the same trait
// using impl Trait syntax that allows both parameters to be of different types.
fn notify_two_items_v0(item1: &impl Summary, item2: &impl Summary) {
    println!("Item 1: {}", item1.summarize());
    println!("Item 2: {}", item2.summarize());
}

// Note: using generic type parameter with trait bound
// to ensure both parameters are of the same type.
fn notify_two_items_v1<T: Summary>(item1: &T, item2: &T) {
    println!("Item 1: {}", item1.summarize());
    println!("Item 2: {}", item2.summarize());
}

// Note: multiple trait bounds can be specified using the + syntax.
fn notify_multiple_traits<T: Summary + std::fmt::Display>(item: &T) {
    println!("Summary: {}", item.summarize());
    println!("Display: {}", item);
}

// using where clause for cleaner syntax with multiple trait bounds
fn notify_with_where_clause<T, U>(item1: &T, item2: &U)
where
    T: Summary + std::fmt::Display,
    U: Summary,
{
    println!("Item 1 Summary: {}", item1.summarize());
    println!("Item 2 Summary: {}", item2.summarize());
}

// Note: returning types that implement traits
fn create_summarizable() -> impl Summary {
    if true {
        News {
            headline: String::from("Latest News"),
            location: String::from("Anywhere"),
            author: String::from("Anyone"),
            content: String::from("This is the content of the latest news."),
        }
    } else {
        /*
        This won't work because the two branches return different types.
        Note: we can only use impl Trait syntax for return types when both branches return the same concrete type.
        Article {
            headline: String::from("New Article"),
            content: String::from("This is the content of the new article."),
        }
         */
        News {
            headline: String::from("Latest News"),
            location: String::from("Anywhere"),
            author: String::from("Anyone"),
            content: String::from("This is the content of the latest news."),
        }
    }
}

// Note: we can conditionally implement traits for types using generics and trait bounds.
struct Pair<T> {
    x: T,
    y: T,
}

// any type T can be used to create a Pair<T>
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// type T that implements PartialOrd and Display can use the cmp_display method.
impl<T: PartialOrd + std::fmt::Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}