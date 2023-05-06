<script lang="ts">
	import { onMount } from 'svelte';
	import Spinner from '../lib/spinner.svelte';
	import Rsa from '$lib/math/rsa.svelte';

	let text = 'wait';
	// onMount(async () => {
	// 	const res = await fetch('http://localhost:3000/get');
	// 	text = await res.json();
	// });
	// const fetchData = (async () => {
	// 	const response = await fetch('http://localhost:3000/get');
	// 	return await response.json();
	// })();
</script>

<div class="container mx-auto mt-4 text-zinc-200">
	<Rsa />
	<!-- {#await fetchData}
		<Spinner />
	{:then data}
		{data.id}
		{data.username}
	{:catch error}
		<p>Could not connect to backend!</p>
	{/await} -->
</div>

<!-- {#await fetchImage}
	<p>...waiting</p>
{:then data}
	<img src={data.message} alt="Dog" />
{:catch error}
	<p>An error occurred!</p>
{/await} -->

<style lang="postcss">
	:global(html) {
		background-color: theme(colors.zinc.900);
	}
</style>
