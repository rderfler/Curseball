// @ts-ignore
import module from './../crate/Cargo.toml';

start_page();
console.log("Curse applied");

export function start_page() {
    module.start();
    module.update_text();
}