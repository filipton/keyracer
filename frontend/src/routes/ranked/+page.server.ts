import { apiUrl } from '$lib/types';
import type { PageServerLoad } from './$types';

export const load = (async ({ fetch, parent }) => {
    let parentData = await parent();
    if (!parentData.user) {
        return {
            rankedAvailable: -2,
        };
    }

    return fetch(`${apiUrl}/ranked`, {
        method: 'GET',
        headers: {
            Auth: parentData.token as string
        }
    }).then(async (x) => {
        return {
            rankedAvailable: Number(await x.text())
        };
    });
}) satisfies PageServerLoad;
