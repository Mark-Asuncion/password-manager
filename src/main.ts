import { skip_setup_page, create_key, load_key } from "./scripts/binder.ts";

function openHome() {
    window.open("./home.html","_self")
}

window.addEventListener("DOMContentLoaded", () => {
    skip_setup_page()
        .then((e: boolean) => {
            if (e) {
                openHome();
            }
        });
    document.querySelector("#create")!
        .addEventListener("click", (e) => {
            e.preventDefault();
            create_key()
                .then(() => {
                    openHome();
                });
        })
    document.querySelector("#load")!
        .addEventListener("click", (e) => {
            e.preventDefault();
            load_key();
        })
});
