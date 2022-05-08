<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"

    export let concatedHtml: string

    let markdown: string = "";
    $: if(concatedHtml) {
        invoke('markdown_to_html', { markdown: concatedHtml }).then((content) => {
            markdown = content as string;
        }) 
    }
</script>


<div class="total">
    <article class="markdown-body">
        {@html markdown}
    </article>
</div>

<style>
    .total {
        line-break: normal;
        overflow: hidden;
        padding: 0.5em;
        border-width: 1px;
        border-style: solid;
        border-radius: 10px;
    }
</style>