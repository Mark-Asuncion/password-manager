import { invoke } from "@tauri-apps/api/tauri";

export async function save() {
    try {
        await invoke("save");
        return;
    }
    catch (e) {
        // console.error(e);
        return  Promise.reject(new Error(e as string));
    }
}

export interface Account {
    username?:   string,
    link?:       string,
    password?:   string
}

export async function update_account(query: Account, update: Account) {
    try {
        await invoke("update_account", { query, update });
    }
    catch (e) {
        // console.error(e);
        return Promise.reject(new Error(e as string));
    }
}

export async function get_accounts(query: Account | undefined): Promise<Account[]> {
    try {
        const a = await invoke("get_accounts", { query }) as Account[];
        return a;
    }
    catch (e) {
        // console.error(e);
    }
    return [];
}

export async function add_account(v: Account) {
    try {
        await invoke("add_account", { v })
    }
    catch (e) {
        // console.error(e);
        return Promise.reject(new Error(e as string));
    }
}

export async function append_account(path: string) {
    try {
        await invoke("append_account", { pathFile: path });
    }
    catch (e) {
        // console.error(e);
        return  Promise.reject(new Error(e as string));
    }
}

export async function create_archive_tar(path: string): Promise<string> {
    try {
        return await invoke("create_archive_tar", { path });
    }
    catch (e) {
        // console.error(e);
        return Promise.reject(new Error(e as string));
    }
}

export async function delete_account(query: Account) {
    try {
        invoke("delete_account", { query });
    }
    catch (e) {
        // console.error(e);
    }
}
