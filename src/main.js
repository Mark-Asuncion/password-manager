import { EVENTS, call } from "/scripts/helper.js";
const HREF = window.location["href"].split('/');
const CURR_PAGE = HREF[HREF.length-1];

console.log(window.location["href"], HREF, CURR_PAGE);

if (CURR_PAGE === "localhost" || CURR_PAGE === "") {
    call("skip_setup_page")
        .then((v) => {
            console.log(v);
            if (v === true) {
                window.open("/pages/home.html","_self")
            }
        })
}

window.addEventListener("DOMContentLoaded", () => {
    switch (CURR_PAGE) {
        case "localhost", "": {
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
            break;
        }
        case "home.html": {
            const table = document.querySelector("table");
            call("load_runtime")
                .then((v) => {
                    const res = JSON.parse(v);
                    if (res.error !== undefined) {
                        console.error(res.error);
                        return;
                    }
                    call("get_accounts")
                        .then((v) => {
                            let res = JSON.parse(v);
                            if (res.error !== undefined) {
                                console.error(res.error);
                            }
                            EVENTS.table.update(table, res);
                        })
                })
                .catch((e) => {
                    console.error(e);
                })
            document.querySelector("#save")
                .addEventListener("click",(e) => {
                    e.preventDefault();
                    console.log("save");
                    const spinner = document.querySelector(".lds-ripple-container");
                    spinner.style.display = "block";
                    const _done = () => {
                        setTimeout(function() {
                            spinner.style.display = "none";
                        }, 500);
                    }
                    EVENTS.save()
                        .then((v) => {
                            _done();
                        })
                        .catch((e) => {
                            _done();
                            console.error(e);
                        });
                })
            document.querySelector("#add")
                .addEventListener("click",(e) => {
                    e.preventDefault();
                    const add_in = document.querySelector(".new-account-bg");
                    const _done = () => {
                        add_in.style.display = "none";
                    }
                    document.querySelector("#img-btn-close").addEventListener("click", (e) => {
                        e.preventDefault();
                        _done();
                    });
                    add_in.addEventListener("click", (e) => {
                        e.preventDefault();
                        const class_names = e.target.className.split(' ');
                        if (e.target.tagName === "DIV" && class_names.find((item) => {
                            return item === "new-account-bg";
                        })) {
                            _done();
                        }
                    });
                    document.querySelector("#bt-new-account")
                        .addEventListener("click", (e) => {
                            e.preventDefault();
                            const username = document.querySelector("#new-account-username");
                            const link = document.querySelector("#new-account-link");
                            const password = document.querySelector("#new-account-password");
                            EVENTS.table.add(table, _done,username.value,link.value,password.value)
                            username.value = "";
                            link.value = "";
                            password.value = "";
                        });
                    add_in.style.display = "block";
                });

            document.querySelector("#more-option")
                .addEventListener("click",(e) => {
                    e.preventDefault()
                    let opt_list = document.querySelector(".more-option-list");
                    opt_list.addEventListener("mouseleave", (e) => {
                        e.preventDefault();
                        opt_list.style.display = "none";
                    })
                    opt_list.style.display = (opt_list.style.display === "none")? "block":"none";
                });
            document.querySelector("#opt-delete")
                .addEventListener("click", (e) => {
                    e.preventDefault();
                    const cancel = document.querySelector("#cancel")
                    cancel.style.display = "initial";
                    cancel.addEventListener("click", (e) => {
                        e.preventDefault();
                        cancel.style.display = "none";
                        const delete_btns = table.querySelectorAll("#btn-delete");
                        for (const [ _k, btn ] of delete_btns.entries()) {
                            btn.parentElement.removeChild(btn);
                        }
                    }, { once: true })
                    const callable = function() {
                        call("get_accounts")
                            .then((v) => {
                                let res = JSON.parse(v);
                                if (res.error !== undefined) {
                                    console.error(res.error);
                                    return;
                                }
                                EVENTS.table.clear(table);
                                EVENTS.table.update(table, res);
                                if (res.length !== 0) { EVENTS.delete_mode(table, callable); }
                                else { cancel.style.display = "none"; }
                            })
                    };
                    EVENTS.delete_mode(table, callable);
                });
            document.querySelector("#opt-append")
                .addEventListener("click", (e) => {
                    e.preventDefault();
                    EVENTS.table.append_select()
                        .then((selected) => {
                            call("append_account", { path: selected })
                                .then((v) => {
                                    let res = JSON.parse(v);
                                    if (res.error !== undefined) {
                                        console.error(res.error);
                                        return;
                                    }
                                    EVENTS.table.clear(table);
                                    EVENTS.table.update(table, res);
                                })
                                .catch((e) => {
                                    console.error(e);
                                })
                        })
                        .catch((e) => {
                            console.error(e)
                        })
                });

            break;
        }
        default:
            break;
    }
});
