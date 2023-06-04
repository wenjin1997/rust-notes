mod rc_use;
mod dag;
mod strtok;
mod enum_use;
mod reader;
mod mywrite;
mod buf_builder;
mod myParse;
mod parseTraitDRY;
mod parseTraitDRY2;
mod add_trait;
mod animal;
mod formatter;
mod ex2;
mod course13 {
    mod ex4_sentence_iter;
}
mod course14 {
    mod raw_buffer;
    mod sync_send;
    mod from_into_trait;
    mod as_ref_trait;
    mod deref_trait;
    mod debug_display_default_trait;
}

/// 15. 数据结构：智能指针
mod course15 {
    mod my_allocator;
    mod borrow_use;
    mod cow_use;
    mod mutex_guard_use;
    mod my_string;
}

mod course16 {
    mod array_deref;
    mod string_deref;
    mod box_t;
}

mod course17 {
    mod hashmap;
    mod hashmap2;
    mod my_hashmap;
    mod btree_map;
}

mod course19 {
    mod closure;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
