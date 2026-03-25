/// Root entry point for unit tests hierarchy.
pub mod common;
pub mod unit {
    pub mod core {
        pub mod config;
        pub mod config_inference;
        pub mod persistence;
        pub mod tree;
        pub mod tokens;
        pub mod file_node;
    }
    pub mod adapters {
        pub mod scanner;
        pub mod noise;
        pub mod noise_elite;
        pub mod filters;
        pub mod filters_path;
        pub mod depth;
        pub mod parsers;
        pub mod office;
        pub mod output;
        pub mod notebook;
    }
}