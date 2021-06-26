table! {
    posts (id) {
        id -> Int8,
        title -> Varchar,
        author -> Varchar,
        content -> Nullable<Text>,
        published -> Bool,
    }
}
