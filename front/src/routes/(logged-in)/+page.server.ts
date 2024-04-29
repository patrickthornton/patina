import type { PageServerLoad } from "./$types";
import { now } from "$lib/hue.svelte";

export const load: PageServerLoad = async () => {
    let current_time = now();
    return {
        posts: await fetch(`http://127.0.0.1:8000/gradient/${current_time}`)
            .then((res) => {
                if (!res.ok) {
                    throw new Error("HTTP error " + res.status);
                }
                return res.json();
            })
            .catch((error) => {
                console.error("Error:", error);
            }),
    };
};
