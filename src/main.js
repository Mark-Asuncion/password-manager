import { EVENTS, call } from "/scripts/helper.js";
call("skip_setup_page")
    .then((v) => {
        console.log(v);
        if (v === true) {
            window.open("/pages/home.html","_self")
        }
    })

window.addEventListener("DOMContentLoaded", () => {
    document.querySelector("#create")
        .addEventListener("click", (e) => {
            e.preventDefault();
            EVENTS.create();
            window.open("/pages/home.html","_self")
        })
    document.querySelector("#load")
        .addEventListener("click", (e) => {
            e.preventDefault();
            EVENTS.load();
        })
});
