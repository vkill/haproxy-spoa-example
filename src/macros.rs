macro_rules! make_frame_kv_list_payload {
    (
        $(#[$struct_meta:meta])*
        $struct_vis:vis struct $struct_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : $field_type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$struct_meta])*
        $struct_vis struct $struct_name {
            $(
                $(#[$field_meta])*
                $field_vis $field_name: $field_type,
            )*
        }

        paste::item! {
            impl $struct_name {
                $(
                    pub fn [<$field_name _name>] () -> String {
                        stringify!($field_name).replace("_", "-")
                    }
                )*
            }
        }
    }
}
