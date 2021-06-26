table! {
    posts (id) {
        id -> Int8,
        title -> Varchar,
        author -> Varchar,
        body -> Nullable<Text>,
        published -> Bool,
    }
}
