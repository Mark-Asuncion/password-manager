import { invoke } from "@tauri-apps/api/tauri";

export interface Account {
    id?: number,
    username: string,
    link: string,
    password: string
    error?: string,
}

export async function create_key() {
    try {
        await invoke("create_key");
    }
    catch (e) {
        console.error(e);
    }
}

export async function load_key() {
    try {
        await invoke("load_key");
    }
    catch (e) {
        console.error(e);
    }
}

export async function skip_setup_page(): Promise<boolean> {
    try {
        let res: boolean = await invoke("skip_setup_page");
        return res;
    }
    catch (e) {
        console.error(e);
        return false;
    }
}

export async function get_accounts(): Promise<[Account] | null> {
    try {
        const res: string = await invoke("get_accounts");
        return JSON.parse(res);
    }
    catch (e) {
        console.error(e);
        return null
    }
}

export async function update_account(id: number, key: string, val: string) {
    try {
        await invoke("update_account", {
            id: id,
            key: key,
            val: val
        });
    }
    catch (e) {
        console.error(e);
    }
}

export async function save_accounts() {
    try {
        await invoke("save_accounts");
    }
    catch (e) {
        console.error(e);
    }
}

export async function add_account(username: string,
    link: string,
    password: string): Promise<Account | null> {
    try {
        const res = await invoke("add_account", {
            username: username,
            link: link,
            password: password
        }) as string;
        const acc = JSON.parse(res) as Account;
        if (acc.error !== undefined) return null;
        return acc
    }
    catch (e) {
        console.error(e);
        return null;
    }
    return null;
}

export async function remove_account(id: number) {
    try {
        await invoke("remove_account", { id: id });
    }
    catch (e) {
        console.error(e);
    }
}

export async function append_account(path: string) {
    try {
        await invoke("append_account", { path: path });
    }
    catch (e) {
        console.error(e);
        return [];
    }
}
