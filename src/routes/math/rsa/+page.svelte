<script lang="ts">
	import { browser } from '$app/environment';
	let inverseTableData = new Array();
	import RsaInput from './rsa_input.svelte';
	import { onMount } from 'svelte';

	let clearText = 'Für die Horde';
	let p = 83;
	let q = 89;
	let N = p * q;
	let phi_n = (p - 1) * (q - 1);
	let e = 65537;
	let d = get_d(phi_n, e);

	let encryptedText: String[] = [];

	let inputClearText = clearText;
	let inputEncryptedText = '';

	// let clearTextChars: String[] = [];

	function printTable(q: any, a: number, b: number, r: any, t1: number, t2: number, t: any) {
		inverseTableData.push([q, a, b, r, t1, t2, t]);
	}

	// this horror will build the html table (and caluclate the multiplicate inverse mod)
	function get_d(a: number, b: number) {
		inverseTableData = [];
		// step 1
		let input1 = a;
		let r = a % b;
		let q = Math.floor(a / b);
		let t1 = 0;
		let t2 = 1;
		let t = t1 - t2 * q;
		printTable(q, a, b, r, t1, t2, t);
		let counter = 0;
		while (true) {
			a = b;
			b = r;
			t1 = t2;
			t2 = t;
			r = a % b;
			q = Math.floor(a / b);
			t = t1 - t2 * q;

			if (a % b >= 0) {
				printTable(q, a, b, r, t1, t2, t);
			} else {
				printTable('x', a, b, 'x', t1, t2, 'x');
				if (t1 < 0) {
					return t1 + input1;
				} else {
					return t1;
				}
			}
		}
	}

	function modularExponentiation(base: number, exponent: number, modulus: number) {
		let result = 1;
		base = base % modulus;

		while (exponent > 0) {
			if (exponent % 2 === 1) {
				result = (result * base) % modulus;
			}
			exponent = Math.floor(exponent / 2);
			base = (base * base) % modulus;
		}
		return result;
	}

	// default state of the website
	encrypt();

	$: p, q, e, updateSite();

	$: inputClearText, encrypt();
	$: inputEncryptedText, decrypt();

	let mountHasRun = false;

	function updateSite() {
		console.log('update');
		p = p;
		q = q;
		N = p * q;
		phi_n = (p - 1) * (q - 1);
		e = e;
		d = get_d(phi_n, e);

		if (mountHasRun) {
			if (browser) {
				const obj = { p: p, q: q, N: N, e: e };
				window.location.hash = JSON.stringify(obj);
			}
		}
	}

	onMount(async () => {
		let jsonString = window.location.hash.replace('#', '');
		jsonString = decodeURI(jsonString);
		try {
			let obj = JSON.parse(jsonString);
			p = obj.p;
			q = obj.q;
			e = obj.e;
		} catch {
			console.log('error parsing #');
		}
		mountHasRun = true;
		updateSite();
	});

	function encrypt() {
		console.log('running enc');
		encryptedText = [];
		// clearTextChars = [];
		for (let char of inputClearText) {
			let ascii_value = char.charCodeAt(0);
			// clearTextChars.push(char + ': ' + ascii_value.toString());
			let result = modularExponentiation(ascii_value, e, N);
			encryptedText.push(result.toString());
		}
		inputEncryptedText = encryptedText.toString();
		// clearTextChars = clearTextChars;
	}

	function decrypt() {
		let enc_array = inputEncryptedText.split(',');

		let dec_array: number[] = new Array();

		enc_array.forEach((part) => {
			let result = modularExponentiation(parseInt(part), d, N);
			dec_array.push(result);
		});

		clearText = String.fromCharCode(...dec_array);
		inputClearText = clearText;
	}
</script>

<div
	class="container mx-auto mt-2 flex flex-wrap justify-between gap-4 rounded bg-zinc-800 p-2 shadow selection:bg-emerald-500 selection:text-zinc-900"
>
	<div class="w-full p-4">
		<p class="mb-2 font-bold text-zinc-400">En / Decrypt</p>

		<RsaInput title="Cleartext" bind:value={inputClearText} />
		<!-- <p>{clearTextChars}</p> -->
		<RsaInput title="Encrypted" bind:value={inputEncryptedText} />
		<!-- <p class="pt-2 text-rose-400">Some special chars (öäü dont get properly encrypted right now)</p> -->
	</div>

	<div class="w-80 p-4">
		<p class="mb-2 font-bold text-zinc-400">Inputs</p>
		<!-- <p class="mb-2 text-sm text-rose-500">
			This whole thing runs without any kind of input validation :)
		</p> -->
		<RsaInput title="p" bind:value={p} />
		<RsaInput title="q" bind:value={q} />
		<RsaInput title="e" bind:value={e} />
		{#if N < 255}
			<p class="text-rose-400">
				Since i'm using Ascii in the background to convert the Cleartext Chars to Numbers, please
				choose p * q >= 255
			</p>
		{/if}
	</div>

	<div class="w-fit p-4">
		<p class="mb-2 font-bold text-zinc-400">computed variables</p>
		<p>N: {N}</p>
		<p>Phi(n): {phi_n}</p>
		<p>d: {d}</p>
	</div>

	<div class="w-fit p-4">
		<p class="mb-2 font-bold text-zinc-400">character debug</p>
		<div class="h-48 overflow-auto">
			{#each inputClearText as char}
				<p>{char}: {char.charCodeAt(0)}</p>
			{/each}
		</div>
	</div>

	<div class="w-fit p-4">
		<p class="mb-2 font-bold text-zinc-400">multiplicative inverse modulo (d)</p>
		<table class="table-auto border-collapse rounded border border-zinc-500 bg-zinc-700 shadow">
			<thead>
				<tr class="bg-zinc-600 text-center">
					<th class="border border-zinc-500 px-2 py-1">q</th>
					<th class="border border-zinc-500 px-2 py-1">phi(N)</th>
					<th class="border border-zinc-500 px-2 py-1">e</th>
					<th class="border border-zinc-500 px-2 py-1">r</th>
					<th class="border border-zinc-500 px-2 py-1">t1</th>
					<th class="border border-zinc-500 px-2 py-1">t2</th>
					<th class="border border-zinc-500 px-2 py-1">t</th>
				</tr>
			</thead>
			<tbody>
				{#each inverseTableData as row, index}
					<tr>
						{#each row as item}
							<td class="border border-zinc-500 px-2 py-1 text-right">
								{item}
							</td>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
