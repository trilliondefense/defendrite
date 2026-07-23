<script lang="ts">
	import { LayoutDashboard, Settings, LifeBuoy, Menu, X } from '@lucide/svelte';
	import { page } from '$app/state';

	let open = $state(false);

	const items = [
		{
			name: 'Dashboard',
			icon: LayoutDashboard,
			href: '/dashboard'
		},
		{
			name: 'Settings',
			icon: Settings,
			href: '/settings'
		},
		{
			name: 'Reports',
			icon: LifeBuoy,
			href: '/reports'
		}
	];

	function toggleMenu() {
		open = !open;
	}
</script>

<!-- Mobile Hamburger -->
<div class="lg:hidden">
	<button
		onclick={toggleMenu}
		class="
		fixed
		top-3
		left-3
		z-[60]
		flex
		h-11
		w-11
		items-center
		justify-center
		border-4
		border-black
		bg-cyan-300
		text-black
		shadow-[4px_4px_0_#000]
		transition
		hover:translate-x-1
		hover:translate-y-1
		hover:shadow-none
		"
	>
		{#if open}
			<X size={22} />
		{:else}
			<Menu size={22} />
		{/if}
	</button>

	{#if open}
		<!-- Overlay -->
		<div
			class="
			fixed
			inset-0
			z-40
			bg-black/50
			"
			onclick={() => (open = false)}
		></div>

		<!-- Mobile Sidebar -->
		<aside
			class="
			fixed
			top-16
			left-0
			z-50
			h-[calc(100vh-4rem)]
			w-72
			border-r-4
			border-black
			bg-neutral-950
			p-5
			"
		>
			<nav class="space-y-4">
				{#each items as item}
					<a
						href={item.href}
						onclick={() => (open = false)}
						class="
						flex
						items-center
						gap-3
						border-4
						border-black
						bg-white
						px-4
						py-3
						font-black
						text-black
						shadow-[4px_4px_0_#000]
						transition
						hover:translate-x-1
						hover:translate-y-1
						hover:shadow-none
						"
					>
						<svelte:component this={item.icon} size={20} />

						{item.name}
					</a>
				{/each}
			</nav>
		</aside>
	{/if}
</div>

<!-- Desktop Sidebar -->

<aside
	class="
	fixed
	top-16
	left-0
	hidden
	h-[calc(100vh-4rem)]
	w-72
	border-r-4
	border-black
	bg-neutral-950
	p-5
	lg:block
	"
>
	<nav class="space-y-4">
		{#each items as item}
			<a
				href={item.href}
				class="
				flex
				items-center
				gap-3
				border-4
				border-black
				bg-white
				px-4
				py-3
				font-black
				text-black
				shadow-[4px_4px_0_#000]
				transition
				hover:translate-x-1
				hover:translate-y-1
				hover:shadow-none
				"
			>
				<svelte:component this={item.icon} size={20} />

				{item.name}
			</a>
		{/each}
	</nav>
</aside>
