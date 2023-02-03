import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load = (async ({ parent }) => {
    let parentData = await parent();

    if (parentData.user) {
        throw redirect(302, `/user/${parentData.user.id}`);
    }
}) satisfies PageServerLoad;
