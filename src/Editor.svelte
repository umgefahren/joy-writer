<script lang="ts">
    import { globalShortcut } from "@tauri-apps/api";

    import { cleanHtml } from "../src/cleaner";
import TextAreaAutoResize from "./TextAreaAutoResize.svelte";

    let htmlContent: string = "";
    export const textContent: string = "Enter your content here";

    $: htmlContent = textContent;

    export let cleanedHtml: string[];

    // $: cleanedHtml = cleanHtml(htmlContent);
    $: cleanedHtml = htmlContent.split("\n");

    let previousContents: string[] = [];

    setInterval(() => {
        previousContents.push(htmlContent);

        if (previousContents.length > 100) previousContents.shift();
    }, 2000);

    globalShortcut.isRegistered("Cmd+Z").then((registered) => {
        if (!registered) {
            globalShortcut.register("Cmd+Z", () => {
                const newContent = previousContents.pop();
                previousContents.push(newContent);
                htmlContent = newContent;
            });
        }
    });


</script>

<div class="main">
    <TextAreaAutoResize bind:value={htmlContent} maxRows=100000000000 />
</div>

<style>
    .main {
        text-align: left;
        padding: 0.5em;
        border-width: 1px;
        border-style: solid;
        border-radius: 10px;
    }

    .writearea {
        width: 100%;
        height: 100%;
        margin-bottom: 0%;
        background-color: black;
        border: none;
    }

    @media (prefers-color-scheme: dark) {
        .writearea {
            background-color: black;
            color: lightgrey;
        }
    }

    @media (prefers-color-scheme: light) {
        .writearea {
            background-color: white;
            color: black;
        }
    }
</style>
