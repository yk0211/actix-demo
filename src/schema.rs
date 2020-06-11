table! {
    t_user (uuid) {
        uuid -> Char,
        account -> Varchar,
        password -> Varchar,
        nickname -> Varchar,
        gender -> Tinyint,
        phone_number -> Char,
        head_image -> Varchar,
        create_at -> Timestamp,
        last_login_at -> Timestamp,
    }
}
