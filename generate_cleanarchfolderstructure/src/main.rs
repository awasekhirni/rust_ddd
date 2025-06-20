//copyright 2025 β ORI Inc. Canada. Awase Khirni Syed
use std::fs;
use std::path::Path;

fn main() {
    let root = "your_project";
    create_dir_if_not_exists(root);

    // Define the folder structure
    let folders = [
        "api/auth",
        "api/user",
        "api/product",
        "api/category",
        "application/auth",
        "application/user",
        "application/product",
        "domain/auth",
        "domain/user",
        "domain/product",
        "domain/category",
        "infrastructure/database",
        "infrastructure/user",
        "infrastructure/auth",
        "infrastructure/product",
        "infrastructure/mfa",
        "infrastructure/category",
        "shared",
        "migrations",
        "tests/unit",
        "tests/integration",
        "tests/functional",
    ];

    // Create all folders
    for folder in &folders {
        create_dir_if_not_exists(&format!("{}/{}", root, folder));
    }

    // Scaffold files with basic content
    scaffold_files(root);

    // Create placeholder files
    create_file(&format!("{}/codeplay_instructions.txt", root), "");
    create_file(
        &format!("{}/app.rs", root),
        "// Entry point\nfn main() {\n    println!(\"Hello, world!\");\n}\n",
    );
    create_file(
        &format!("{}/Cargo.toml", root),
        "[package]\nname = \"your_project\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
    );

    println!("✅ Project structure created under '{}'", root);
}

fn create_dir_if_not_exists(path: &str) {
    if let Err(e) = fs::create_dir_all(path) {
        eprintln!("Error creating directory {}: {}", path, e);
    }
}

fn create_file(path: &str, content: &str) {
    if let Err(e) = fs::write(path, content) {
        eprintln!("Error writing file {}: {}", path, e);
    }
}

fn scaffold_files(root: &str) {
    let files = vec![
        // API Layer
        ("api/auth/auth_controller.rs", "// Auth Controller"),
        ("api/auth/auth_service.rs", "// Auth Service"),
        ("api/auth/auth_dto.rs", "// Auth DTOs"),
        ("api/auth/auth_mapper.rs", "// Auth Mappers"),
        ("api/user/user_controller.rs", "// User Controller"),
        ("api/user/user_service.rs", "// User Service"),
        ("api/user/user_dto.rs", "// User DTOs"),
        ("api/product/product_controller.rs", "// Product Controller"),
        ("api/product/product_service.rs", "// Product Service"),
        (
            "api/category/category_controller.rs",
            "// Category Controller",
        ),
        ("api/category/category_service.rs", "// Category Service"),
        // Application Layer
        ("application/auth/auth_service.rs", "// Auth Business Logic"),
        ("application/auth/auth_dto.rs", "// Auth DTOs"),
        ("application/auth/mfa_service.rs", "// MFA Logic"),
        ("application/user/user_service.rs", "// User Business Logic"),
        ("application/user/user_dto.rs", "// User DTOs"),
        (
            "application/product/product_service.rs",
            "// Product Business Logic",
        ),
        ("application/product/product_dto.rs", "// Product DTOs"),
        // Domain Layer
        (
            "domain/auth/user_repository_interface.rs",
            "pub trait UserRepositoryInterface {}",
        ),
        (
            "domain/auth/mfa_repository_interface.rs",
            "pub trait MfaRepositoryInterface {}",
        ),
        (
            "domain/auth/mfa_strategy_interface.rs",
            "pub trait MfaStrategyInterface {}",
        ),
        ("domain/user/user.rs", "#[derive(Debug)] pub struct User {}"),
        (
            "domain/user/user_repository_interface.rs",
            "pub trait UserRepositoryInterface {}",
        ),
        (
            "domain/user/user_service_interface.rs",
            "pub trait UserServiceInterface {}",
        ),
        (
            "domain/product/product.rs",
            "#[derive(Debug)] pub struct Product {}",
        ),
        (
            "domain/category/category.rs",
            "#[derive(Debug)] pub struct Category {}",
        ),
        // Infrastructure Layer
        (
            "infrastructure/database/database.rs",
            "// DB connection setup",
        ),
        (
            "infrastructure/database/generic_repository.rs",
            "pub struct GenericRepository;",
        ),
        ("infrastructure/user/user_dao.rs", "pub struct UserDAO;"),
        (
            "infrastructure/user/user_repository.rs",
            "pub struct UserRepository;",
        ),
        (
            "infrastructure/user/user_mapper.rs",
            "// Map between User entity and DAO",
        ),
        ("infrastructure/auth/auth_dao.rs", "pub struct AuthDAO;"),
        ("infrastructure/auth/auth_mapper.rs", "// Auth Mapper"),
        (
            "infrastructure/product/product_dao.rs",
            "pub struct ProductDAO;",
        ),
        (
            "infrastructure/product/product_mapper.rs",
            "// Product Mapper",
        ),
        ("infrastructure/mfa/mfa_dao.rs", "pub struct MfaDAO;"),
        (
            "infrastructure/mfa/mfa_strategy.rs",
            "pub struct MfaStrategy;",
        ),
        (
            "infrastructure/category/category_dao.rs",
            "pub struct CategoryDAO;",
        ),
        // Shared
        ("shared/utils.rs", "// Common utilities"),
        ("shared/logger.rs", "// Logger setup"),
        ("shared/exceptions.rs", "// Error types and handling"),
        // Migrations
        ("migrations/", "# migrations"),
    ];

    for (file, content) in files {
        create_file(&format!("{}/{}", root, file), content);
    }
}
