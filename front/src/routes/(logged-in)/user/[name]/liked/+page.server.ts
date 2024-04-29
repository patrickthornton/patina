import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
    return {
        name: params.name,
        posts: await fetch(`http://127.0.0.1:8000/user/${params.name}/liked`)
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
