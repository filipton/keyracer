import { internalApiUrl, type NrResult, type User } from '$lib/types';
import type { PageServerLoad } from './$types';

export const load = (async ({ fetch, parent, params }) => {
    await parent();
    let userInfo: User | null = null;
    let userResults: NrResult[] = [];

    await Promise.all([
        fetch(`${internalApiUrl}/auth/${params.id}`)
            .then((res) => res.json())
            .then((x: User) => {
                userInfo = x;
            }),

        fetch(`${internalApiUrl}/results/${params.id}`)
            .then((res) => res.json())
            .then((x: NrResult[]) => {
                x.reverse();
                userResults = x;
            })
    ]);

    return {
        userInfo,
        userResults
    };
}) satisfies PageServerLoad;
