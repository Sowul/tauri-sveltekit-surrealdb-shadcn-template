<script>
	import { invoke } from '@tauri-apps/api/tauri';

	import { Button } from '$lib/components/ui/button';

	let queryRes = null;
	const sql = 'select * from base';

	async function query() {
		await invoke('query', { sql: sql })
			.then((message) => {
				console.log(message);
				queryRes = message;
			})
			.catch((error) => console.error(error));
	}
</script>

<div>
	<Button on:click={query}>Get data</Button>
	<pre>{JSON.stringify(queryRes, null, 2)}</pre>
</div>
