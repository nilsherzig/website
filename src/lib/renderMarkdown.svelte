<script lang="ts">
	import { marked } from 'marked';
	export let data: any;

	const html = marked.parse(data.content);
</script>

<p>Backend Status: {data.state}</p>

<div class="prose prose-invert">
	{@html html}
</div>
