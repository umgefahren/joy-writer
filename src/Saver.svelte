<script lang="ts">
    export let fileContent: string;
    export let status: IndicatorMessage;

    import { dialog, fs } from "@tauri-apps/api";
    import { IndicatorMessage } from "./indicating";
    import { saveFile } from "./saver";

    async function saveHandler() {
        let filePath = await dialog.save();

        if (filePath === null) throw "No";

        await saveFile(filePath, fileContent);
        status = IndicatorMessage.Save
    }
</script>

<main>
    <button on:click={saveHandler}>Save File</button>
</main>
