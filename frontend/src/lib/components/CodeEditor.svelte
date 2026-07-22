<script lang="ts">
	let data: { code: string } = $props();
</script>

<div
	class="
fixed
inset-0
flex
items-center
justify-center
bg-black/60"
>
	<div
		class="
h-[75vh]
w-4/5
overflow-hidden
border-4
border-black
bg-neutral-950
shadow-[8px_8px_0_#000]"
	>
		<div
			class="
flex
justify-between
border-b-4
border-black
bg-neutral-800
px-4
py-2"
		>
			<div class="flex gap-2">
				<div class="h-3 w-3 rounded-full bg-red-500" />

				<div class="h-3 w-3 rounded-full bg-yellow-500" />

				<div class="h-3 w-3 rounded-full bg-green-500" />
			</div>

			<span class="font-bold"> config.yml </span>

			<button> Close </button>
		</div>

		<pre
			class="
h-full
overflow-auto
p-6
font-mono
text-green-400
">

{data.code}

</pre>
	</div>
</div>
