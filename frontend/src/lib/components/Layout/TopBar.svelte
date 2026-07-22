<script lang="ts">
	import { Bell, Plus, Search, Bot } from '@lucide/svelte';

	let data: { onAdd: () => {} } = $props();
</script>

<nav
	class="
	fixed
	top-0
	right-0
	left-0
	z-50
	h-16
	border-b-4
	border-black
	bg-neutral-950
	shadow-[0_6px_0_#000]
	"
>
	<div
		class="
		flex
		h-full
		items-center
		gap-4
		px-4
		"
	>
		<!-- Mobile space for hamburger -->
		<div class="w-12 lg:hidden"></div>

		<!-- Logo -->

		<div
			class="
			flex
			h-11
			w-11
			shrink-0
			items-center
			justify-center
			border-4
			border-black
			bg-lime-300
			shadow-[4px_4px_0_#000]
			transition
			hover:translate-x-1
			hover:translate-y-1
			hover:shadow-none
			"
		>
			<Bot size={22} class="text-black" />
		</div>

		<!-- Title -->

		<div class="hidden md:block">
			<h1
				class="
				text-xl
				font-black
				text-white
				"
			>
				MCP Dashboard
			</h1>

			<p
				class="
				text-xs
				text-neutral-400
				"
			>
				Manage Servers & AI Agents
			</p>
		</div>

		<div class="flex-1"></div>

		<!-- Search -->

		<div
			class="
			relative
			hidden
			w-full
			max-w-sm
			lg:block
			"
		>
			<Search
				size={18}
				class="
				absolute
				top-1/2
				left-3
				-translate-y-1/2
				text-black
				"
			/>

			<input
				placeholder="Search..."
				class="
				w-full
				border-4
				border-black
				bg-white
				py-2
				pr-4
				pl-10
				font-semibold
				text-black
				shadow-[4px_4px_0_#000]
				transition
				outline-none
				focus:translate-x-1
				focus:translate-y-1
				focus:shadow-none
				"
			/>
		</div>

		<!-- Notification -->

		<button
			class="
			border-4
			border-black
			bg-yellow-300
			p-2
			shadow-[4px_4px_0_#000]
			transition
			hover:translate-x-1
			hover:translate-y-1
			hover:shadow-none
			"
		>
			<Bell size={20} class="text-black" />
		</button>

		<!-- Add -->

		<button
			onclick={data.onAdd}
			class="
			flex
			items-center
			gap-2
			border-4
			border-black
			bg-cyan-300
			px-4
			py-2
			font-black
			text-black
			shadow-[4px_4px_0_#000]
			transition
			hover:translate-x-1
			hover:translate-y-1
			hover:shadow-none
			"
		>
			<Plus size={18} />

			<span class="hidden sm:block"> Add </span>
		</button>
	</div>
</nav>
