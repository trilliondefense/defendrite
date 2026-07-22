<script lang="ts">
	import * as yaml from 'js-yaml';
	import CodeEditor from '../CodeEditor.svelte';

	let { open = $bindable() } = $props();

	let server = $state('');
	let url = $state('');
	let apiKey = $state('');
	let agent = $state('');

	let generated = $state('');

	let showEditor = $state(false);

	function createYaml() {
		generated = yaml.dump({
			mcp: {
				server,
				url,
				apiKey,
				aiAgent: agent
			}
		});

		showEditor = true;
	}
</script>

{#if open}
	<div class="fixed inset-0 flex items-center justify-center bg-black/50">
		<div
			class="
w-full max-w-lg
space-y-4
border-4 border-black
bg-neutral-100
p-6
shadow-[8px_8px_0_#000]"
		>
			<h2 class="text-2xl font-black">Add MCP Configuration</h2>

			<input bind:value={server} placeholder="Server" />

			<input bind:value={url} placeholder="URL" />

			<input bind:value={apiKey} placeholder="API Key" />

			<input bind:value={agent} placeholder="AI Agent" />

			<button
				onclick={createYaml}
				class="
border-4
border-black bg-lime-300
px-5 py-2
shadow-[4px_4px_0_#000]
"
			>
				Generate Config
			</button>
		</div>
	</div>
{/if}

{#if showEditor}
	<CodeEditor code={generated} />
{/if}
