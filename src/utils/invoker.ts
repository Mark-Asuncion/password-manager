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

export async function update(stringIndex: [string, string] | undefined, numIndex: number | undefined, ): Promise<void | Error> {
    try {
        invoke("update", { nindex: numIndex, sindex: stringIndex });
        return;
    }
    catch (e) {
        console.error(e);
        return new Error(e as string);
    }
}
