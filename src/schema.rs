table! {
    t_user (uuid) {
        uuid -> Char,
        gender -> Tinyint,
        phone_number -> Char,
        head_url -> Varchar,
        account -> Varchar,
        password -> Varchar,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}
