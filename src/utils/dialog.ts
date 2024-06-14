import { open, save } from '@tauri-apps/api/dialog';

export async function mopen(): Promise<string> {
    const selected = await open({
        multiple: false,
        title: "Pick File to Append",
    });
    return selected as string;
}

export async function exportDialog(): Promise<string | null> {
    const selected = await save({
        title: "Choose a destination",
        defaultPath: "accounts",
        filters: [{
            name: 'Archive',
            extensions: ['tar']
        }]
    });
    return selected;
}
