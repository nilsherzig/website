<script lang="ts">
	export let title: String;
	export let value: any;
</script>

<div class="flex gap-2">
	<p class=" text-zinc-300">{title}</p>
	<input
		class="mb-2 w-full rounded bg-zinc-700 p-0.5 px-2 shadow focus:outline-none focus:ring-2 focus:ring-emerald-500"
		bind:value
	/>
</div>
