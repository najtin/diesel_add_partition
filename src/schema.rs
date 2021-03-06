// @generated automatically by Diesel CLI.

diesel::table! {
    course (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    course_user (course_id, member_user_id) {
        course_id -> Int8,
        member_user_id -> Int8,
        role -> Text,
    }
}

diesel::table! {
    course_user_partiton_22_ss (course_id, member_user_id) {
        course_id -> Int8,
        member_user_id -> Int8,
        role -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Int8,
        last_name -> Text,
        common_name -> Text,
        last_activity_year -> Int2,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    course,
    course_user,
    course_user_partiton_22_ss,
    user,
);
