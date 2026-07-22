<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import TopBar from '$lib/components/Layout/TopBar.svelte';
	import AddConfigDialog from '$lib/components/Dialogs/AddConfigDialog.svelte';
	import SideBar from '$lib/components/Layout/SideBar.svelte';

	let { children } = $props();
	let showDialog = $state(false);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<div class="min-h-screen bg-neutral-950 text-white">
	<TopBar onAdd={() => (showDialog = true)} />
	<SideBar />
	<AddConfigDialog bind:open={showDialog} />

	<main class="pt-16 lg:ml-72">
		<div class="p-6">
			{@render children()}
		</div>
	</main>
</div>
