pub mod synapsed {
    pub mod api {
        pub mod settings {
            tonic::include_proto!("synapsed.api.settings");
        }
        pub mod pref {
            tonic::include_proto!("synapsed.api.pref");
        }
        pub mod helper {
            tonic::include_proto!("synapsed.api.helper");
        }
    }
    pub mod pref {}
}
