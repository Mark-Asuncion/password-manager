const { invoke } = window.__TAURI__.tauri;
const { open } = window.__TAURI__.dialog;

// invoke("g_contents", {})
// [ Objects ]
// format:
// {
//  username: "",
//  link: "",
//  password: "",
// }

/**
 * @param {Number} row
 * @param {String} username
 * @param {String} link
 * @param {String} password
 * @returns {HTMLElement}
 */
function _create_row(row, username, link, password) {
    /**
     * @param {Number} row
     * @param {String} key
     * @param {String} val
     * @returns {HTMLElement}
     */
    function _create_cell(row, key, val) {
        const cell = document.createElement("td");
        const input = document.createElement("input");
        input.type = (key === "password")? "password": "text";
        input.value = val;
        input.disabled = key === "password";

        input.addEventListener("focusout", (e) => {
            // if (e.key !== "Enter") { return; }
            e.preventDefault();
            call("update_account", {
                row: row,
                key: key,
                val: input.value
            })
                .catch((e) => {
                    console.error(e);
                });
        });

        cell.appendChild(input);
        if (key === "password") {
            const btn_edit = document.createElement("img");
            const btn_toggle = document.createElement("img");
            const btn_copy = document.createElement("img");

            btn_edit.setAttribute("class", "img-btn");
            btn_toggle.setAttribute("class", "img-btn");
            btn_copy.setAttribute("class", "img-btn");
            //
            // TODO change icon
            btn_edit.setAttribute("src", "/assets/edit_FILL0_wght400_GRAD0_opsz24.svg");
            btn_toggle.setAttribute("src", "/assets/visibility_FILL0_wght400_GRAD0_opsz24.svg");
            btn_copy.setAttribute("src", "/assets/content_copy_FILL0_wght400_GRAD0_opsz24.svg");

            btn_edit.addEventListener("click", (e) => {
                e.preventDefault();
                input.disabled = !input.disabled;
            });
            btn_toggle.addEventListener("click", (e) => {
                e.preventDefault();
                input.type = (input.type === "text")? "password": "text";
                let img_src = ( input.type === "text" )? "/assets/visibility_off_FILL0_wght400_GRAD0_opsz24.svg":"/assets/visibility_FILL0_wght400_GRAD0_opsz24.svg";
                btn_toggle.setAttribute("src", img_src);
            });
            btn_copy.addEventListener("click", (e) => {
                e.preventDefault();
                navigator.clipboard.writeText(input.value);
            });

            cell.appendChild(btn_edit);
            cell.appendChild(btn_toggle);
            cell.appendChild(btn_copy);
        }
        return cell
    }
    const trow = document.createElement("tr");
    trow.appendChild(_create_cell(row,"username", username));
    trow.appendChild(_create_cell(row, "link", link));
    trow.appendChild(_create_cell(row,"password", password));
    return trow;
}

export async function call(fn_name, args = {}) {
    return await invoke(fn_name, args);
}

export const EVENTS = {
    create: () => {
        call("create_key")
            .then((v) => {
                console.log(v);
            })
            .catch((e) => {
                console.error(e);
            });
    },
    load: () => {
        call("load_key")
            .then((v) => {
                console.log(v);
            })
            .catch((e) => {
                console.error(e);
            });
    },
    save: () => {
        return call("save_accounts");
    },
    table: {
        /**
         * @param { HTMLElement } root
         * @param { [Object] } accounts
         */
        update: (root, accounts) => {
            for (let row=0;row<accounts.length;row++) {
                root.appendChild(
                    _create_row(
                        row,
                        accounts[row].username,
                        accounts[row].link,
                        accounts[row].password
                    )
                );
            }
        },
        /**
         * @param { HTMLElement } root
         */
        add: (root, hide, username, link, password) => {
            call("add_account", {
                username: username,
                link: link,
                password: password
            })
            .then((v) => {
                    const res = JSON.parse(String(v));
                    if (res.error !== undefined) {
                        console.error(res.error);
                        return;
                    }
                    else {
                        root.appendChild(
                            _create_row(
                                root.childElementCount,
                                res.username,
                                res.link,
                                res.password
                            )
                        );
                    }
                    hide();
                })
            .catch((e) => {
                    hide();
                    console.error(e);
                })
        },
        /**
         * @param {HTMLTableElement} root
         */
        clear: (root) => {
            const childs = root.querySelectorAll("tr");
            for (let i=1;i<childs.length;i++) {
                const child = childs[i];
                root.removeChild(child);
            }
        }
    },
    /**
    * @param {HTMLTableElement} root
    * @param {() -> void} callable
    */
    delete_mode: (root, callable) => {
        function _create_delete_btn(row) {
            const img = document.createElement("img");
            img.setAttribute("class", "img-btn");
            img.id = "btn-delete";
            img.src = "/assets/delete_FILL0_wght400_GRAD0_opsz24.svg";
            img.addEventListener("click", (e) => {
                e.preventDefault();
                call("remove_account", {row:row});
                callable();
            });
            return img;
        }
        const trs = root.querySelectorAll("tr");
        for (let i=1;i<trs.length;i++) {
            if (trs[i].firstChild.firstChild.tagName === "IMG") {
                const img = trs[i].firstChild.querySelector("img");
                trs[i].firstChild.removeChild(img);
            }
            else {
                trs[i].firstChild.prepend(_create_delete_btn(i-1));
            }
        }
    },
};

EVENTS.table.append_select = async() => {
    const selected = await open({
        multiple: false,
        title: "Select a Account File (csv)",
        filters: [{
            name: "CSV document",
            extensions: ["csv", "*"]
        }]
    });
    return selected;
}
