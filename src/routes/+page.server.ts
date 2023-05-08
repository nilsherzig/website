/** @type {import('./$types').PageLoad} */
export async function load({ fetch, params }) {
    const res = await fetch('http://localhost:3000/healthcheck');
    const item = await res.json();
    console.log(item);

    return { item };
}