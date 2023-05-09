/** @type {import('./$types').PageLoad} */
export async function load({ fetch, params }) {
    try {
    const res = await fetch('http://localhost:3000/healthcheck');
    const item = await res.json();
        return { item };
    } catch(error) {
        const item = await JSON.parse('{"state":"offline"}');
        return { item };
    }
}