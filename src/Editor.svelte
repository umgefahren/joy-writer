<script lang="ts">
import { globalShortcut } from "@tauri-apps/api";

    import { cleanHtml } from "../src/cleaner";

    let htmlContent: string;
    export let textContent: string = "Enter your content here";

    $: htmlContent = textContent.split("\n").map((line) => `<div>${line}</div>`).join("\n")

    export let cleanedHtml: string[];

    $: cleanedHtml = cleanHtml(htmlContent);

    let previousContents: string[] = []

    setInterval(() => {
        console.log("New element")

        previousContents.push(htmlContent)


        if (previousContents.length > 100) previousContents.shift()
    }, 2000)

    globalShortcut.isRegistered("Cmd+Z").then((registered) => {
        console.log("Trying to register")
        if (!registered) {
            globalShortcut.register("Cmd+Z", () => {
                console.log("back")
                const newContent = previousContents.pop()
                previousContents.push(newContent)
                htmlContent = newContent
            })
        }
    })
</script>

<div class="main">
    <div contenteditable="true" bind:innerHTML={ htmlContent } spellcheck="false"></div>
</div>

<style>
    .main {
        text-align: left;
        padding: 0.5em;
        border-width: 1px;
        border-style: solid;
        border-radius: 10px;
    }
</style>