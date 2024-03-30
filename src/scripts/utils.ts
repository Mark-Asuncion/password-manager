import { open, save } from "@tauri-apps/api/dialog";
import { Account, add_account, remove_account, update_account } from "./binder";
import imEditOn from "/assets/edit_white.svg";
import imEditOff from "/assets/edit_off_white.svg";
import imVisOn from "/assets/visibility_on_white.svg";
import imCopy from "/assets/content_copy_white.svg";
import imVisOff from "/assets/visibility_off_white.svg";
import imDel from "/assets/delete_white.svg";

function _createActionsCell(inputPassword: HTMLInputElement, id: number): HTMLTableCellElement {
    const cell = document.createElement("td");
    const groupInput = document.createElement("div");
    groupInput.setAttribute("class", "input-group justify-content-center");
    const btn_edit = document.createElement("button");
    const btn_toggle = document.createElement("button");
    const btn_copy = document.createElement("button");
    const btn_del = document.createElement("button");

    const btnClass = "btn btn-outline-secondary";
    btn_edit.setAttribute("class", btnClass);
    btn_toggle.setAttribute("class", btnClass);
    btn_copy.setAttribute("class", btnClass);
    btn_del.setAttribute("class", btnClass);

    btn_edit.role = "group";
    btn_toggle.role = "group";
    btn_copy.role = "group";
    btn_del.role = "group";

    const imgEdit = document.createElement("img")! as HTMLImageElement;
    const imgToggle = document.createElement("img")! as HTMLImageElement;
    const imgCopy = document.createElement("img")! as HTMLImageElement;
    const imgDel = document.createElement("img")! as HTMLImageElement;
    imgEdit.setAttribute("src", imEditOn);
    imgToggle.setAttribute("src", imVisOn);
    imgCopy.setAttribute("src", imCopy);
    imgDel.setAttribute("src", imDel);

    btn_edit.appendChild(imgEdit);
    btn_toggle.appendChild(imgToggle);
    btn_copy.appendChild(imgCopy);
    btn_del.appendChild(imgDel);

    btn_edit.addEventListener("click", (e) => {
        e.preventDefault();
        inputPassword.disabled = !inputPassword.disabled;
        imgEdit.setAttribute("src", (inputPassword.disabled)? imEditOn:imEditOff);
    });
    btn_toggle.addEventListener("click", (e) => {
        e.preventDefault();
        inputPassword.type = (inputPassword.type === "text")? "password": "text";
        let img_src = ( inputPassword.type === "text" )? imVisOff:imVisOn;
        imgToggle.setAttribute("src", img_src);
    });
    btn_copy.addEventListener("click", (e) => {
        e.preventDefault();
        navigator.clipboard.writeText(inputPassword.value);
        cell.dispatchEvent(new CustomEvent("notify", {
            bubbles: true,
            detail: { text: "Content Copied to Clipboard" }
        }));
    });
    btn_del.addEventListener("click", async (e) => {
        e.preventDefault();
        await remove_account(id);
        cell.dispatchEvent(new Event("tablereload", { bubbles: true }));
    });

    groupInput.appendChild(btn_edit);
    groupInput.appendChild(btn_toggle);
    groupInput.appendChild(btn_copy);
    groupInput.appendChild(btn_del);
    cell.appendChild(groupInput);
    return cell;
}

function _createRow(account: Account): HTMLTableRowElement {
    function _createCell(id: number, key: string, value: string): HTMLTableCellElement {
        const cell = document.createElement("td");
        const input = document.createElement("input");
        input.setAttribute("class", "form-control min-w-auto bg-dark text-light");
        input.type = (key !== "password")? "text":key;
        input.value = value;
        input.addEventListener("focusout", (e: FocusEvent) => {
            // if (e.key !== "Enter") { return; }
            e.preventDefault();
            update_account(
                id,
                key,
                input.value
            );
        });
        input.disabled = key === "password";
        cell.appendChild(input);
        return cell;
    }
    const trow = document.createElement("tr");
    trow.appendChild(_createCell(account.id!,"username", account.username));
    trow.appendChild(_createCell(account.id!, "link", account.link));
    const inputPass = _createCell(account.id!,"password", account.password);
    trow.appendChild(inputPass);
    trow.appendChild(_createActionsCell(inputPass.firstChild! as HTMLInputElement, account.id!));
    return trow;
}

export const Table = {
    add: async function(table: HTMLTableElement, account: Account) {
        const acc = await add_account(
            account.username,
            account.link,
            account.password
        );
        if (acc === null) return;
        const tbody = table.querySelector("tbody")!;
        tbody.appendChild(_createRow(acc));
    },
    render: function(table: HTMLTableElement, res: [Account]) {
        const tbody = table.querySelector("tbody")!;
        for (let id=0;id<res.length;id++) {
            tbody.appendChild(_createRow(res[id]));
        }
    },
    reload: function(table: HTMLTableElement, res: [Account]) {
        const tbody = table.querySelector("tbody")!;
        tbody.replaceChildren();
        this.render(table, res);
    },
    appendSelect: async function(): Promise<string | string[] | null> {
        const selected = await open({
            multiple: false,
            title: "Select a Account File (csv)",
            filters: [{
                name: "CSV document",
                extensions: ["csv", "*"]
            }]
        });
        return selected;
    },
}

export async function exportDialog(): Promise<string | null> {
    const selected = await save({
        title: "Choose a destination",
        defaultPath: "./accounts"
    });
    return selected;
}

export function createToast(text: string): HTMLDivElement {
    const template = document.createElement("template");
    const toast = `<div class="toast bg-dark show" role="alert" aria-live="assertive" aria-atomic="true">
            <div class="d-flex">
            <img src="${imCopy}" class="rounded m-auto" alt="...">
            <div class="toast-body"> ${text} </div>
            <button type="button" class="btn-close btn-close-white m-auto" data-bs-dismiss="toast" aria-label="Close"></button>
            </div>
            </div>`;
    template.innerHTML = toast;
    return template.content.children[0] as HTMLDivElement;
}

export async function setWorkspace(): Promise<string | null> {
    const selected = await open({
        title: "Set a workspace folder",
        multiple: false,
        directory: true,
    });
    return selected as string | null;
}
