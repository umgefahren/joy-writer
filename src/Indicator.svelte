<script lang="ts">
    import { IndicatorMessage } from "./indicating";

    export let notifyStatus: IndicatorMessage

    let active = false
    let content: string = "Saved..."

    $: if (notifyStatus === IndicatorMessage.Save) {
        active = true;
        content = "Saved..."
        setTimeout(() => {
            active = false
            notifyStatus = IndicatorMessage.None
        }, 1000)
    }
</script>

<main>
    {#if active}
        <p>{ content }</p>
    {:else} 
        <p class="invisible">{ content }</p>
    {/if}
</main>

<style>
    .invisible {
        opacity: 0%;
    }
</style>
