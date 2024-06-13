import { invoke } from "@tauri-apps/api/tauri";

export async function save(): Promise<void | Error> {
    try {
        invoke("save");
        return;
    }
    catch (e) {
        console.error(e);
        return new Error(e as string);
    }
}

export interface Account {
    username?:   string,
    link?:       string,
    password?:   string
}

export async function update_account(query: Account, update: Account): Promise<void | Error> {
    try {
        invoke("update_account", { query, update });
        return;
    }
    catch (e) {
        console.error(e);
        return new Error(e as string);
    }
}

export async function get_accounts(query: Account | undefined): Promise<Account[]> {
    try {
        const a = await invoke("get_accounts", { query }) as Account[];
        return a;
    }
    catch (e) {
        console.error(e);
    }
    return [];
}

export async function add_account(v: Account) {
    try {
        invoke("add_account", { v })
    }
    catch (e) {
        console.error(e);
    }
}

export async function append_account(path: string): Promise<Error | void> {
    try {
        invoke("append_account", { pathFile: path });
        return;
    }
    catch (e) {
        console.error(e);
        return new Error(e as string);
    }
}

export async function mexport(): Promise<Error | void> {
    try {
        invoke("export");
        return;
    }
    catch (e) {
        console.error(e);
        return new Error(e as string);
    }
}

export async function delete_account(query: Account) {
    try {
        invoke("delete_account", { query });
    }
    catch (e) {
        console.error(e);
    }
}
