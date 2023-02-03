import type { PageServerLoad } from './$types';

export const load = (async ({ fetch, parent }) => {
    await parent();

    return {test: "dsa"};
}) satisfies PageServerLoad;
