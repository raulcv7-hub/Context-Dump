/// Root entry point for unit tests hierarchy.
pub mod common;
pub mod unit {
    pub mod core {
        pub mod config;
        pub mod config_inference;
        pub mod file_node;
        pub mod persistence;
        pub mod tokens;
        pub mod tree;
    }
    pub mod adapters {
        pub mod depth;
        pub mod filters;
        pub mod filters_path;
        pub mod noise;
        pub mod noise_elite;
        pub mod notebook;
        pub mod office;
        pub mod output;
        pub mod parsers;
        pub mod scanner;
        pub mod scanner_links;
    }
}
