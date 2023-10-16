import {open} from "@tauri-apps/api/dialog";

async function pick_folder(): Promise<string | undefined> {
    const selected = await open({
        directory: true,
    });

    if (typeof(selected) == "string") {
        return selected;
    }
}

export {pick_folder};