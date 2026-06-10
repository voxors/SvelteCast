<script lang="ts">
	import { onMount } from 'svelte';

	let message: string = $state('Loading...');

	async function fetchMessage(): Promise<void> {
		const res = await fetch('/api/hello');
		const data = await res.json();
		message = data.message;
	}

	onMount(() => {
		fetchMessage();
	});
</script>

<main>
	<h1>{message}</h1>
</main>
