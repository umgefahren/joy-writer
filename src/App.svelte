<script lang="ts">
	import Editor from "./Editor.svelte";
	import View from "./View.svelte";
	import Opener from "./Opener.svelte";
	import Saver from "./Saver.svelte";
	import { globalShortcut } from "@tauri-apps/api";
	import { saveFile } from "./saver";
	import Indicator from "./Indicator.svelte";
	import { IndicatorMessage } from "./indicating";
	import { openedFilePathStore } from "./stores";

	let openedFilePath: string = "";

	openedFilePathStore.subscribe((c) => { openedFilePath = c})

	let fileContent: string;

	let textContent: string;

	$: textContent = fileContent;

	let cleandHtmlArray: string[] = [];

	let status: IndicatorMessage = IndicatorMessage.None;

	let concatedHtml: string;
	$: concatedHtml = cleandHtmlArray.join("\n");

	globalShortcut.isRegistered("Cmd+S").then((registered) => {
		if (!registered) {
			globalShortcut.register("Cmd+S", async () => {
				console.log("We are here");
				if (openedFilePath !== "") {
					console.log("This is registered");
					status = IndicatorMessage.Save;
					await saveFile(openedFilePath, concatedHtml);
				}
			});
		}
	});
</script>

<main>
	<div class="indicator">
		<Indicator bind:notifyStatus={status} />
	</div>

	<div class="open-button">
		<Opener bind:fileContent bind:filePath={openedFilePath} />
	</div>

	<div class="save-button">
		<Saver bind:fileContent bind:status />
	</div>

	<div class="one">
		<Editor bind:cleanedHtml={cleandHtmlArray} bind:textContent />
	</div>

	<div class="two">
		<View bind:concatedHtml />
	</div>
</main>

<style>
	main {
		display: grid;
		grid-auto-columns: minmax(0, 1fr);
		grid-auto-flow: column;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	.open-button {
		grid-column: 1;
		grid-row: 2;
	}

	.save-button {
		grid-column: 2;
		grid-row: 2;
	}

	.one {
		margin-right: 2em;
		grid-column: 1;
		grid-row: 3;
	}

	.two {
		margin-right: 2em;
		grid-column: 2;
		grid-row: 3;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
