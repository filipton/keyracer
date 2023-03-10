import { internalApiUrl, type User } from "$lib/types";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ fetch, cookies }) => {
    if (cookies.get("token")) {
        let session_token = cookies.get("token");

        return await fetch(`${internalApiUrl}/auth/session`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(session_token)
        })
            .then((res) => res.json())
            .then((x: User) => {
                return {
                    user: x,
                    token: session_token
                };
            })
            .catch(() => {
                console.log("Eror while trying to get session");
            });
    }
}) satisfies LayoutServerLoad;
