import { invoke } from "@tauri-apps/api/tauri";
import { append_account, get_accounts, save_accounts, search } from "./binder";
import { Table } from "./utils";

window.addEventListener("DOMContentLoaded", () => {
    const table = document.querySelector("table")!;

    async function trySearch(value: string) {
        if (value.length === 0) {
            const searchSc = document.querySelector("#search-sc")!;
            const searchFocused = document.querySelector("#search-focused")!;
            searchSc.classList.remove("d-none");
            searchFocused.classList.replace("d-flex", "d-none");
            table.dispatchEvent(new Event("tablereload"));
            return;
        }
        const res = await search(value);
        table.dispatchEvent(new CustomEvent("tablesearch", {
            detail: { results: res }
        }));
    }

    table.addEventListener("tablereload", async () => {
        const searchBarV = ( document.querySelector("#search-bar")! as HTMLInputElement).value;
        if ( searchBarV.length === 0 ) {
            const acc = await get_accounts();
            if (acc != null)
            Table.reload(table, acc);
        }
        else {
            trySearch(searchBarV);
        }
    });

    table.addEventListener("tablesearch", ((e: CustomEvent) => {
        if (e.detail.results === null) return;
        const searchSc = document.querySelector("#search-sc")!;
        const searchFocused = document.querySelector("#search-focused")!;
        searchSc.classList.add("d-none");
        searchFocused.classList.replace("d-none", "d-flex");
        searchFocused.querySelector("#search-results")!.textContent
            = `${ e.detail.results.length } result/s`;
        Table.reload(table, e.detail.results);
    } ) as EventListener);

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

    async function handleSearch(e: Event) {
        const inp = (e.currentTarget as HTMLInputElement).value;
        trySearch(inp);
    }

    function handleSearchClose(e: Event) {
        e.preventDefault();
        const searchBar = (document.querySelector("#search-bar")! as HTMLInputElement);
        searchBar.value = "";
        const searchSc = document.querySelector("#search-sc")!;
        const searchFocused = document.querySelector("#search-focused")!;
        searchSc.classList.remove("d-none");
        searchFocused.classList.replace("d-flex", "d-none");
        table.dispatchEvent(new Event("tablereload"));
    }

    load_runtime();

    const btnSave = document.querySelector("#save")!;
    const btnAdd = document.querySelector("#add")!;
    const btnAppend = document.querySelector("#opt-append")!;
    const searchBar = document.querySelector("#search-bar")!;
    const searchFocused = document.querySelector("#search-focused")!;
    const searchBtnClose = searchFocused.querySelector("#btn-close")!;
    btnAdd.addEventListener("click", addAccount);
    btnSave.addEventListener("click", saveAccounts)
    btnAppend.addEventListener("click", appendAccounts);
    searchBar.addEventListener("keyup", handleSearch);
    searchBtnClose.addEventListener("click", handleSearchClose);

    document.addEventListener("keydown", (e: KeyboardEvent) => {
        const target = (e.target as Element).tagName;
        if (target === "INPUT") return;
        e.preventDefault();
        if (e.key === '/')
            ( searchBar as HTMLInputElement ).focus();
    });
});
