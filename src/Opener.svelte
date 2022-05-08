<script lang="ts">
    import { dialog, fs } from "@tauri-apps/api";
    import type { OpenDialogOptions } from "@tauri-apps/api/dialog";
import { openedFileContentStore, openedFilePathStore } from "./stores";


    
    export let fileContent: string;
    export let filePath: string;

    async function openHandler() {
        const openDialogOptions: OpenDialogOptions = {
            directory: false,
            multiple: false
        };
        filePath = await dialog.open(openDialogOptions) as string 
        openedFilePathStore.set(filePath)

        if (filePath === null) {
            throw "No"
        }

        fileContent = await fs.readTextFile(filePath as string, null)
        openedFileContentStore.set(fileContent)
    }
</script>

<main>
    <button on:click={openHandler}>Open File</button>
</main>