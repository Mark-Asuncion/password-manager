import { invoke } from "@tauri-apps/api/tauri";
import { append_account, get_accounts, save_accounts } from "./binder";
import { Table } from "./utils";

window.addEventListener("DOMContentLoaded", () => {
    const table = document.querySelector("table")!;
    table.addEventListener("tablereload", async () => {
        const acc = await get_accounts();
        if (acc != null)
            Table.reload(table, acc);
    });

    function saveAccounts(e: Event) {
        e.preventDefault();
        const spinner = document.querySelector(".lds-ripple-container") as HTMLDivElement;
        spinner.style.display = "block";
        const _done = () => {
            setTimeout(function() {
                spinner.style.display = "none";
            }, 500);
        }
        save_accounts()
            .then(() => {
                _done()
            })
            .catch(() => {
                _done()
            });
    }

    function addAccount(e: Event) {
        e.preventDefault();
        document.querySelector("#bt-new-account")!
            .addEventListener("click", (e) => {
                e.preventDefault();
                const username: HTMLInputElement = document.querySelector("#new-account-username")!;
                const link: HTMLInputElement = document.querySelector("#new-account-link")!;
                const password: HTMLInputElement = document.querySelector("#new-account-password")!;
                Table.add(table, {
                    username: username.value,
                    link: link.value,
                    password: password.value
                })
                username.value = "";
                link.value = "";
                password.value = "";
            });
    }

    async function load_runtime() {
        let v: string = await invoke("load_runtime") as string;
        const res = JSON.parse(v);
        if (res.error !== undefined) {
            console.error(res.error);
            ;
        }
        const acc = await get_accounts();
        if (acc != null)
        Table.render(table, acc);
    }

    async function appendAccounts(e: Event) {
        e.preventDefault();
        let selected = await Table.appendSelect();
        if (selected === null) return;
        if (typeof selected !== "string") {
            selected.forEach((v) => {
                append_account(v);
            });
        }
        append_account(selected as string);
        table.dispatchEvent(new Event("tablereload"));
    }

    load_runtime();

    const btnSave = document.querySelector("#save")!;
    const btnAdd = document.querySelector("#add")!;
    const btnAppend = document.querySelector("#opt-append")!;
    btnAdd.addEventListener("click", addAccount);
    btnSave.addEventListener("click", saveAccounts)
    btnAppend.addEventListener("click", appendAccounts);
});
