use actix_web_static_files::npm_resource_dir;

fn main() {
    npm_resource_dir("./andaman-frontend/src").build().unwrap();
}
