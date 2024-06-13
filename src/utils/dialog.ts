import { open } from '@tauri-apps/api/dialog';

export async function mopen(): Promise<string> {
    const selected = await open({
        multiple: false,
        title: "Pick File to Append",
    });
    return selected as string;
}
